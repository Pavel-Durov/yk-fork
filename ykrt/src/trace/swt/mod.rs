//! Software tracer.

use super::{
    AOTTraceIterator, AOTTraceIteratorError, TraceAction, TraceRecorder, TraceRecorderError, Tracer,
};
use crate::mt::{MTThread, DEFAULT_TRACE_TOO_LONG};
use std::{
    cell::RefCell,
    collections::HashMap,
    error::Error,
    ffi::CString,
    sync::{Arc, LazyLock},
};

#[derive(Debug, Eq, PartialEq, Clone)]
struct TracingBBlock {
    function_index: usize,
    block_index: usize,
}
use crate::compile::jitc_yk::AOT_MOD;

// Mapping of function indexes to function names.
static FUNC_NAMES: LazyLock<HashMap<usize, String>> = LazyLock::new(|| {
    let mut fnames = HashMap::new();
    let funcs = AOT_MOD.get_funcs();
    for (index, function) in funcs.iter().enumerate() {
        // println!("Index: {:?}, Value: {}", index, value);
        fnames.insert(index, function.name().to_string());
    }
    fnames
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
pub extern "C" fn yk_trace_basicblock(function_index: usize, block_index: usize) {
    MTThread::with(|mtt| {
        if mtt.is_tracing() {
            BASIC_BLOCKS.with(|v| {
                v.borrow_mut().push(TracingBBlock {
                    function_index,
                    block_index,
                });
            })
        }
    });
}

// extern "C" {
//     fn get_function_names(
//         section: *const BitcodeSection,
//         result: *mut *mut IRFunctionNameIndex,
//         len: *mut usize,
//     );
// }

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IRFunctionNameIndex {
    pub index: usize,
    pub name: *const libc::c_char,
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

struct SWTTraceRecorder {}

impl TraceRecorder for SWTTraceRecorder {
    fn stop(self: Box<Self>) -> Result<Box<dyn AOTTraceIterator>, TraceRecorderError> {
        let bbs = BASIC_BLOCKS.with(|tb| tb.replace(Vec::new()));
        if bbs.len() > DEFAULT_TRACE_TOO_LONG {
            Err(TraceRecorderError::TraceTooLong)
        } else if bbs.is_empty() {
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
                    func_name:  CString::new(name.as_str()).unwrap(),
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
