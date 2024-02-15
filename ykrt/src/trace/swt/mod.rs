//! Software tracer.

use crate::frame::BitcodeSection;

use super::{errors::InvalidTraceError, AOTTraceIterator, TraceRecorder, TracedAOTBlock};
use std::sync::Once;
use std::{cell::RefCell, collections::HashMap, error::Error, ffi::CString, sync::Arc};

static FUNC_NAMES_INIT: Once = Once::new();

#[derive(Debug, Eq, PartialEq)]
struct TracingBlock {
    function_index: usize,
    block_index: usize,
}

thread_local! {
    static BASIC_BLOCKS: RefCell<Vec<TracingBlock>> = RefCell::new(vec![]);
    static FUNC_NAMES: RefCell<HashMap<usize, CString>> = RefCell::new(HashMap::new());
}

extern "C" {
    fn get_function_names(
        section: *const BitcodeSection,
        result: *mut *mut IRFunctionNameIndex,
        len: *mut usize,
    );
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IRFunctionNameIndex {
    pub index: usize,
    pub name: *const libc::c_char,
}
/// Inserts LLVM IR basicblock metadata into a thread-local BASIC_BLOCKS vector.
///
/// # Arguments
/// * `function_index` - The index of the function to which the basic block belongs.
/// * `block_index` - The index of the basic block within the function.
pub fn trace_basicblock(function_index: usize, block_index: usize) {
    BASIC_BLOCKS.with(|v| {
        v.borrow_mut().push(TracingBlock {
            function_index,
            block_index,
        });
    })
}

pub(crate) struct SWTracer {}

impl super::Tracer for SWTracer {
    fn start_recorder(self: Arc<Self>) -> Result<Box<dyn TraceRecorder>, Box<dyn Error>> {
        BASIC_BLOCKS.with(|bbs| {
            bbs.borrow_mut().clear();
        });
        return Ok(Box::new(SWTTraceRecorder {
            promotions: RefCell::new(Vec::new()),
        }));
    }
}

impl SWTracer {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(SWTracer {})
    }
}

struct SWTTraceRecorder {
    promotions: RefCell<Vec<usize>>,
}

impl TraceRecorder for SWTTraceRecorder {
    fn stop(
        self: Box<Self>,
    ) -> Result<(Box<dyn AOTTraceIterator>, Box<[usize]>), InvalidTraceError> {
        let mut aot_blocks: Vec<TracedAOTBlock> = vec![];
        BASIC_BLOCKS.with(|tb| {
            FUNC_NAMES.with(|fnames| {
                FUNC_NAMES_INIT.call_once(|| {
                    let mut functions: *mut IRFunctionNameIndex = std::ptr::null_mut();
                    let bc_section = crate::compile::jitc_llvm::llvmbc_section();
                    let mut functions_len: usize = 0;
                    unsafe {
                        get_function_names(
                            &BitcodeSection {
                                data: bc_section.as_ptr(),
                                len: u64::try_from(bc_section.len()).unwrap(),
                            },
                            &mut functions,
                            &mut functions_len,
                        );
                        for entry in std::slice::from_raw_parts(functions, functions_len) {
                            fnames.borrow_mut().insert(
                                entry.index,
                                std::ffi::CStr::from_ptr(entry.name).to_owned(),
                            );
                        }
                    }
                });

                aot_blocks = tb
                    .borrow()
                    .iter()
                    .map(|tb| match fnames.borrow_mut().get(&tb.function_index) {
                        Some(name) => TracedAOTBlock::Mapped {
                            func_name: name.to_owned(),
                            bb: tb.block_index,
                        },
                        _ => panic!(
                            "Failed to get function name by index {:?}",
                            tb.function_index
                        ),
                    })
                    .collect();
            })
        });
        if aot_blocks.is_empty() {
            return Err(InvalidTraceError::EmptyTrace);
        } else {
            return Ok((
                Box::new(SWTraceIterator {
                    trace: aot_blocks.into_iter(),
                }),
                self.promotions.into_inner().into_boxed_slice(),
            ));
        }
    }
    fn promote_usize(&self, val: usize) -> bool {
        // Return false by default until implemented.
        return false;
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

impl AOTTraceIterator for SWTraceIterator {}
