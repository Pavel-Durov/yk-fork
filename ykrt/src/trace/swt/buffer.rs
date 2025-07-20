use std::alloc::{Layout, alloc, dealloc, handle_alloc_error};
use std::collections::HashMap;
use yksmp::Location::{Direct, Indirect};
use yksmp::Record;

use crate::trace::swt::cp::{ControlPointStackMapId, LiveVarsBuffer};

/// A safer wrapper around allocated buffer memory with proper RAII
#[derive(Debug)]
pub(crate) struct AlignedBuffer {
    ptr: *mut u8,
    layout: Layout,
}

impl AlignedBuffer {
    /// Create a new aligned buffer with the specified size
    pub(crate) unsafe fn new(size: usize) -> Self {
        let layout = Layout::from_size_align(size, 16)
            .expect("Failed to create layout for live vars buffer");
        let ptr = unsafe { 
            let ptr =  alloc(layout);
            if ptr.is_null() {
                handle_alloc_error(layout);
            }
            std::ptr::write_bytes(ptr, 0, size);
            ptr
        };
        
        AlignedBuffer { ptr, layout }
    }
    
    /// Get the raw pointer to the buffer
    pub(crate) fn as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    
    /// Get the size of the buffer
    pub(crate) fn size(&self) -> usize {
        self.layout.size()
    }
    
    /// Get the layout of the buffer  
    pub(crate) fn layout(&self) -> Layout {
        self.layout
    }
}

impl Drop for AlignedBuffer {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                dealloc(self.ptr, self.layout);
            }
        }
    }
}

// SAFETY: AlignedBuffer is Send because we control access to the raw pointer
// and ensure proper ownership transfer
unsafe impl Send for AlignedBuffer {}

// NOTE: We do NOT implement Sync for AlignedBuffer because concurrent access
// to the same buffer would be unsafe. Each buffer should be used by only one
// thread at a time, or protected by explicit synchronization.

thread_local! {
    static OPT_BUFFER: std::cell::RefCell<Option<AlignedBuffer>> = std::cell::RefCell::new(None);
    static UNOPT_BUFFER: std::cell::RefCell<Option<AlignedBuffer>> = std::cell::RefCell::new(None);
}

/// Calculates the size of the live vars buffer.
/// The buffer is aligned to 16 bytes.
pub(crate) fn calculate_live_vars_buffer_size(src_rec: &Record) -> i32 {
    let mut src_val_buffer_size: i32 = 0;
    for (_, src_var) in src_rec.live_vals.iter().enumerate() {
        match src_var.get(0).unwrap() {
            Indirect(_, _, src_val_size) | Direct(_, _, src_val_size) => {
                src_val_buffer_size += *src_val_size as i32;
            }
            _ => { /* DO NOTHING */ }
        }
    }
    // Align the buffer size to 16 bytes (only round up, never down)
    if src_val_buffer_size % 16 == 0 {
        src_val_buffer_size
    } else {
        ((src_val_buffer_size / 16) + 1) * 16
    }
}

/// Gets or creates a buffer for the given stack map ID.
/// Returns (ptr, layout, size) tuple.
pub(crate) fn get_or_create_buffer(src_rec: &Record, smid: ControlPointStackMapId) -> (*mut u8, Layout, usize) {
    let src_val_buffer_size = calculate_live_vars_buffer_size(src_rec);

    if src_val_buffer_size == 0 {
        return (std::ptr::null_mut(), Layout::new::<u8>(), 0);
    }

    let buffer_cell = match smid {
        ControlPointStackMapId::UnOpt => &UNOPT_BUFFER,
        ControlPointStackMapId::Opt => &OPT_BUFFER,
    };

    buffer_cell.with(|cell| {
        let mut buffer_opt = cell.borrow_mut();
        
        // Check if we need a new buffer (first use or size changed)
        let needs_new_buffer = buffer_opt.as_ref()
            .map_or(true, |buf| buf.size() < src_val_buffer_size as usize);
            
        if needs_new_buffer {
            // Create new buffer with required size
            let new_buffer = unsafe { AlignedBuffer::new(src_val_buffer_size as usize) };
            *buffer_opt = Some(new_buffer);
        }
        
        let buffer = buffer_opt.as_ref().unwrap();
        (buffer.as_ptr(), buffer.layout(), buffer.size())
    })
}

/// Creates a LiveVarsBuffer from buffer components
pub(crate) fn create_live_vars_buffer(
    ptr: *mut u8, 
    layout: Layout, 
    size: usize, 
    variables: HashMap<i32, i32>
) -> LiveVarsBuffer {
    LiveVarsBuffer {
        ptr,
        layout,
        variables,
        size: size as i32,
    }
} 