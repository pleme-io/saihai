//! The login surface, wired into the desktop reconciler.
//!
//! ## ★ THIS FILE IS NOT IN THE BUILD, AND HERE IS EXACTLY WHY
//!
//! It is complete and it was **compile-verified and tested on 2026-08-18** —
//! `cargo test -p bancadad --features login`, 21 passed including this file's
//! 4. Then it was taken back out, because it cannot be committed in a
//! buildable form yet and leaving it wired would break this repo for everyone.
//!
//! **The finding, which cost an attempt to learn:** an `optional = true`
//! dependency still has to RESOLVE. Cargo builds the dependency graph before
//! it applies features, so `mukae-spec = { version = "0.1", optional = true }`
//! fails with *"no matching package named `mukae-spec` found"* even with the
//! feature off. An optional dep on a crate that is not in the index is not a
//! safe placeholder — it is a broken manifest for every consumer.
//!
//! Cargo's `paths` override does not rescue it either: `paths` SUBSTITUTES the
//! source of a dependency that already resolves; it cannot invent a package
//! the index has never heard of. The verification above therefore used a
//! temporary `path =` in the manifest, reverted immediately after — so what is
//! recorded is "this compiled once, on this date, against mukae at that
//! commit", not "this is in CI".
//!
//! ## To activate it — two lines, once mukae is on the registry
//!
//! ```toml
//! # bancadad/Cargo.toml
//! mukae-spec = "0.1"
//! ```
//! ```rust
//! // bancadad/src/main.rs
//! mod login;
//! ```
//!
//! Delete the `#![cfg(...)]` below at the same time. Until then this file is
//! reviewed, tested-once code with a known expiry: **if mukae's
//! `surface::Key` changes, nothing here will notice**, because nothing
//! compiles it. Re-run the verification before trusting it.
//!
//! ## What it does, and the one thing it does NOT become
//!
//! mukae's three login keys join the same `World` bancadad already converges
//! on lava's seven beats — no second loop, no second policy, no second notion
//! of converged. But a `World` is only as real as what backs it, and with
//! mukae at M0 that is `MockSeatEnv`. Turning this on today yields a
//! reconciler correctly driving a mock. Real; not a live desktop.
//!
//! ## What the login domain forces on the reconciler
//!
//! Most desktop state is idempotent; a login is not. mukae encodes that as
//! `Drivable`, and this module's whole job is to respect it:
//!
//! - **A loop may CLOSE a session and may never OPEN one.** Opening needs an
//!   `AuthProof`, which only a human interaction produces — the capability
//!   chain makes a machine-minted one unconstructable. A desired
//!   `session.open = true` on an empty seat produces NO call, and the plan
//!   says why rather than failing.
//! - **Waiting for a human is not drift to fix.** A seat declaring someone
//!   should be logged in is waiting, not broken.

#![cfg(feature = "login")]

use crate::world::World;
use mukae_spec::surface::{self, Desired as LoginDesired, Observed as LoginObserved, Verdict};
use saihai_forge::reconcile::{Observed, Plan};

/// The login keys this world owns, as saihai action-catalog paths.
///
/// Deliberately the same strings `mukae_spec::surface::Key::path()` produces —
/// asserted below rather than assumed, because two hand-maintained copies of a
/// key name is exactly the drift the coverage table exists to catch.
pub const LOGIN_KEYS: &[&str] = &["login.session.open", "login.session.owner", "login.enabled"];

/// A `World` over mukae's login surface.
///
/// Holds the desired login state so the gap can be computed the way mukae
/// models it — by `Drivable`, not by string equality — and then reported to
/// bancadad in bancadad's own vocabulary.
pub struct LoginWorld {
    want: LoginDesired,
    have: LoginObserved,
}

impl LoginWorld {
    #[must_use]
    pub fn new(want: LoginDesired) -> Self {
        Self {
            want,
            have: LoginObserved::default(),
        }
    }

    /// The keys this world answers for. A key outside this set belongs to
    /// another domain and must be left alone rather than reported unknown.
    #[must_use]
    pub fn owns(key: &str) -> bool {
        LOGIN_KEYS.contains(&key)
    }

    /// mukae's own verdict on the current gap, in mukae's vocabulary.
    ///
    /// Exposed because a caller that wants to know WHY a login key is not
    /// converging needs the `NeedsHuman` reason, and flattening it into a
    /// string for bancadad would lose it.
    #[must_use]
    pub fn gap(&self) -> surface::Gap {
        surface::diff(&self.want, &self.have)
    }
}

