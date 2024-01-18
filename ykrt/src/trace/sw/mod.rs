//! Software tracer.

use crate::frame::BitcodeSection;

use super::{errors::InvalidTraceError, TraceCollector, TraceIterator, TracedAOTBlock};
use std::{cell::RefCell, collections::HashMap, error::Error, ffi::CString, sync::Arc};

pub(crate) struct SWTracer {}

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
    ) -> libc::c_uint;
    fn free_key_values(result: *mut IRFunctionNameIndex);
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IRFunctionNameIndex {
    pub index: u32, //libc::c_int,
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
            unsafe {
                let func_indices: Vec<u32> = mtt.borrow().iter().map(|x| x.function_index).collect();
                let mut result: *mut IRFunctionNameIndex = std::ptr::null_mut();
                let bc = crate::compile::jitc_llvm::llvmbc_section();
                let mut result_len: libc::size_t = 0;
                let status = get_function_names(
                    &BitcodeSection {
                        data: bc.as_ptr(),
                        len: u64::try_from(bc.len()).unwrap(),
                    },
                    func_indices.as_ptr(),
                    func_indices.len(),
                    &mut result,
                    &mut result_len,
                );
                if status == 0 {
                    let func_names = std::slice::from_raw_parts(result, result_len);

                    for entry in func_names {
                        let st = std::ffi::CStr::from_ptr(entry.name).to_owned();
                        let index = entry.index;
                        func_store.insert(index, st);
                    }
                    free_key_values(result);
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
        for aot in aot_blocks.iter() {
            println!("{:?}", aot);
        }
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
