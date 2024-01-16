//! Software tracer.

use super::{errors::InvalidTraceError, TraceCollector, TraceIterator, TracedAOTBlock};
use std::{cell::RefCell, error::Error, ffi::CString, sync::Arc};

pub(crate) struct SWTracer {}

thread_local! {
    static BASIC_BLOCKS: RefCell<Vec<BB>> = RefCell::new(vec![]);
}

/// Inserts LLVM IR basicblock metadata into a thread-local BASIC_BLOCKS vector.
///
/// # Arguments
/// * `function_index` - The index of the function to which the basic block belongs.
/// * `block_index` - The index of the basic block within the function.
pub fn trace_basicblock(function_index: u32, block_index: u32) {
    BASIC_BLOCKS.with(|v| {
        v.borrow_mut().push(BB {
            function_index,
            block_index,
        });
    })
}

impl super::Tracer for SWTracer {
    fn start_collector(self: Arc<Self>) -> Result<Box<dyn TraceCollector>, Box<dyn Error>> {
        return Ok(Box::new(SWTTraceCollector {}));
    }
}

impl SWTracer {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(SWTracer {})
    }
}

struct SWTTraceCollector {}

impl TraceCollector for SWTTraceCollector {
    fn stop_collector(self: Box<Self>) -> Result<Box<dyn TraceIterator>, InvalidTraceError> {
        let mut aot_blocks: Vec<TracedAOTBlock> = vec![];
        BASIC_BLOCKS.with(|mtt| {
            for bb in mtt.borrow().iter() {
                // TODO: Get function name from IR by function_index
                let func_name = CString::new("test_func").expect("Failed to create CString");
                aot_blocks.push(TracedAOTBlock::Mapped {
                    func_name,
                    bb: bb.block_index as usize,
                })
            }
        });
        if aot_blocks.is_empty() {
            return Err(InvalidTraceError::EmptyTrace);
        } else {
            return Ok(Box::new(SWTraceIterator {
                trace: aot_blocks.into_iter(),
            }));
        }
    }
}

struct SWTraceIterator {
    trace: std::vec::IntoIter<TracedAOTBlock>,
}

impl Iterator for SWTraceIterator {
    type Item = TracedAOTBlock;
    fn next(&mut self) -> Option<Self::Item> {
        self.trace.next()
    }
}

impl TraceIterator for SWTraceIterator {}

#[derive(Debug, Eq, PartialEq)]
struct BB {
    function_index: u32,
    block_index: u32,
}