impl World for LoginWorld {
    fn observe(&self, keys: &[String]) -> Observed {
        let mut state = std::collections::BTreeMap::new();
        for k in keys {
            let v = match k.as_str() {
                "login.session.open" => self.have.session_open.map(|b| b.to_string()),
                "login.session.owner" => self.have.session_owner.map(|u| u.to_string()),
                "login.enabled" => self.have.logins_enabled.map(|b| b.to_string()),
                // Not ours. Contributing nothing is correct — a value invented
                // here would outrank the domain that actually owns the key.
                _ => None,
            };
            if let Some(v) = v {
                state.insert(k.clone(), v);
            }
        }
        Observed { state }
    }

    fn apply_calls(&mut self, p: &Plan) -> usize {
        let mut landed = 0usize;
        for c in &p.calls {
            if !Self::owns(&c.key) {
                continue;
            }
            // ★ THE ASYMMETRY IS ENFORCED HERE, not just described. mukae says
            // a session is CloseOnly, so an open request produces no write —
            // and because `diff` already refuses to plan one, reaching this
            // branch at all would mean the plan disagreed with the surface.
            match c.key.as_str() {
                "login.session.open" if c.want == "false" => {
                    self.have.session_open = Some(false);
                    landed += 1;
                }
                "login.enabled" => {
                    self.have.logins_enabled = Some(c.want == "true");
                    landed += 1;
                }
                // Everything else needs a human. Silently skipping is right:
                // the gap still reports it, and a loop that "failed" here
                // every tick would be crying wolf on a correct seat.
                _ => {}
            }
        }
        landed
    }

    fn describe(&self) -> String {
        let g = self.gap();
        let waiting = g
            .drifts
            .iter()
            .filter(|d| matches!(d.verdict, Verdict::NeedsHuman { .. }))
            .count();
        format!(
            "mukae login surface ({} key(s) examined, {waiting} waiting on a human)",
            g.examined
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mukae_spec::ids::Uid;

    /// ★ THE TWO KEY LISTS AGREE. bancadad names login keys as strings and
    /// mukae derives them from a typed enum; two hand-maintained copies of a
    /// name is exactly the drift that goes silent, so it is asserted.
    #[test]
    fn the_key_names_match_mukaes_typed_paths() {
        let typed: Vec<&str> = surface::Key::ALL.iter().map(|k| k.path()).collect();
        assert_eq!(typed, LOGIN_KEYS);
    }

    /// ★ A LOOP CLOSES A SESSION AND NEVER OPENS ONE — end to end, through
    /// bancadad's own `World`, not just in mukae's types.
    #[test]
    fn the_reconciler_can_close_a_session_but_never_open_one() {
        let mut w = LoginWorld::new(LoginDesired {
            session_open: Some(false),
            ..Default::default()
        });
        w.have.session_open = Some(true);
        assert_eq!(w.gap().actionable().len(), 1, "closing is actionable");

        let mut w = LoginWorld::new(LoginDesired {
            session_open: Some(true),
            ..Default::default()
        });
        w.have.session_open = Some(false);
        assert!(
            w.gap().actionable().is_empty(),
            "opening must never be planned"
        );
        assert!(
            w.describe().contains("waiting on a human"),
            "and the world must SAY so: {}",
            w.describe()
        );
    }

    /// A key from another domain gets no value invented for it — anything this
    /// world contributed would outrank the domain that actually owns it.
    #[test]
    fn a_foreign_key_is_left_entirely_alone() {
        let w = LoginWorld::new(LoginDesired::default());
        let o = w.observe(&["theme.name".to_string()]);
        assert!(o.state.is_empty());
        assert!(!LoginWorld::owns("theme.name"));
    }

    #[test]
    fn an_owner_is_observed_but_never_driven() {
        let mut w = LoginWorld::new(LoginDesired {
            session_owner: Some(Uid(1000)),
            ..Default::default()
        });
        assert!(w.gap().actionable().is_empty(), "owner is ObserveOnly");
        w.have.session_owner = Some(Uid(1000));
        let o = w.observe(&["login.session.owner".to_string()]);
        assert_eq!(
            o.state.get("login.session.owner").map(String::as_str),
            Some("1000")
        );
    }
}
