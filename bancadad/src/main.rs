//! `bancadad` — the daemon that holds a declared desktop at its declared shape.
//!
//! Reads `(defbancada …)`, reads the world, closes the difference, and does it
//! again — forever. It plans nothing when there is nothing to close, and it
//! says out loud what it will not touch.
//!
//! ## Modes
//!
//! - `plan`   — one tick, print what would happen, change nothing. The default
//!              posture, so a dry run is not a flag someone can forget.
//! - `once`   — one tick, apply it.
//! - `run`    — tick forever at an interval.
//! - `verify` — parse and validate the declaration only; the thing CI runs.
//!
//! ## What this daemon does NOT do, deliberately
//!
//! It holds no rung of its own. The rung is passed in, and every gap it lacks
//! authority for is reported rather than attempted — so running it as an
//! unprivileged user is a legitimate configuration that reports honestly,
//! rather than a broken one that fails per-action.
//!
//! It also never mints a [`saihai_spec::Warrant`]. Break-glass actions are not
//! reachable from a loop, by construction: the constructor is not linked here.

mod config;

use config::{BancadaConfig, ConfiguredRung};
use shikumi::TieredConfig;

use std::collections::BTreeMap;
use std::path::Path;
use std::process::ExitCode;
use std::time::Duration;

use saihai_forge::reconcile::{apply, plan, Desired, Gap, Observed, Plan};
use saihai_spec::{bancada::Bancada, Catalog, Rung};

const CATALOG: &str = include_str!("../../catalog/desktop.saihai.lisp");

/// Where observed state comes from.
///
/// A trait because the real backend is a compositor that does not exist yet.
/// The loop is proven against a mock, which is the fleet's default delivery
/// method — and it means the daemon's logic is testable with no desktop at all.
trait World {
    /// Read the fields the declaration cares about.
    fn observe(&self, keys: &[String]) -> Observed;
    /// Perform the convergeable calls. Returns how many landed.
    fn apply_calls(&mut self, p: &Plan) -> usize;
    fn describe(&self) -> String;
}

/// An in-memory desktop. The daemon's logic is identical against this and
/// against a real compositor; only the impl differs.
#[derive(Default)]
struct MockWorld {
    state: BTreeMap<String, String>,
}

impl World for MockWorld {
    fn observe(&self, keys: &[String]) -> Observed {
        Observed {
            state: keys
                .iter()
                .filter_map(|k| self.state.get(k).map(|v| (k.clone(), v.clone())))
                .collect(),
        }
    }
    fn apply_calls(&mut self, p: &Plan) -> usize {
        let next = apply(p, &Observed { state: self.state.clone() });
        self.state = next.state;
        p.calls.len()
    }
    fn describe(&self) -> String {
        format!("mock ({} keys)", self.state.len())
    }
}

fn render(p: &Plan) -> String {
    use std::fmt::Write as _;
    let mut s = String::new();
    if p.is_converged() {
        let _ = writeln!(s, "converged — {} keys examined, nothing to do", p.examined);
    } else {
        let _ = writeln!(s, "{} call(s), {} examined:", p.calls.len(), p.examined);
        for c in &p.calls {
            let _ = writeln!(
                s,
                "  {:<26} {} = {:?} (was {:?})",
                c.action.as_str(),
                c.key,
                c.want,
                c.have.as_deref().unwrap_or("<unset>")
            );
        }
    }
    // Gaps print EVERY tick, converged or not. A loop that stays quiet about
    // what it cannot do reports a convergence it has not reached.
    for g in &p.gaps {
        match g {
            Gap::NoAction { key, want } => {
                let _ = writeln!(s, "  GAP  no action sets {key} (wanted {want:?})");
            }
            Gap::Unauthorized {
                key,
                action,
                needs,
                holds,
            } => {
                let _ = writeln!(
                    s,
                    "  GAP  {key} needs {} at {needs:?}, loop holds {holds:?}",
                    action.as_str()
                );
            }
            Gap::Blind { key, action } => {
                let _ = writeln!(s, "  GAP  {key} is set by {} which is blind", action.as_str());
            }
        }
    }
    s
}

