//! bancada on lava's convergence engine.
//!
//! ## Why this exists rather than the hand-rolled loop
//!
//! `lava-viggy` is a published crate carrying the fleet's seven-beat
//! convergence engine — `Observe → Diff → Classify → Decide → Act → Attest →
//! Tick` — with anomaly routing, a remediation policy and an outcome chain
//! behind it. bancada had independently grown a loop with the same first three
//! beats and a hand-written fourth.
//!
//! Two convergence engines in one fleet is the duplication this substrate
//! exists to remove, and the rule is extend-the-near-miss: bancada implements
//! [`PromessaController`] and the desktop becomes one more thing lava converges,
//! rather than a second thing that converges its own way. What bancada keeps is
//! the genuinely desktop-specific part — what the state keys are, which action
//! sets one, and what authority that needs.
//!
//! ## What adopting lava actually BOUGHT (not just re-plumbing)
//!
//! The hand-rolled loop had one behaviour for every gap: log it, tick again,
//! forever. lava's default [`RemediationPolicy`] gives the desktop three, and
//! they turn out to be exactly the three the desktop needs:
//!
//! | severity | lava's default action | what it means for a desktop |
//! |---|---|---|
//! | Cosmetic | `Alert` | a blind key — say so, acting is meaningless |
//! | Functional | `AutoCorrect` | ordinary drift — close it |
//! | Critical | `RequireApproval` | the loop lacks the rung; a human must grant it |
//!
//! That last row is new. Under the old loop an unauthorized key produced an
//! identical line every tick and the daemon reported itself healthy; now the
//! tick's phase is `HoldingForApproval`, which is a different state from
//! `Stable` and cannot be mistaken for one.
//!
//! `RequireApproval` does NOT mean the tick does nothing — see `act`. The
//! approval is required for the part of the declaration the loop cannot
//! honour; the part it can, it closes.
//!
//! ## The severity mapping is the load-bearing choice
//!
//! Severity is not "how many keys drifted". It is what the drift MEANS:
//!
//! - a convergeable gap is **Functional** — the loop will close it next tick
//! - an **unauthorized** gap is **Critical** — the declaration cannot be
//!   honoured at all, and no amount of ticking will change that
//! - a **blind** gap is **Cosmetic** — a standing fact about the world, not a
//!   fault; the desktop simply cannot be held at that value
//!
//! Mapping unauthorized to Critical is what makes the policy table above work.
//! A loop quietly failing to converge because it lacks a rung looks identical,
//! tick after tick, to a loop that is working — unless the severity says
//! otherwise.

use chrono::Utc;
use indexmap::IndexMap;
use lava_anomaly::{PolicyRouter, RemediationAction, RemediationPolicy, RoutingDecision};
use lava_drift::{DriftReport, DriftedField, Severity};
use lava_outcome_chain::{ContentHash, ResourceAddress};
use lava_viggy::{PromessaController, TickPhase, TickReport, ViggyEngine, ViggyError};
use saihai_forge::reconcile::{plan, Desired, Gap, Plan};
use saihai_spec::{Catalog, Rung};
use std::cell::RefCell;

use crate::world::World;

/// What the controller threads between beats.
pub struct TickContext {
    pub plan: Plan,
}

/// bancada as a lava controller.
///
/// Owns its world: the engine drives every beat through `&self`, so the world
/// sits behind a `RefCell` rather than being threaded down a call stack. That
/// is not a workaround — it is the honest shape. The controller IS the thing
/// that knows how to read and write this desktop; nobody else should hold it.
pub struct BancadaController {
    pub catalog: Catalog,
    pub desired: Desired,
    pub rung: Rung,
    /// Whether `act` may actually touch the desktop.
    ///
    /// A false value does not silently no-op: the tick reports
    /// `HoldingForApproval`, the same phase a Critical gap produces, because
    /// both mean "work exists and is not being done".
    pub apply: bool,
    /// Source text whose hash identifies this declaration in the outcome chain.
    pub spec_source: String,
    world: RefCell<Box<dyn World>>,
    /// The last plan computed, kept so the daemon can render what happened.
    last_plan: RefCell<Option<Plan>>,
    /// What was attested, in order. In-memory today — a real `OutcomeChain`
    /// sink is the next rung and is NOT claimed here.
    ledger: RefCell<Vec<String>>,
}

