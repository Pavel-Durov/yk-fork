//! Software tracer.

use super::{errors::InvalidTraceError, TraceCollector, TraceIterator, TracedAOTBlock};
use hwtracer::Tracer;
use std::{cell::RefCell, error::Error, sync::Arc};

pub(crate) struct SWTracer {}

thread_local! {
    static BASIC_BLOCKS: RefCell<Vec<BasicBlock>> = RefCell::new(vec![]);
}

impl super::Tracer for SWTracer {
    fn start_collector(mut self: Arc<Self>) -> Result<Box<dyn TraceCollector>, Box<dyn Error>> {
        todo!("[SWTracer] start_collector");
        return Ok(Box::new(SWTTraceCollector {}));
    }
}


impl SWTracer {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(SWTracer {
            
        })
    }
    pub fn stop_collection(mut self) {
        todo!("implement me!")
    }
}

struct SWTTraceCollector {}

impl SWTTraceCollector {
    fn collect(mut self: Box<Self>, function_index: i32, block_index: i32) {
        todo!("[SWTTraceCollector] collect");
    }
}

impl TraceCollector for SWTTraceCollector {
    fn stop_collector(self: Box<Self>) -> Result<Box<dyn TraceIterator>, InvalidTraceError> {
        todo!("[TraceCollector] stop_collector");
        // return Err(InvalidTraceError::EmptyTrace);
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


struct BasicBlock {
    function_index: u32,
    block_index: u32,
}

/// Collects LLVM IR basicblock metadata and stores it in the thread-local
/// BASIC_BLOCKS vector.
///
/// # Arguments
///
/// * `function_index` - The index of the function to which the basic block belongs.
/// * `block_index` - The index of the basic block within the function.
pub fn trace_basicblock(function_index: u32, block_index: u32) {
    BASIC_BLOCKS.with(|v| {
        v.borrow_mut().push(BasicBlock {
            function_index,
            block_index,
        });
    })
}
