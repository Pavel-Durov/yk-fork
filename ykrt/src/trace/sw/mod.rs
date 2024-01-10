//! Software tracer.

use super::{errors::InvalidTraceError, TraceCollector, TraceIterator};
use std::{error::Error, sync::Arc};

pub(crate) struct SWTracer {}

impl super::Tracer for SWTracer {
    fn start_collector(mut self: Arc<Self>) -> Result<Box<dyn TraceCollector>, Box<dyn Error>> {
        println!("[SWTracer] start_collector");
        return Ok(Box::new(SWTTraceCollector {
            blocks: Arc::new(vec![]),
        }));
    }
}

impl SWTracer {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        println!("[SWTracer] new");
        Ok(SWTracer {})
    }
    pub fn stop_collection(mut self) {}
}

struct SWTrace {}

impl SWTrace {
    fn new() -> Self {
        SWTrace {}
    }
}

struct SWTTraceCollector {
    blocks: Arc<Vec<TraceBlock>>,
}
impl SWTTraceCollector {
    fn collect(mut self: Box<Self>, function_index: i32, block_index: i32) {
        println!("[TraceCollector] collect called");
        let mut_blocks = Arc::get_mut(&mut self.blocks).unwrap();
        mut_blocks.push(TraceBlock::new(function_index, block_index))
    }
}

impl TraceCollector for SWTTraceCollector {
    fn stop_collector(self: Box<Self>) -> Result<Box<dyn TraceIterator>, InvalidTraceError> {
        println!("[SWTracer] stop_collector");
        return Err(InvalidTraceError::EmptyTrace);
    }
}

pub struct TraceBlock {
    function_index: i32,
    block_index: i32,
}

impl TraceBlock {
    pub fn new(function_index: i32, block_index: i32) -> Self {
        println!("[TraceBlock] collect called");
        TraceBlock {
            function_index,
            block_index,
        }
    }
}
