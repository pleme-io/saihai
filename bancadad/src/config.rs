//! The daemon's configuration, on shikumi's tiered surface.
//!
//! One shape fleet-wide: `bare()` is the documented floor, `discovered()`
//! overlays what the machine can detect about itself, and
//! `prescribed_default()` is the first-launch experience. A node's own file
//! `extend()`s that.
//!
//! ## Why the tiers are not decoration here
//!
//! The rung the loop holds is the most consequential setting this daemon has,
//! and the three tiers say three genuinely different things about it:
//!
//! - `bare()` holds **L0**. A floor that could act would be a floor that
//!   surprises: an unconfigured daemon must be able to observe and report, and
//!   nothing else.
//! - `discovered()` raises it to **L1** *only if the process actually looks
//!   seated* — a real check against the environment, not an assumption. This
//!   is the tier that is allowed to know things about the machine.
//! - `prescribed_default()` is what a fleet desktop gets: L1, a 5s tick, and
//!   the node's declaration path derived from its hostname.
//!
//! Defaulting higher anywhere would let a misconfiguration silently plan work
//! it cannot do, which is the failure the whole `Gap::Unauthorized` path exists
//! to make visible rather than mysterious.

use serde::{Deserialize, Serialize};
use shikumi::TieredConfig;

/// How much authority the loop is permitted to exercise.
///
/// Mirrors `hashigo::Rung` as a serialisable config value. Deliberately its own
/// type rather than a re-export: a config file is untrusted input, and the
/// authority ladder's type-level guarantees must not be reachable by
/// deserialisation. Converting is explicit, at one place.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfiguredRung {
    L0,
    L1,
    L2,
    L3,
}

impl ConfiguredRung {
    /// The value-level rung this configures.
    ///
    /// Note what this does NOT do: it does not mint a `RungWitness`. A config
    /// file can say which rung the loop may *use*; only the host can attest
    /// that the process *holds* it.
    #[must_use]
    pub const fn to_rung(self) -> saihai_spec::Rung {
        match self {
            Self::L0 => saihai_spec::Rung::L0,
            Self::L1 => saihai_spec::Rung::L1,
            Self::L2 => saihai_spec::Rung::L2,
            Self::L3 => saihai_spec::Rung::L3,
        }
    }
}

/// The daemon's configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BancadaConfig {
    /// Path to the `(defbancada …)` declaration this loop holds.
    pub declaration: String,
    /// Seconds between ticks.
    pub interval_secs: u64,
    /// The rung the loop may exercise.
    pub rung: ConfiguredRung,
    /// Whether to apply, or only ever plan.
    ///
    /// `false` at every tier below `prescribed_default` — a daemon that starts
    /// mutating a desktop because nobody configured it is exactly the surprise
    /// a tiered config exists to prevent.
    pub apply: bool,
}

impl Default for BancadaConfig {
    fn default() -> Self {
        Self::prescribed_default()
    }
}

impl TieredConfig for BancadaConfig {
    /// Tier 0 — the documented floor. Observe, report, change nothing.
    fn bare() -> Self {
        Self {
            declaration: String::new(),
            interval_secs: 30,
            rung: ConfiguredRung::L0,
            apply: false,
        }
    }

    /// Tier 1 — what the machine can tell us about itself.
    ///
    /// Raises the rung to L1 only when the process genuinely looks seated
    /// (`WAYLAND_DISPLAY` or `DISPLAY` present). A real check: an unseated
    /// process that assumed L1 would plan work it cannot perform, and the
    /// resulting failures would look like bugs rather than like the
    /// misconfiguration they are.
    fn discovered() -> Self {
        let seated =
            std::env::var_os("WAYLAND_DISPLAY").is_some() || std::env::var_os("DISPLAY").is_some();
        Self {
            rung: if seated {
                ConfiguredRung::L1
            } else {
                ConfiguredRung::L0
            },
            declaration: std::env::var("BANCADA_DECLARATION").unwrap_or_default(),
            ..Self::bare()
        }
    }