impl BancadaController {
    #[must_use]
    pub fn new(
        catalog: Catalog,
        desired: Desired,
        rung: Rung,
        apply: bool,
        spec_source: String,
        world: Box<dyn World>,
    ) -> Self {
        Self {
            catalog,
            desired,
            rung,
            apply,
            spec_source,
            world: RefCell::new(world),
            last_plan: RefCell::new(None),
            ledger: RefCell::new(Vec::new()),
        }
    }

    /// The plan from the most recent tick, for rendering.
    #[must_use]
    pub fn last_plan(&self) -> Option<Plan> {
        self.last_plan.borrow().clone()
    }

    /// What has been attested so far.
    #[must_use]
    pub fn ledger(&self) -> Vec<String> {
        self.ledger.borrow().clone()
    }

    #[must_use]
    pub fn describe_world(&self) -> String {
        self.world.borrow().describe()
    }

    /// The severity of a plan, by what its gaps MEAN.
    #[must_use]
    pub fn severity_of(plan: &Plan) -> Option<Severity> {
        if plan.gaps.iter().any(|g| matches!(g, Gap::Unauthorized { .. })) {
            // The declaration cannot be honoured. Ticking will not help, and a
            // loop reporting this as ordinary drift would look healthy forever
            // while never converging.
            return Some(Severity::Critical);
        }
        if !plan.calls.is_empty() {
            return Some(Severity::Functional);
        }
        if plan.gaps.iter().any(|g| matches!(g, Gap::Blind { .. })) {
            // A standing fact about the world, not a fault: the desktop cannot
            // be HELD at this value because it cannot be read back.
            return Some(Severity::Cosmetic);
        }
        // Genuinely converged.
        None
    }
}

impl PromessaController for BancadaController {
    type Context = TickContext;

    fn observe(
        &self,
        _source: &ResourceAddress,
        _bindings: &IndexMap<String, String>,
    ) -> Result<Self::Context, ViggyError> {
        let keys: Vec<String> = self.desired.state.keys().cloned().collect();
        let observed = self.world.borrow().observe(&keys);
        let p = plan(&self.catalog, &self.desired, &observed, self.rung);
        *self.last_plan.borrow_mut() = Some(p.clone());
        Ok(TickContext { plan: p })
    }

    fn diff(
        &self,
        ctx: &Self::Context,
        _bindings: &IndexMap<String, String>,
    ) -> Result<DriftReport, ViggyError> {
        let mut fields: Vec<DriftedField> = Vec::new();

        for c in &ctx.plan.calls {
            let mut f = DriftedField::new(c.action.as_str(), &c.key, Severity::Functional);
            f.observed.clone_from(&c.have);
            f.declared = Some(c.want.clone());
            fields.push(f);
        }
        // Gaps are drift too, and the severity is what distinguishes them. A
        // report that omitted them would show a converged desktop while the
        // declaration went unhonoured.
        for g in &ctx.plan.gaps {
            let (address, key, sev) = match g {
                Gap::Unauthorized { key, action, .. } => {
                    (action.as_str(), key, Severity::Critical)
                }
                Gap::Blind { key, action } => (action.as_str(), key, Severity::Cosmetic),
                // Nothing in the catalog sets this key at all — a declaration
                // that can never be honoured, which is as critical as lacking
                // the authority to honour it.
                Gap::NoAction { key, .. } => ("<none>", key, Severity::Critical),
            };
            let mut f = DriftedField::new(address, key, sev);
            f.declared = self.desired.state.get(key).cloned();
            fields.push(f);
        }

        let max_severity = Self::severity_of(&ctx.plan);
        Ok(DriftReport {
            spec_hash: ContentHash::of(self.spec_source.as_bytes()),
            scanned_at: Utc::now(),
            drifted_fields: fields,
            max_severity,
        })
    }

    fn classify(&self, report: &DriftReport) -> Severity {
        // lava's default is `max_severity.unwrap_or(Cosmetic)`, which is right,
        // but stated explicitly here so the mapping above is the single place
        // severity is decided for a desktop.
        report.max_severity.unwrap_or(Severity::Cosmetic)
    }

