//! Where observed desktop state comes from.
//!
//! A trait because the real backend is a compositor that does not exist yet.
//! The loop is proven against a mock, which is the fleet's default delivery
//! method — and it means the daemon's logic is testable with no desktop at all.
//!
//! Lives in its own module rather than in `main.rs` because the lava controller
//! owns the world: the engine drives `observe` and `act` through `&self`, so the
//! world has to sit behind the controller's interior mutability rather than be
//! passed down the daemon's call stack.

use saihai_forge::reconcile::{apply, Observed, Plan};
use std::collections::BTreeMap;

/// A desktop that can be read and written.
pub trait World {
    /// Read the fields the declaration cares about.
    fn observe(&self, keys: &[String]) -> Observed;
    /// Perform the convergeable calls. Returns how many landed.
    fn apply_calls(&mut self, p: &Plan) -> usize;
    fn describe(&self) -> String;
}

/// An in-memory desktop. The daemon's logic is identical against this and
/// against a real compositor; only the impl differs.
#[derive(Default)]
pub struct MockWorld {
    pub state: BTreeMap<String, String>,
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
        let next = apply(
            p,
            &Observed {
                state: self.state.clone(),
            },
        );
        self.state = next.state;
        p.calls.len()
    }
    fn describe(&self) -> String {
        let n = self.state.len();
        format!("mock ({n} keys)")
    }
}
