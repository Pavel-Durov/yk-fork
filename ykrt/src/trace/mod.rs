//! Record and process traces.

#![allow(clippy::len_without_is_empty)]
#![allow(clippy::new_without_default)]
#![allow(clippy::missing_safety_doc)]

mod errors;
use std::{
    error::Error,
    ffi::{CStr, CString},
    sync::Arc,
};

#[cfg(tracer_hwt)]
pub(crate) mod hwt;
#[cfg(tracer_swt)]
pub(crate) mod swt;

pub(crate) use errors::InvalidTraceError;

/// A `Tracer` is a front-end to a tracer backend (e.g. hardware or software tracing). The tracer
/// backend may have its own configuration options, which is why `Tracer` does not have a `new`
/// method.
pub(crate) trait Tracer: Send + Sync {
    /// Start recording a trace of the current thread.
    fn start_recorder(self: Arc<Self>) -> Result<Box<dyn TraceRecorder>, Box<dyn Error>>;
}

/// Return a [Tracer] instance or `Err` if none can be found. The [Tracer] returned will be
/// selected on a combination of what the platform can support and other (possibly run-time) user
/// configuration.
pub(crate) fn default_tracer() -> Result<Arc<dyn Tracer>, Box<dyn Error>> {
    #[cfg(tracer_hwt)]
    {
        return Ok(Arc::new(hwt::HWTracer::new()?));
    }
    #[cfg(tracer_swt)]
    {
        return Ok(Arc::new(swt::SWTracer::new()?));
    }
    #[allow(unreachable_code)]
    Err("No tracing backend this platform/configuration.".into())
}

/// Represents a thread which is currently tracing.
pub(crate) trait TraceRecorder {
    /// Stop recording a trace of the current thread and return an iterator which successively
    /// produces the traced blocks.
    fn stop(
        self: Box<Self>,
    ) -> Result<(Box<dyn AOTTraceIterator>, Box<[usize]>), InvalidTraceError>;
    /// Records `val` as a value to be promoted at this point in the trace. Returns `true` if
    /// recording succeeded or `false` otherwise. If `false` is returned, this `TraceRecorder` is
    /// no longer able to trace successfully and further calls are probably pointless, though they
    /// must not cause the tracer to enter undefined behaviour territory.
    fn promote_usize(&self, val: usize) -> bool;
}

/// An iterator which takes an underlying raw trace and successively produces [TracedAOTBlock]s.
pub(crate) trait AOTTraceIterator: Iterator<Item = TracedAOTBlock> + Send {}

/// An AOT LLVM IR block that has been traced at JIT time.
#[derive(Debug, Eq, PartialEq)]
pub enum TracedAOTBlock {
    /// A sucessfully mapped block.
    Mapped {
        /// The name of the function containing the block.
        ///
        /// PERF: Use a string pool to avoid duplicated function names in traces.
        func_name: CString,
        /// The index of the block within the function.
        bb: usize,
    },
    /// One or more machine blocks that could not be mapped.
    ///
    /// This usually means that the blocks were compiled outside of ykllvm.
    Unmappable,
}

impl TracedAOTBlock {
    pub fn new_mapped(func_name: CString, bb: usize) -> Self {
        // At one point, `bb = usize::MAX` was a special value, but it no longer is. We believe
        // that no part of the code sets/checks for this value, but just in case there is a
        // laggardly part of the code which does so, we've left this `assert` behind to catch it.
        debug_assert_ne!(bb, usize::MAX);
        Self::Mapped { func_name, bb }
    }

    pub fn new_unmappable() -> Self {
        Self::Unmappable
    }

    /// If `self` is a mapped block, return the function name, otherwise panic.
    pub fn func_name(&self) -> &CStr {
        if let Self::Mapped { func_name, .. } = self {
            func_name.as_c_str()
        } else {
            panic!();
        }
    }

    /// If `self` is a mapped block, return the basic block index, otherwise panic.
    pub fn bb(&self) -> usize {
        if let Self::Mapped { bb, .. } = self {
            *bb
        } else {
            panic!();
        }
    }

    /// Determines whether `self` represents unmappable code.
    pub fn is_unmappable(&self) -> bool {
        matches!(self, Self::Unmappable)
    }
}

#[cfg(tracer_swt)]
pub(crate) fn trace_basicblock(function_index: usize, block_index: usize) {
    swt::trace_basicblock(function_index, block_index)
}
