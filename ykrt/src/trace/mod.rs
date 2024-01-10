//! Utilities for collecting and decoding traces.

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
// #[cfg(tracer_swt)]
pub(crate) mod sw;

pub(crate) use errors::InvalidTraceError;

/// A `Tracer` is a front-end to a tracer backend (e.g. hardware or software tracing). The tracer
/// backend may have its own configuration options, which is why `Tracer` does not have a `new`
/// method.
pub(crate) trait Tracer: Send + Sync {
    /// Start collecting a trace of the current thread.
    fn start_collector(self: Arc<Self>) -> Result<Box<dyn TraceCollector>, Box<dyn Error>>;
}

/// Return a [Tracer] instance or `Err` if none can be found. The [Tracer] returned will be
/// selected on a combination of what the platform can support and other (possibly run-time) user
/// configuration.
pub(crate) fn default_tracer() -> Result<Arc<dyn Tracer>, Box<dyn Error>> {
    #[cfg(tracer_hwt)]
    {
        return Ok(Arc::new(hwt::HWTracer::new()?));
    }

    #[allow(unreachable_code)]
    Err("No tracing backend this platform/configuration.".into())
}

/// Return a [Tracer] instance or `Err` if none can be found. The [Tracer] returned will be
/// selected on a combination of what the platform can support and other (possibly run-time) user
/// configuration.
// #[cfg(tracer_swt)]
pub(crate) fn software_tracer() -> Result<Arc<dyn Tracer>, Box<dyn Error>> {
    return Ok(Arc::new(sw::SWTracer::new()?));
}

/// Represents a thread which is currently tracing.
pub(crate) trait TraceCollector {
    /// Stop collecting a trace of the current thread and return an iterator which successively
    /// produces the traced blocks.
    fn stop_collector(self: Box<Self>) -> Result<Box<dyn TraceIterator>, InvalidTraceError>;
}

/// An iterator which takes an underlying raw trace and successively produces [TracedAOTBlock]s.
pub(crate) trait TraceIterator: Iterator<Item = TracedAOTBlock> + Send {}

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
        ///
        /// The special value `usize::MAX` indicates unmappable code.
        bb: usize,
    },
    /// One or more machine blocks that could not be mapped.
    ///
    /// This usually means that the blocks were compiled outside of ykllvm.
    Unmappable,
}

impl TracedAOTBlock {
    pub fn new_mapped(func_name: CString, bb: usize) -> Self {
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