    fn act(
        &self,
        ctx: &Self::Context,
        decision: &RoutingDecision,
        _report: &DriftReport,
    ) -> Result<(TickPhase, Option<String>), ViggyError> {
        // Whether to touch the desktop and what phase to report are TWO
        // questions, and collapsing them is a bug this loop already made once.
        //
        // lava's action answers the second. The first is answered by the plan,
        // which by construction holds only calls this loop IS authorized to
        // make — the unauthorized ones are gaps, not calls. So a declaration
        // that is partly unhonourable still gets its honourable part closed;
        // refusing to set the theme because the monitor scale needs a rung we
        // lack would hold authorized work hostage to unauthorized work, which
        // is the same shape the fleet rejects for a PR holding an environment.
        let (may_touch, held) = match decision.action {
            RemediationAction::AutoCorrect => (true, None),
            RemediationAction::RequireApproval => (
                true,
                Some("part of this declaration needs a rung the loop does not hold"),
            ),
            // A blind key routes here: alerting is the whole available action,
            // because the desktop cannot be read back to know if it took.
            RemediationAction::NoOp | RemediationAction::Alert => (false, None),
            RemediationAction::Escalate => (false, Some("escalated")),
        };

        if !may_touch {
            return Ok((
                if held.is_some() {
                    TickPhase::Escalated
                } else {
                    TickPhase::Stable
                },
                held.map(str::to_string),
            ));
        }

        if !self.apply {
            // NOT Stable. Work exists and is not being done, which is the same
            // fact `HoldingForApproval` records for an unauthorized gap —
            // reusing it keeps plan-mode from reading as healthy.
            let n = ctx.plan.calls.len();
            return Ok((
                TickPhase::HoldingForApproval,
                Some(format!("plan-only: {n} call(s) withheld")),
            ));
        }

        let n = self.world.borrow_mut().apply_calls(&ctx.plan);
        Ok(match held {
            // Converged what it could, and says plainly that it could not
            // converge everything. Both halves are true at once.
            Some(why) => (
                TickPhase::HoldingForApproval,
                Some(format!("{n} call(s) applied; {why}")),
            ),
            None => (TickPhase::Reconverging, Some(format!("{n} call(s) applied"))),
        })
    }

    fn attest(&self, report: &TickReport) -> Result<(), ViggyError> {
        // In-memory today. A real `OutcomeChain` sink is the next rung and is
        // deliberately not claimed: this records that the beats ran, not that
        // anyone else can verify they did.
        self.ledger.borrow_mut().push(format!(
            "{} {} beats={}",
            report.started_at.to_rfc3339(),
            report.final_phase.as_str(),
            report.beats.len()
        ));
        Ok(())
    }
}

/// The desktop's engine: bancada's controller, lava's policy router, lava's
/// default remediation policy.
///
/// The policy is deliberately NOT customised. Its defaults already say the
/// right thing for a desktop (see this module's header), and a fleet that
/// re-decides remediation per consumer has as many policies as consumers.
#[must_use]
pub fn engine(controller: BancadaController) -> ViggyEngine<BancadaController, PolicyRouter> {
    ViggyEngine::new(controller, PolicyRouter, RemediationPolicy::default())
}

