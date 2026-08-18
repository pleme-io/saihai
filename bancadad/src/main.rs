//! `bancadad` — the daemon that holds a declared desktop at its declared shape.
//!
//! Reads `(defbancada …)`, reads the world, closes the difference, and does it
//! again — forever. It plans nothing when there is nothing to close, and it
//! says out loud what it will not touch.
//!
//! ## It does not own its loop
//!
//! Every tick is [`lava_viggy::ViggyEngine::tick`] walking the fleet's seven
//! beats. This daemon supplies the desktop-specific parts — the state keys, the
//! action that sets one, the authority that needs — and lava supplies
//! observation ordering, severity routing, remediation policy and attestation.
//! See `viggy.rs` for what that adoption actually bought.
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
//! authority for is routed to `HoldingForApproval` rather than attempted — so
//! running it as an unprivileged user is a legitimate configuration that
//! reports honestly, rather than a broken one that fails per-action.
//!
//! It also never mints a [`saihai_spec::Warrant`]. Break-glass actions are not
//! reachable from a loop, by construction: the constructor is not linked here.

mod config;
mod viggy;
mod world;

use config::{BancadaConfig, ConfiguredRung};
use shikumi::TieredConfig;

use std::path::Path;
use std::process::ExitCode;
use std::time::Duration;

use indexmap::IndexMap;
use lava_viggy::{TickPhase, TickReport};
use saihai_forge::reconcile::{Desired, Gap, Plan};
use saihai_spec::{bancada::Bancada, Catalog, Rung};
use viggy::{address, engine, BancadaController};
use world::MockWorld;

const CATALOG: &str = include_str!("../../catalog/desktop.saihai.lisp");

fn render(r: &TickReport, p: Option<&Plan>) -> String {
    use std::fmt::Write as _;
    let mut s = String::new();
    let _ = writeln!(
        s,
        "{} ({} ms)",
        r.final_phase.as_str(),
        r.duration().num_milliseconds()
    );
    let Some(p) = p else { return s };

    if p.calls.is_empty() {
        let _ = writeln!(s, "  {} key(s) examined, nothing to do", p.examined);
    } else {
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

fn load(decl: &Path) -> Result<(Catalog, Bancada, String), String> {
    let cat = Catalog::from_source(CATALOG).map_err(|e| format!("catalog: {e}"))?;
    cat.validate().map_err(|e| format!("catalog: {e}"))?;
    let src = std::fs::read_to_string(decl).map_err(|e| format!("{}: {e}", decl.display()))?;
    let b = Bancada::from_source(&src).map_err(|e| format!("{}: {e}", decl.display()))?;
    b.validate().map_err(|e| format!("{}: {e}", decl.display()))?;
    Ok((cat, b, src))
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
        let (cat, b, src) = load(Path::new(&decl))?;
        let want = Desired { state: b.lower() };
        let node = b.node.as_str().to_string();

        if mode == "verify" {
            println!(
                "ok — {node} declares {} desired key(s); catalog has {} actions",
                want.state.len(),
                cat.actions.len()
            );
            return Ok(ExitCode::SUCCESS);
        }

        // `plan` differs from `once`/`run` by ONE field, and it is the field
        // that decides whether the desktop is touched. Nothing else branches.
        let apply = mode != "plan";
        let e = engine(BancadaController::new(
            cat,
            want,
            rung,
            apply,
            src,
            Box::new(MockWorld::default()),
        ));
        let addr = address(&node);

        match mode {
            "plan" | "once" => {
                let r = e.tick(addr.clone(), IndexMap::new());
                print!("{}", render(&r, e.controller.last_plan().as_ref()));
                if mode == "plan" {
                    return Ok(ExitCode::SUCCESS);
                }
                // A second tick is the convergence assertion: an applier that
                // has to run twice has not converged, it has merely acted.
                let after = e.tick(addr, IndexMap::new());
                if after.final_phase == TickPhase::Stable {
                    println!("converged after one tick");
                    Ok(ExitCode::SUCCESS)
                } else {
                    println!("NOT converged after one tick — {}", after.final_phase.as_str());
                    Ok(ExitCode::FAILURE)
                }
            }
            "run" => {
                println!(
                    "bancadad: holding {node} at its declaration, world = {}, rung = {rung:?}, tick {}s",
                    e.controller.describe_world(),
                    cfg.interval_secs
                );
                let mut ticks = 0u64;
                loop {
                    let r = e.tick(addr.clone(), IndexMap::new());
                    ticks += 1;
                    // Print a tick only when something happened. A loop that
                    // logs every quiet tick trains its operator to ignore it,
                    // which is how a real one gets missed.
                    if r.final_phase != TickPhase::Stable {
                        print!(
                            "[tick {ticks}] {}",
                            render(&r, e.controller.last_plan().as_ref())
                        );
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