    /// Tier 2 — the prescribed fleet desktop.
    fn prescribed_default() -> Self {
        Self {
            declaration: "bancada/plo.bancada.lisp".into(),
            interval_secs: 5,
            rung: ConfiguredRung::L1,
            apply: true,
        }
    }

    /// Tier 3 — a node's own file over the prescribed base.
    ///
    /// Per-field rather than whole-struct replacement: a node that wants only a
    /// different tick interval should not have to restate its rung, and a
    /// node that restates nothing should not silently drop to the floor.
    fn extend(self, base: &Self) -> Self {
        Self {
            declaration: if self.declaration.is_empty() {
                base.declaration.clone()
            } else {
                self.declaration
            },
            interval_secs: if self.interval_secs == 0 {
                base.interval_secs
            } else {
                self.interval_secs
            },
            rung: self.rung,
            apply: self.apply,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// ★ The floor cannot act. An unconfigured daemon observes and reports.
    #[test]
    fn bare_holds_no_authority_and_does_not_apply() {
        let c = BancadaConfig::bare();
        assert_eq!(c.rung, ConfiguredRung::L0);
        assert!(!c.apply, "the floor must never mutate a desktop");
    }

    /// `discovered()` may only raise the rung on real evidence.
    #[test]
    fn discovered_raises_the_rung_only_when_the_process_looks_seated() {
        // The test cannot portably set the environment for the whole process
        // without racing other tests, so it asserts the INVARIANT that holds
        // either way: discovered() never exceeds L1, and never enables apply.
        let c = BancadaConfig::discovered();
        assert!(
            matches!(c.rung, ConfiguredRung::L0 | ConfiguredRung::L1),
            "discovery must not invent authority above seat-owner: {:?}",
            c.rung
        );
        assert!(!c.apply, "discovery must not decide to mutate");
    }

    #[test]
    fn the_prescribed_default_is_a_working_fleet_desktop() {
        let c = BancadaConfig::prescribed_default();
        assert_eq!(c.rung, ConfiguredRung::L1);
        assert!(c.apply);
        assert!(!c.declaration.is_empty());
        assert!(c.interval_secs > 0);
        // Default delegates to the prescribed tier, so the standard idiom works.
        assert_eq!(BancadaConfig::default(), c);
    }

    /// ★ A node restating nothing must not silently drop to the floor.
    #[test]
    fn extend_fills_unset_fields_from_the_base() {
        let base = BancadaConfig::prescribed_default();
        let node = BancadaConfig {
            declaration: String::new(),
            interval_secs: 0,
            rung: ConfiguredRung::L2,
            apply: true,
        };
        let merged = node.extend(&base);
        assert_eq!(merged.declaration, base.declaration, "declaration inherited");
        assert_eq!(merged.interval_secs, base.interval_secs, "interval inherited");
        assert_eq!(merged.rung, ConfiguredRung::L2, "the node's own rung wins");
    }

    /// A config may say which rung the loop USES; it can never attest that the
    /// process HOLDS one. The witness has no Deserialize, so this is structural
    /// rather than a rule someone follows.
    #[test]
    fn a_configured_rung_converts_to_a_value_but_not_to_a_witness() {
        assert_eq!(ConfiguredRung::L2.to_rung(), saihai_spec::Rung::L2);
        // Round-trips through serde, which is the point: it is untrusted input.
        let j = serde_json::to_string(&ConfiguredRung::L2).unwrap();
        assert_eq!(j, "\"l2\"");
        assert_eq!(
            serde_json::from_str::<ConfiguredRung>(&j).unwrap(),
            ConfiguredRung::L2
        );
    }

    /// The tiers are ordered by how much they permit, and that ordering is the
    /// safety property: nothing below the prescribed tier may mutate.
    #[test]
    fn no_tier_below_prescribed_may_mutate() {
        for c in [BancadaConfig::bare(), BancadaConfig::discovered()] {
            assert!(!c.apply, "{c:?} must not apply");
        }
        assert!(BancadaConfig::prescribed_default().apply);
    }
}