/// Where a desktop lives, in lava's address vocabulary.
#[must_use]
pub fn address(node: &str) -> ResourceAddress {
    ResourceAddress::new("local", "desktop", node)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::MockWorld;
    use lava_viggy::Beat;
    use saihai_forge::reconcile::Observed;
    use saihai_spec::bancada::Bancada;

    const CATALOG: &str = include_str!("../../catalog/desktop.saihai.lisp");
    const DECL: &str = include_str!("../../bancada/plo.bancada.lisp");

    fn controller(rung: Rung, apply: bool) -> BancadaController {
        let cat = Catalog::from_source(CATALOG).unwrap();
        let b = Bancada::from_source(DECL).unwrap();
        BancadaController::new(
            cat,
            Desired { state: b.lower() },
            rung,
            apply,
            DECL.to_string(),
            Box::new(MockWorld::default()),
        )
    }

    /// ★ ONE TICK WALKS ALL SEVEN BEATS. This is the statement that bancada
    /// runs on lava's engine rather than merely implementing its trait.
    #[test]
    fn a_tick_walks_every_beat_in_canonical_order() {
        let e = engine(controller(Rung::L2, true));
        let r = e.tick(address("plo"), IndexMap::new());
        assert!(r.visited_every_beat(), "beats: {:?}", r.beats);
        assert_eq!(
            r.beats.iter().map(|b| b.beat).collect::<Vec<_>>(),
            Beat::canonical_order().to_vec(),
            "beats must run in lava's canonical order"
        );
        assert_eq!(r.final_phase, TickPhase::Reconverging);
    }

    /// ★ THE ENGINE CONVERGES AND THEN GOES STABLE. The fixpoint, expressed
    /// through lava rather than through bancada's own loop.
    #[test]
    fn the_engine_converges_then_reports_stable() {
        let e = engine(controller(Rung::L2, true));
        let first = e.tick(address("plo"), IndexMap::new());
        assert_eq!(first.final_phase, TickPhase::Reconverging);

        for i in 0..5 {
            let r = e.tick(address("plo"), IndexMap::new());
            // plo declares one blind key, so the steady state is Cosmetic →
            // Alert → Stable, not a clean report.
            assert_eq!(r.final_phase, TickPhase::Stable, "tick {i}");
            let p = e.controller.last_plan().unwrap();
            assert!(p.calls.is_empty(), "tick {i} planned work: {:?}", p.calls);
        }
    }

    /// ★ AN UNAUTHORIZED DECLARATION HOLDS FOR APPROVAL — it does not report
    /// Stable, and it does not report Reconverging either. This behaviour did
    /// not exist before adopting lava.
    #[test]
    fn an_underprivileged_loop_holds_for_approval() {
        let e = engine(controller(Rung::L0, true));
        let r = e.tick(address("plo"), IndexMap::new());
        assert_eq!(r.final_phase, TickPhase::HoldingForApproval);
    }

    /// Plan-only mode is held, not healthy: the same phase, for the same
    /// reason — work exists and is not being done.
    #[test]
    fn plan_only_mode_holds_rather_than_reporting_stable() {
        let e = engine(controller(Rung::L2, false));
        let r = e.tick(address("plo"), IndexMap::new());
        assert_eq!(r.final_phase, TickPhase::HoldingForApproval);
        // And it changed nothing, so the next tick plans the same work.
        let again = e.tick(address("plo"), IndexMap::new());
        assert_eq!(again.final_phase, TickPhase::HoldingForApproval);
    }

    /// ★ A PARTLY-UNHONOURABLE DECLARATION STILL CONVERGES ITS HONOURABLE
    /// PART. plo at L1 can set 12 of its 16 keys and lacks the rung for two
    /// outputs; refusing all 12 because of those two would hold authorized
    /// work hostage to unauthorized work.
    #[test]
    fn partial_authority_converges_what_it_may_and_holds_the_rest() {
        let e = engine(controller(Rung::L1, true));
        let r = e.tick(address("plo"), IndexMap::new());
        assert_eq!(r.final_phase, TickPhase::HoldingForApproval);

        let keys: Vec<String> = e.controller.desired.state.keys().cloned().collect();
        let seen = e.controller.world.borrow().observe(&keys);
        assert_eq!(
            seen.state.get("theme.name").map(String::as_str),
            Some("nord"),
            "the authorized part must be closed"
        );
        assert!(
            !seen.state.contains_key("outputs.scale.DP-1"),
            "the unauthorized part must NOT be"
        );

        // And it stays held rather than flapping: the next tick has no calls
        // left to make but the same two gaps.
        let again = e.tick(address("plo"), IndexMap::new());
        assert_eq!(again.final_phase, TickPhase::HoldingForApproval);
        assert!(e.controller.last_plan().unwrap().calls.is_empty());
    }

    /// ★ DRIFT IS CLOSED. Something changes the desktop behind the loop's
    /// back; the next tick puts it back. "Always attempting to reach state"
    /// as a test rather than a claim.
    #[test]
    fn drift_is_detected_and_closed_on_the_next_tick() {
        let mut w = MockWorld::default();
        w.state.insert("theme.name".into(), "vellum".into());
        let cat = Catalog::from_source(CATALOG).unwrap();
        let b = Bancada::from_source(DECL).unwrap();
        let e = engine(BancadaController::new(
            cat,
            Desired { state: b.lower() },
            Rung::L2,
            true,
            DECL.to_string(),
            Box::new(w),
        ));

        e.tick(address("plo"), IndexMap::new());
        let p = e.controller.last_plan().unwrap();
        assert!(
            p.calls.iter().any(|c| c.key == "theme.name"
                && c.have.as_deref() == Some("vellum")
                && c.want == "nord"),
            "the drifted key must be planned: {:?}",
            p.calls
        );

        let after = e.tick(address("plo"), IndexMap::new());
        assert_eq!(after.final_phase, TickPhase::Stable);
    }

    /// Every drifted field names the action that would close it, so an
    /// operator reading the report sees a verb rather than a bare key.
    #[test]
    fn drifted_fields_name_the_action_and_both_values() {
        let c = controller(Rung::L2, true);
        let ctx = c.observe(&address("plo"), &IndexMap::new()).unwrap();
        let report = c.diff(&ctx, &IndexMap::new()).unwrap();
        let theme = report
            .drifted_fields
            .iter()
            .find(|f| f.attribute == "theme.name")
            .expect("theme.name must appear");
        assert_eq!(theme.address, "theme-select");
        assert_eq!(theme.declared.as_deref(), Some("nord"));
        assert_eq!(theme.observed, None, "unset in a bare world");
    }

    /// A blind gap is reported as drift even on a converged desktop — it is a
    /// standing fact, and a report that hid it would claim a convergence the
    /// loop cannot actually hold.
    #[test]
    fn a_converged_desktop_still_reports_its_blind_keys() {
        let c = controller(Rung::L2, true);
        // Converge everything the loop can.
        let ctx = c.observe(&address("plo"), &IndexMap::new()).unwrap();
        let rd = RoutingDecision {
            action: RemediationAction::AutoCorrect,
            requeue_after: chrono::Duration::seconds(5),
            escalation: None,
        };
        let report = c.diff(&ctx, &IndexMap::new()).unwrap();
        c.act(&ctx, &rd, &report).unwrap();

        let ctx2 = c.observe(&address("plo"), &IndexMap::new()).unwrap();
        assert!(ctx2.plan.calls.is_empty(), "should be converged");
        let r2 = c.diff(&ctx2, &IndexMap::new()).unwrap();
        assert_eq!(c.classify(&r2), Severity::Cosmetic);
        assert!(!r2.clean(), "a blind gap is still drift");
    }

    /// The spec hash identifies the declaration, so two declarations cannot be
    /// confused in the outcome chain.
    #[test]
    fn the_spec_hash_tracks_the_declaration() {
        let a = controller(Rung::L2, true);
        let mut b = controller(Rung::L2, true);
        b.spec_source = "a different declaration".into();
        let ra = a
            .diff(
                &a.observe(&address("plo"), &IndexMap::new()).unwrap(),
                &IndexMap::new(),
            )
            .unwrap();
        let rb = b
            .diff(
                &b.observe(&address("plo"), &IndexMap::new()).unwrap(),
                &IndexMap::new(),
            )
            .unwrap();
        assert_ne!(ra.spec_hash, rb.spec_hash);
    }

    /// Attestation records every tick, in order.
    #[test]
    fn every_tick_is_attested() {
        let e = engine(controller(Rung::L2, true));
        for _ in 0..3 {
            e.tick(address("plo"), IndexMap::new());
        }
        assert_eq!(e.controller.ledger().len(), 3);
    }

    /// The mock world honours the plan, which is what makes every test above
    /// a statement about the loop rather than about the mock.
    #[test]
    fn the_world_is_actually_written() {
        let e = engine(controller(Rung::L2, true));
        e.tick(address("plo"), IndexMap::new());
        let keys: Vec<String> = e.controller.desired.state.keys().cloned().collect();
        let seen: Observed = e.controller.world.borrow().observe(&keys);
        assert_eq!(
            seen.state.get("theme.name").map(String::as_str),
            Some("nord")
        );
    }
}
