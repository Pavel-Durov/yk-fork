//! Software tracer.

use crate::frame::BitcodeSection;

use super::{errors::InvalidTraceError, TraceCollector, TraceIterator, TracedAOTBlock};
use std::{cell::RefCell, collections::HashMap, error::Error, ffi::CString, sync::Arc};

#[derive(Debug, Eq, PartialEq)]
struct BB {
    function_index: u32,
    block_index: u32,
}

thread_local! {
    static BASIC_BLOCKS: RefCell<Vec<BB>> = RefCell::new(vec![]);
}

extern "C" {
    fn get_function_names(
        section: *const BitcodeSection,
        vec: *const libc::c_uint,
        vec_size: usize,
        result: *mut *mut IRFunctionNameIndex,
        len: *mut libc::size_t,
    );
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IRFunctionNameIndex {
    pub index: u32,
    pub name: *const libc::c_char,
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

pub(crate) struct SWTracer {}

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
            let mut func_store: HashMap<u32, CString> = HashMap::new();
            let func_indices: Vec<u32> = mtt.borrow().iter().map(|x| x.function_index).collect();
            let mut func_names: *mut IRFunctionNameIndex = std::ptr::null_mut();
            let mut func_names_len: libc::size_t = 0;
            let bc_section = crate::compile::jitc_llvm::llvmbc_section();
            unsafe {
                get_function_names(
                    &BitcodeSection {
                        data: bc_section.as_ptr(),
                        len: u64::try_from(bc_section.len()).unwrap(),
                    },
                    func_indices.as_ptr(),
                    func_indices.len(),
                    &mut func_names,
                    &mut func_names_len,
                );
                for entry in std::slice::from_raw_parts(func_names, func_names_len) {
                    func_store.insert(entry.index, std::ffi::CStr::from_ptr(entry.name).to_owned());
                }
            }
            aot_blocks = mtt
                .borrow()
                .iter()
                .map(|bb| TracedAOTBlock::Mapped {
                    func_name: func_store.get(&bb.function_index).unwrap().to_owned(),
                    bb: bb.block_index as usize,
                })
                .collect();
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
