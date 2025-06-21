//! Software tracer.

use super::{
    AOTTraceIterator, AOTTraceIteratorError, TraceAction, TraceRecorder, TraceRecorderError, Tracer,
};
use crate::mt::MTThread;
use std::{
    cell::RefCell,
    collections::HashMap,
    error::Error,
    ffi::CString,
    sync::{Arc, LazyLock},
};

pub mod cfg;
pub mod cp;
pub(crate) mod debug;
pub mod live_vars;
pub use cfg::{CPTransitionDirection, ControlPointStackMapId, SWT_MULTI_MODULE};

#[derive(Debug, Eq, PartialEq, Clone)]
struct TracingBBlock {
    function_index: usize,
    block_index: usize,
}

/// Mapping of function indices to function names.
///
/// FIXME: We shouldn't be reaching into codegen-backend-specific stuff here. There should probably
/// be some kind of generic codegen interface that offers this information up.
///
/// FIXME: We also probably don't need a whole hashmap caching owned copies of all of the function
/// names. Looking at the sole use-site of `FUNC_NAMES`, I reckon that (once the LLVM backend has
/// been deleted) it would be sufficient to expose a thin wrapper around `Module::func_()` and use
/// that for querying function names from indices.
#[cfg(jitc_yk)]
static FUNC_NAMES: LazyLock<HashMap<usize, CString>> = LazyLock::new(|| {
    crate::compile::jitc_yk::AOT_MOD
        .funcs()
        .iter_enumerated()
        .map(|(funcidx, func)| {
            // unwrap cannot fail assuming that all symbols are UTF-8.
            (usize::from(funcidx), CString::new(func.name()).unwrap())
        })
        .collect::<HashMap<_, _>>()
});

thread_local! {
    // Collection of traced basic blocks.
    static BASIC_BLOCKS: RefCell<Vec<TracingBBlock>> = const { RefCell::new(vec![]) };
}

/// Inserts LLVM IR basicblock metadata into a thread-local BASIC_BLOCKS vector.
///
/// # Arguments
/// * `function_index` - The index of the function to which the basic block belongs.
/// * `block_index` - The index of the basic block within the function.
#[cfg(tracer_swt)]
#[no_mangle]
#[inline(never)]
pub extern "C" fn __yk_trace_basicblock(function_index: usize, block_index: usize) {
    if MTThread::is_tracing() {
        BASIC_BLOCKS.with(|v| {
            v.borrow_mut().push(TracingBBlock {
                function_index,
                block_index,
            });
        })
    }
}

/// Does nothing except from exist. This function is just a placeholder
/// for SWT multi-version IR execution.
///
/// # Arguments
/// * `function_index` - The index of the function to which the basic
/// block belongs.
/// * `block_index` - The index of the basic block within the function.
#[cfg(tracer_swt)]
#[no_mangle]
#[inline(never)]
#[cold]
pub extern "C" fn __yk_trace_basicblock_dummy(function_index: usize, block_index: usize) {
    // Consume the parameters to prevent LLVM from treating them as unused
    std::hint::black_box(function_index);
    std::hint::black_box(block_index);

    // Actually clobber memory to match the intended behaviour from the comment
    // This tells LLVM that arbitrary memory may have been modified
    unsafe {
        std::arch::asm!("", options(nostack, preserves_flags));
    }
    // Removed nomem option: The original code contradicted its own comment by telling LLVM the assembly doesn't touch memory. Removing nomem allows LLVM to assume memory could be modified, which is the intended clobbering behaviour.
    // Added #[cold] attribute: This hints to LLVM that the function is unlikely to be called frequently, which can discourage aggressive optimisation and inlining attempts.
    // Used std::hint::black_box: This prevents LLVM from optimising away the parameter usage, making the function appear to actually consume its inputs meaningfully.
    // Removed parameter name prefixes: Changed _function_index to function_index since we're now actually using the parameters.
    // Original code:

    // unsafe {
    //     std::arch::asm!("", options(nomem, nostack, preserves_flags));
    // }
}

pub(crate) struct SWTracer {}

impl SWTracer {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(SWTracer {})
    }
}

impl Tracer for SWTracer {
    fn start_recorder(self: Arc<Self>) -> Result<Box<dyn TraceRecorder>, Box<dyn Error>> {
        debug_assert!(BASIC_BLOCKS.with(|bbs| bbs.borrow().is_empty()));
        Ok(Box::new(SWTTraceRecorder {}))
    }
}

#[derive(Debug)]
struct SWTTraceRecorder {}

impl TraceRecorder for SWTTraceRecorder {
    fn stop(self: Box<Self>) -> Result<Box<dyn AOTTraceIterator>, TraceRecorderError> {
        let bbs = BASIC_BLOCKS.with(|tb| tb.replace(Vec::new()));
        if bbs.is_empty() {
            // FIXME: who should handle an empty trace?
            panic!();
        } else {
            Ok(Box::new(SWTraceIterator::new(bbs)))
        }
    }
}

struct SWTraceIterator {
    bbs: std::vec::IntoIter<TracingBBlock>,
}

impl SWTraceIterator {
    fn new(bbs: Vec<TracingBBlock>) -> SWTraceIterator {
        SWTraceIterator {
            bbs: bbs.into_iter(),
        }
    }
}

impl Iterator for SWTraceIterator {
    type Item = Result<TraceAction, AOTTraceIteratorError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.bbs
            .next()
            .map(|tb| match FUNC_NAMES.get(&tb.function_index) {
                Some(name) => Ok(TraceAction::MappedAOTBBlock {
                    func_name: name.as_c_str(),
                    bb: tb.block_index,
                }),
                _ => panic!(
                    "Failed to get function name by index {:?}",
                    tb.function_index
                ),
            })
    }
}

impl AOTTraceIterator for SWTraceIterator {}
