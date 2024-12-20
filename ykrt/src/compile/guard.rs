//! Guards: track the state of a guard in a trace.

use crate::{
    compile::CompiledTrace,
    mt::{HotThreshold, MT},
};
use parking_lot::Mutex;
use std::sync::Arc;

/// Responsible for tracking how often a guard in a `CompiledTrace` fails. A hotness counter is
/// incremented each time the matching guard failure in a `CompiledTrace` is triggered. Also stores
/// the side-trace once its compiled.
#[derive(Debug)]
pub(crate) struct Guard(Mutex<GuardState>);

#[derive(Debug)]
enum GuardState {
    Counting(HotThreshold),
    SideTracing,
    Compiled(Arc<dyn CompiledTrace>),
}

impl Guard {
    pub(crate) fn new() -> Self {
        Self(Mutex::new(GuardState::Counting(0)))
    }

    /// This guard has failed (i.e. evaluated to true/false when false/true was expected). Returns
    /// `true` if this guard has failed often enough to be worth side-tracing.
    pub fn inc_failed(&self, mt: &Arc<MT>) -> bool {
        let mut lk = self.0.lock();
        match &*lk {
            GuardState::Counting(x) => {
                if x + 1 >= mt.sidetrace_threshold() {
                    *lk = GuardState::SideTracing;
                    true
                } else {
                    *lk = GuardState::Counting(x + 1);
                    false
                }
            }
            GuardState::SideTracing => false,
            GuardState::Compiled(_) => false,
        }
    }

    /// Stores a compiled side-trace inside this guard.
    pub fn set_ctr(&self, ctr: Arc<dyn CompiledTrace>) {
        let mut lk = self.0.lock();
        match &*lk {
            GuardState::SideTracing => *lk = GuardState::Compiled(ctr),
            _ => panic!(),
        }
    }

    /// Return the compiled side-trace or None if no side-trace has been compiled.
    pub fn ctr(&self) -> Option<Arc<dyn CompiledTrace>> {
        let lk = self.0.lock();
        match &*lk {
            GuardState::Compiled(ctr) => Some(Arc::clone(ctr)),
            _ => None,
        }
    }
}

/// Identify a [Guard] within a trace.
///
/// This is guaranteed to be an index into an array that is freely convertible to/from [usize].
#[derive(Clone, Copy, Debug)]
pub(crate) struct GuardIdx(usize);

impl From<usize> for GuardIdx {
    fn from(v: usize) -> Self {
        Self(v)
    }
}

impl From<GuardIdx> for usize {
    fn from(v: GuardIdx) -> Self {
        v.0
    }
}
