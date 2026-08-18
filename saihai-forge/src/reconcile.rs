//! `bancada` — the loop that holds a declared desktop at its declared shape.
//!
//! The workbench: the surface you work at, laid out the way you declared it,
//! **and put back that way**.
//!
//! ## The shape
//!
//! `(desired, observed) -> Vec<Call>`. A reconciler, not a re-applier: the
//! distinction is [`Plan::is_converged`], and a re-applier has no such notion
//! because it re-issues everything every tick and can never say whether it
//! changed anything.
//!
//! ## Three properties the types carry
//!
//! 1. **Only `Converging` actions may close a gap.** [`Class::Blind`] actions
//!    have no read-back path, so planning one and calling the state reached
//!    would be a lie the next tick cannot catch. They are returned separately,
//!    never counted.
//! 2. **Authority is checked before an action is planned, not after.** A gap
//!    the loop lacks the rung to close becomes a [`Gap::Unauthorized`] the
//!    operator can see, rather than a runtime failure buried in a log.
//! 3. **Vacuous convergence is unreachable.** An empty plan that examined
//!    nothing is not converged — a discovery bug must not read as a healthy
//!    desktop.

use std::collections::BTreeMap;

use saihai_spec::{ActionId, Catalog, Class, Rung};

/// One field of desired world-state, addressed the way the catalog addresses
/// what it observes: `windows.focused`, `theme.name`, `outputs.mode`.
pub type StateKey = String;

/// The declared desktop — what the operator said it should look like.
///
/// A flat map on purpose at this stage: the declarative surface
/// (`(defbancada …)`) lowers INTO this, so the reconciler has one shape to
/// reason about no matter how rich the authoring form becomes.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Desired {
    pub state: BTreeMap<StateKey, String>,
}

/// What the world actually reports. Same address space, so a gap is a
/// key-by-key comparison rather than a bespoke diff per domain.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Observed {
    pub state: BTreeMap<StateKey, String>,
}

/// A call the loop intends to make.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
    pub action: ActionId,
    pub key: StateKey,
    pub want: String,
    pub have: Option<String>,
}

/// A gap the loop found but will not act on, and why.
///
/// Surfacing these is the point. A loop that silently skips what it cannot do
/// reports convergence it has not reached.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Gap {
    /// No catalog action closes this gap. The desired state names something
    /// the desktop has no verb for.
    NoAction { key: StateKey, want: String },
    /// An action exists but requires a higher rung than the loop holds.
    Unauthorized {
        key: StateKey,
        action: ActionId,
        needs: Rung,
        holds: Rung,
    },
    /// An action exists but is blind — firing it can close the gap, but the
    /// loop can never confirm it, so it must not be part of a fixpoint.
    Blind { key: StateKey, action: ActionId },
}

/// The result of one tick.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plan {
    /// Calls whose effect is observable — the convergeable work.
    pub calls: Vec<Call>,
    /// Everything found and deliberately not planned, with the reason.
    pub gaps: Vec<Gap>,
    /// How many desired keys were examined. Carried so an empty plan that
    /// looked at nothing is distinguishable from a converged desktop.
    pub examined: usize,
}

impl Plan {
    /// Converged: nothing convergeable to do, and something was examined.
    ///
    /// Gaps do NOT block convergence — a blind action or a missing rung is a
    /// standing fact about the world, not work in progress. They are reported
    /// every tick precisely so "converged" never quietly means "gave up".
    #[must_use]
    pub fn is_converged(&self) -> bool {
        self.calls.is_empty() && self.examined > 0
    }
}

/// Maps a state key to the action that sets it.
///
/// Built from the catalog's own `observed` rows: an action that observes
/// `windows.focused` is the action that can set it. So the routing table is
/// derived from the same data that decides observability — one source, and a
/// new action becomes routable by being authored, not by editing a second map.
#[must_use]
pub fn routes(cat: &Catalog) -> BTreeMap<StateKey, &saihai_spec::ActionSpec> {
    let mut m = BTreeMap::new();
    for a in &cat.actions {
        if a.class() != Class::Converging {
            continue;
        }
        for o in &a.observed {
            let key = format!("{:?}.{}", o.domain, o.field.as_str()).to_lowercase();
            // First writer wins, and the catalog is ordered, so this is stable.
            m.entry(key).or_insert(a);
        }
    }
    m
}

/// Compute the tick.
///
/// Pure: no I/O, so a plan can be shown before anything is touched, and a test
/// needs no desktop. That is also what makes the loop testable at all —
/// `(desired, observed)` in, `Plan` out.
#[must_use]
pub fn plan(cat: &Catalog, desired: &Desired, observed: &Observed, holds: Rung) -> Plan {
    let table = routes(cat);
    let mut calls = Vec::new();
    let mut gaps = Vec::new();
    let mut examined = 0usize;

    for (key, want) in &desired.state {
        examined += 1;
        let have = observed.state.get(key);
        if have == Some(want) {
            continue;
        }
        let Some(spec) = table.get(key) else {
            gaps.push(Gap::NoAction {
                key: key.clone(),
                want: want.clone(),
            });
            continue;
        };
        // Authority BEFORE planning: a gap the loop cannot close is reported,
        // not attempted and failed.
        if !holds.permits(spec.rung()) {
            gaps.push(Gap::Unauthorized {
                key: key.clone(),
                action: spec.id.clone(),
                needs: spec.rung(),
                holds,
            });
            continue;
        }
        calls.push(Call {
            action: spec.id.clone(),
            key: key.clone(),
            want: want.clone(),
            have: have.cloned(),
        });
    }

    Plan {
        calls,
        gaps,
        examined,
    }
}

