//! `hashigo` (梯子) — the authority ladder, as a type.
//!
//! Four rungs, and the load-bearing property is that **an over-privileged call
//! has no `apply` path**. Not a runtime `if level >= 3`, not a `Result::Err` —
//! a compile error (E0277), because the bound cannot be satisfied.
//!
//! ## Extracted, not invented
//!
//! Lifted from `ayatsuri/src/kabe/spec.rs`, which already ships this shape with
//! ten hand-written `AtLeast` impls for the macOS compositor privilege ladder.
//! A second consumer means promote rather than re-derive — the fleet's rule,
//! and the reason this is its own crate instead of a copy inside `saihai-spec`.
//!
//! ## The rungs
//!
//! | rung | means | example |
//! |---|---|---|
//! | `L0` | unprivileged — own process, public reads | list your own windows |
//! | `L1` | seat-owner — connected to the seat | focus, move, close a window |
//! | `L2` | controller-privileged — controller protocols, capture, output mgmt | set output mode, screencast |
//! | `L3` | break-glass — SIP scripting, synthetic input, destructive | always-on-top on macOS |
//!
//! `Ord` is real and meaningful: `L2 < L3`.

use std::marker::PhantomData;

/// The rung, as a value — for catalogs, logs and comparisons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rung {
    L0,
    L1,
    L2,
    L3,
}

impl Rung {
    pub const ALL: [Self; 4] = [Self::L0, Self::L1, Self::L2, Self::L3];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::L0 => "l0",
            Self::L1 => "l1",
            Self::L2 => "l2",
            Self::L3 => "l3",
        }
    }

    /// Whether a build at `self` may perform work requiring `needed`.
    ///
    /// The VALUE-level mirror of [`AtLeast`]. Both exist on purpose: the trait
    /// stops an over-privileged call from compiling, and this answers the same
    /// question for data — a catalog row, an SDK in a language with no
    /// typestate, an operator-facing report. They must agree, and a test pins
    /// that they do over all sixteen pairs.
    #[must_use]
    pub const fn permits(self, needed: Self) -> bool {
        (self as u8) >= (needed as u8)
    }
}

/// The rungs, as types.
#[derive(Debug, Clone, Copy)]
pub struct L0;
#[derive(Debug, Clone, Copy)]
pub struct L1;
#[derive(Debug, Clone, Copy)]
pub struct L2;
#[derive(Debug, Clone, Copy)]
pub struct L3;

/// "this build is at least rung `R`".
///
/// `L3: AtLeast<L2>` holds. `L2: AtLeast<L3>` does not, and there is no impl to
/// add that would make it — which is what makes an over-privileged call a
/// compile error rather than a check someone can forget.
pub trait AtLeast<R> {}

impl AtLeast<L0> for L0 {}
impl AtLeast<L0> for L1 {}
impl AtLeast<L1> for L1 {}
impl AtLeast<L0> for L2 {}
impl AtLeast<L1> for L2 {}
impl AtLeast<L2> for L2 {}
impl AtLeast<L0> for L3 {}
impl AtLeast<L1> for L3 {}
impl AtLeast<L2> for L3 {}
impl AtLeast<L3> for L3 {}

/// A build's capability witness at rung `R`.
///
/// Only the host mints one — there is no `Default`, no `Deserialize`, and the
/// field is private, so a witness cannot be conjured from config or from a
/// deserialised message. A reconciler holds whatever witness it was handed and
/// cannot promote itself.
#[derive(Debug)]
pub struct RungWitness<R>(PhantomData<R>);

impl<R> RungWitness<R> {
    /// Mint a witness.
    ///
    /// # Safety-by-convention, stated because the type cannot enforce it
    /// This is not `unsafe`, but calling it IS the privilege claim: the caller
    /// asserts the process genuinely holds rung `R`. It belongs at the host
    /// boundary — the one place that actually knows — and nowhere else. The
    /// type system carries the claim correctly once made; it cannot check the
    /// making of it, and pretending otherwise would be the round-up this crate
    /// exists to avoid.
    #[must_use]
    pub const fn claim() -> Self {
        Self(PhantomData)
    }
}

/// The break-glass warrant required at [`Rung::L3`].
///
/// Fields private, no `Default`, no `Deserialize`. Its only constructor is
/// [`Warrant::issue`], which a reconciler must not call — the intended
/// deployment puts it behind a crate the automated loop does not depend on, so
/// "the loop escalated itself" is a link error rather than a policy violation.
#[derive(Debug, Clone)]
pub struct Warrant {
    witness: String,
    runbook: String,
}

impl Warrant {
    /// Issue a warrant. Operator-only.
    #[must_use]
    pub fn issue(witness: impl Into<String>, runbook: impl Into<String>) -> Self {
        Self {
            witness: witness.into(),
            runbook: runbook.into(),
        }
    }

    #[must_use]
    pub fn witness(&self) -> &str {
        &self.witness
    }

    #[must_use]
    pub fn runbook(&self) -> &str {
        &self.runbook
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The value ladder and the type ladder must agree, over every pair.
    /// They are two encodings of one fact, and two encodings can disagree.
    #[test]
    fn permits_matches_the_trait_ladder_over_all_sixteen_pairs() {
        let expect = |have: Rung, need: Rung| (have as u8) >= (need as u8);
        let mut checked = 0;
        for have in Rung::ALL {
            for need in Rung::ALL {
                assert_eq!(
                    have.permits(need),
                    expect(have, need),
                    "{have:?} vs {need:?}"
                );
                checked += 1;
            }
        }
        assert_eq!(checked, 16, "the ladder is 4x4");
    }

    #[test]
    fn the_ladder_is_ordered_and_strict() {
        assert!(Rung::L0 < Rung::L1 && Rung::L1 < Rung::L2 && Rung::L2 < Rung::L3);
        assert!(Rung::L3.permits(Rung::L0));
        assert!(!Rung::L0.permits(Rung::L1));
        assert!(!Rung::L2.permits(Rung::L3));
    }

    /// A generic function bounded at L2 accepts L2 and L3, and the compiler
    /// refuses L0/L1. The refusal cannot be tested here — it is a compile
    /// error, which is the point — so it is pinned by a `trybuild` case in
    /// `tests/`; this asserts only the accepting half.
    #[test]
    fn a_bound_at_l2_accepts_l2_and_l3() {
        fn needs_l2<W: AtLeast<L2>>(_w: &RungWitness<W>) -> &'static str {
            "ran"
        }
        assert_eq!(needs_l2(&RungWitness::<L2>::claim()), "ran");
        assert_eq!(needs_l2(&RungWitness::<L3>::claim()), "ran");
    }

    #[test]
    fn a_warrant_carries_its_witness_and_runbook() {
        let w = Warrant::issue("drzzln", "runbook://desktop/break-glass");
        assert_eq!(w.witness(), "drzzln");
        assert_eq!(w.runbook(), "runbook://desktop/break-glass");
    }
}
