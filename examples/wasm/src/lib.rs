// Re-export the C API
pub use freeverb_clib::*;

// Provide a function that allocate
#[unsafe(no_mangle)]
pub extern "C" fn createBuffer(size: usize) -> *mut f32 {
    let mut buf = Vec::<f32>::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr as *mut f32
}