fn load(decl: &Path) -> Result<(Catalog, Bancada), String> {
    let cat = Catalog::from_source(CATALOG).map_err(|e| format!("catalog: {e}"))?;
    cat.validate().map_err(|e| format!("catalog: {e}"))?;
    let src = std::fs::read_to_string(decl).map_err(|e| format!("{}: {e}", decl.display()))?;
    let b = Bancada::from_source(&src).map_err(|e| format!("{}: {e}", decl.display()))?;
    b.validate().map_err(|e| format!("{}: {e}", decl.display()))?;
    Ok((cat, b))
}

fn tick(cat: &Catalog, want: &Desired, world: &mut dyn World, rung: Rung, do_apply: bool) -> Plan {
    let keys: Vec<String> = want.state.keys().cloned().collect();
    let observed = world.observe(&keys);
    let p = plan(cat, want, &observed, rung);
    if do_apply && !p.calls.is_empty() {
        world.apply_calls(&p);
    }
    p
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mode = args.first().map(String::as_str).unwrap_or("help");

    // ── Configuration resolves through shikumi's tiers ────────────────────
    // Not ad-hoc argument parsing: the prescribed tier is the fleet desktop,
    // `discovered()` may raise the rung only on real evidence that the process
    // is seated, and `bare()` is a floor that cannot act. Command-line
    // arguments are the LAST overlay, expressed as an `extend` over that base,
    // so a flag never silently reintroduces authority the tiers withheld.
    let base = BancadaConfig::prescribed_default();
    let overlay = BancadaConfig {
        declaration: args.get(1).cloned().unwrap_or_default(),
        interval_secs: 0,
        rung: match args.get(2).map(String::as_str) {
            Some("l0") => ConfiguredRung::L0,
            Some("l2") => ConfiguredRung::L2,
            Some("l3") => ConfiguredRung::L3,
            Some("l1") => ConfiguredRung::L1,
            _ => base.rung,
        },
        apply: base.apply,
    };
    let cfg = overlay.extend(&base);
    let decl = cfg.declaration.clone();
    let rung: Rung = cfg.rung.to_rung();

    let run = || -> Result<ExitCode, String> {
        let (cat, b) = load(Path::new(&decl))?;
        let want = Desired { state: b.lower() };
        let mut world = MockWorld::default();

        match mode {
            "verify" => {
                println!(
                    "ok — {} declares {} desired key(s); catalog has {} actions",
                    b.node.as_str(),
                    want.state.len(),
                    cat.actions.len()
                );
                Ok(ExitCode::SUCCESS)
            }
            "plan" => {
                let p = tick(&cat, &want, &mut world, rung, false);
                print!("{}", render(&p));
                Ok(ExitCode::SUCCESS)
            }
            "once" => {
                let p = tick(&cat, &want, &mut world, rung, true);
                print!("{}", render(&p));
                let after = tick(&cat, &want, &mut world, rung, false);
                if after.is_converged() {
                    println!("converged after one tick");
                    Ok(ExitCode::SUCCESS)
                } else {
                    println!("NOT converged after one tick — {} remain", after.calls.len());
                    Ok(ExitCode::FAILURE)
                }
            }
            "run" => {
                println!(
                    "bancadad: holding {} at its declaration, world = {}, rung = {rung:?}, tick {}s",
                    b.node.as_str(),
                    world.describe(),
                    cfg.interval_secs
                );
                let mut ticks = 0u64;
                loop {
                    let p = tick(&cat, &want, &mut world, rung, true);
                    ticks += 1;
                    // Print a tick only when it DID something or found a gap.
                    // A loop that logs every quiet tick trains its operator to
                    // ignore it, which is how a real one gets missed.
                    if !p.calls.is_empty() || !p.gaps.is_empty() {
                        print!("[tick {ticks}] {}", render(&p));
                    }
                    std::thread::sleep(Duration::from_secs(cfg.interval_secs));
                }
            }
            _ => {
                eprintln!(
                    "bancadad — hold a declared desktop at its declared shape\n\n\
                     USAGE:\n    \
                       bancadad verify [decl]           parse + validate only\n    \
                       bancadad plan   [decl] [rung]    one tick, change nothing\n    \
                       bancadad once   [decl] [rung]    one tick, apply, assert convergence\n    \
                       bancadad run    [decl] [rung]    tick forever\n\n\
                     rung is l0|l1|l2|l3 and defaults to l1 (seat-owner).\n"
                );
                Ok(ExitCode::from(2))
            }
        }
    };

    match run() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("bancadad: {e}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> (Catalog, Desired) {
        let cat = Catalog::from_source(CATALOG).unwrap();
        let b = Bancada::from_source(include_str!("../../bancada/plo.bancada.lisp")).unwrap();
        (cat, Desired { state: b.lower() })
    }

    /// ★ THE LOOP CONVERGES AND STAYS CONVERGED. Not one tick — repeated ticks
    /// against a world that keeps its state must do nothing further, which is
    /// the daemon-level statement of the fixpoint.
    #[test]
    fn the_loop_converges_and_then_stays_quiet() {
        let (cat, want) = setup();
        let mut w = MockWorld::default();

        let first = tick(&cat, &want, &mut w, Rung::L2, true);
        assert!(!first.calls.is_empty(), "expected work on a bare desktop");

        for i in 0..5 {
            let p = tick(&cat, &want, &mut w, Rung::L2, true);
            assert!(p.calls.is_empty(), "tick {i} planned work after convergence: {:?}", p.calls);
            assert!(p.is_converged());
        }
    }

    /// ★ DRIFT IS CLOSED. Something changes the world out from under the
    /// declaration; the next tick puts it back. This is "always attempting to
    /// reach state" as a test rather than a claim.
    #[test]
    fn drift_is_detected_and_closed_on_the_next_tick() {
        let (cat, want) = setup();
        let mut w = MockWorld::default();
        tick(&cat, &want, &mut w, Rung::L2, true);
        assert!(tick(&cat, &want, &mut w, Rung::L2, false).is_converged());

        // Someone changes the theme behind the loop's back.
        w.state.insert("theme.name".into(), "vellum".into());

        let p = tick(&cat, &want, &mut w, Rung::L2, true);
        assert_eq!(p.calls.len(), 1, "expected exactly the drifted key");
        assert_eq!(p.calls[0].key, "theme.name");
        assert_eq!(p.calls[0].have.as_deref(), Some("vellum"));
        assert_eq!(p.calls[0].want, "nord");

        assert!(tick(&cat, &want, &mut w, Rung::L2, false).is_converged());
    }

    /// An under-privileged loop still converges what it CAN, and reports the
    /// rest instead of failing or lying.
    #[test]
    fn an_l0_loop_reports_gaps_rather_than_failing() {
        let (cat, want) = setup();
        let mut w = MockWorld::default();
        let p = tick(&cat, &want, &mut w, Rung::L0, true);
        assert!(!p.gaps.is_empty(), "L0 should lack authority for most of this");
        assert!(
            p.gaps.iter().any(|g| matches!(g, Gap::Unauthorized { .. })),
            "gaps: {:?}",
            p.gaps
        );
        // And it reported them rather than pretending: the render always
        // includes gaps, converged or not.
        assert!(render(&p).contains("GAP"));
    }

    #[test]
    fn the_declaration_lowers_to_something_the_catalog_can_route() {
        let (cat, want) = setup();
        let mut w = MockWorld::default();
        let p = tick(&cat, &want, &mut w, Rung::L2, false);
        // Not every key need be routable — an unroutable one is an honest gap —
        // but the theme key must be, or the pente seam is broken.
        assert!(
            p.calls.iter().any(|c| c.key == "theme.name"),
            "theme.name did not route; calls {:?} gaps {:?}",
            p.calls,
            p.gaps
        );
        assert!(cat.actions.iter().any(|a| a.id.as_str() == "theme-select"));
    }
}