/// Apply a plan to an observed state, as a pure function.
///
/// Models what a backend would do: each call sets its key. Real backends are
/// injected behind this at the daemon layer; keeping the state transition pure
/// is what lets the fixpoint be a test rather than a hope.
#[must_use]
pub fn apply(plan: &Plan, observed: &Observed) -> Observed {
    let mut next = observed.clone();
    for c in &plan.calls {
        next.state.insert(c.key.clone(), c.want.clone());
    }
    next
}

#[cfg(test)]
mod tests {
    use super::*;

    const CATALOG: &str = include_str!("../../catalog/desktop.saihai.lisp");

    fn cat() -> Catalog {
        Catalog::from_source(CATALOG).unwrap()
    }

    fn desired(pairs: &[(&str, &str)]) -> Desired {
        Desired {
            state: pairs
                .iter()
                .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
                .collect(),
        }
    }

    /// The routing table is derived from the catalog's own observed rows, so a
    /// new convergeable action is routable by being authored.
    #[test]
    fn routes_are_derived_from_the_catalog() {
        let c = cat();
        let r = routes(&c);
        assert!(!r.is_empty());
        assert!(r.contains_key("theme.name"), "keys: {:?}", r.keys().collect::<Vec<_>>());
        // Only convergeable actions route — a blind action must never be
        // reachable as a way to close a gap.
        for spec in r.values() {
            assert_eq!(spec.class(), Class::Converging);
        }
    }

    /// ★ THE FIXPOINT. Apply a plan, replan, and the loop has nothing left.
    #[test]
    fn apply_then_replan_converges() {
        let c = cat();
        let d = desired(&[("theme.name", "nord")]);
        let o = Observed::default();

        let first = plan(&c, &d, &o, Rung::L2);
        assert!(!first.calls.is_empty(), "expected work, got {first:?}");
        assert!(!first.is_converged());

        let o2 = apply(&first, &o);
        let second = plan(&c, &d, &o2, Rung::L2);
        assert!(
            second.is_converged(),
            "expected convergence, still planned {:?}",
            second.calls
        );
    }

    /// A desktop already at its declared shape plans nothing.
    #[test]
    fn an_already_correct_desktop_plans_nothing() {
        let c = cat();
        let d = desired(&[("theme.name", "nord")]);
        let o = Observed {
            state: d.state.clone(),
        };
        let p = plan(&c, &d, &o, Rung::L2);
        assert!(p.calls.is_empty());
        assert!(p.is_converged());
        assert_eq!(p.examined, 1);
    }

    /// ★ Vacuous convergence is unreachable.
    #[test]
    fn an_empty_plan_that_examined_nothing_is_not_converged() {
        let vacuous = Plan {
            calls: vec![],
            gaps: vec![],
            examined: 0,
        };
        assert!(!vacuous.is_converged());
    }

    /// ★ Authority is checked BEFORE planning. A loop holding L1 cannot plan an
    /// L2 action; it reports the gap instead of attempting and failing.
    #[test]
    fn an_underprivileged_loop_reports_rather_than_attempts() {
        let c = cat();
        // outputs.mode is set by output-set-mode, which needs L2.
        let d = desired(&[("outputs.mode", "2560x1440")]);
        let o = Observed::default();

        let low = plan(&c, &d, &o, Rung::L1);
        assert!(low.calls.is_empty(), "L1 must not plan an L2 action");
        assert!(matches!(
            low.gaps.first(),
            Some(Gap::Unauthorized {
                needs: Rung::L2,
                holds: Rung::L1,
                ..
            })
        ));

        let high = plan(&c, &d, &o, Rung::L2);
        assert_eq!(high.calls.len(), 1, "L2 may plan it");
        assert!(high.gaps.is_empty());
    }

    /// A desired key no action can set is reported, never silently dropped.
    #[test]
    fn an_unroutable_key_is_reported_as_a_gap() {
        let c = cat();
        let d = desired(&[("nonsense.field", "x")]);
        let p = plan(&c, &d, &Observed::default(), Rung::L3);
        assert!(p.calls.is_empty());
        assert!(matches!(p.gaps.first(), Some(Gap::NoAction { .. })));
        // Examined, so this is NOT vacuous — it looked and found no route.
        assert_eq!(p.examined, 1);
        assert!(p.is_converged(), "a standing gap does not block convergence");
    }

    /// ★ A blind action can never be routed, so it can never be counted toward
    /// a fixpoint. `window-raise` observes nothing and must not appear.
    #[test]
    fn blind_actions_are_not_routable() {
        let c = cat();
        let blind: Vec<_> = c.by_class(Class::Blind).iter().map(|a| a.id.clone()).collect();
        assert!(!blind.is_empty(), "the catalog must exercise the blind case");
        let routed: Vec<_> = routes(&c).values().map(|a| a.id.clone()).collect();
        for b in blind {
            assert!(!routed.contains(&b), "{} is blind but routable", b.as_str());
        }
    }

    /// The tick is bounded by the desired state, not by the world: a desktop
    /// with ten thousand windows does not make a bigger plan.
    #[test]
    fn the_plan_is_bounded_by_the_declaration() {
        let c = cat();
        let d = desired(&[("theme.name", "nord"), ("focus.workspace", "3")]);
        let mut o = Observed::default();
        for i in 0..1000 {
            o.state.insert(format!("windows.w{i}"), "x".into());
        }
        let p = plan(&c, &d, &o, Rung::L2);
        assert_eq!(p.examined, 2);
        assert!(p.calls.len() + p.gaps.len() <= 2);
    }
}
