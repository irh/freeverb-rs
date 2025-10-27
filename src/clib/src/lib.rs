pub use freeverb::Freeverb;

/// Create a Freeverb instance with a given sample rate
///
/// The client is responsible for freeing the instance's memory when it's no longer required,
/// see `destroy()`.
#[unsafe(no_mangle)]
pub extern "C" fn create(sample_rate: usize) -> *mut Freeverb {
    Box::into_raw(Box::new(Freeverb::new(sample_rate)))
}

/// Destroy a Freeverb instance
///
/// # Safety
///
/// The instance must have been previously created using `create()`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn destroy(freeverb: *mut Freeverb) {
    if !freeverb.is_null() {
        unsafe {
            let _ = Box::from_raw(freeverb);
        }
    } else {
        panic!("")
    }
}

/// Process an audio buffer
///
/// # Safety
///
/// The input and output buffers must be (at least) sample_count f32s in size.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn process(
    freeverb: &mut Freeverb,
    input_l: *const f32,
    input_r: *const f32,
    output_l: *mut f32,
    output_r: *mut f32,
    sample_count: usize,
) { unsafe {
    for i in 0..sample_count as isize {
        let out = freeverb.tick((*input_l.offset(i) as f64, *input_r.offset(i) as f64));
        *output_l.offset(i) = out.0 as f32;
        *output_r.offset(i) = out.1 as f32;
    }
}}

#[unsafe(no_mangle)]
pub extern "C" fn set_dampening(freeverb: &mut Freeverb, value: f64) {
    freeverb.set_dampening(value)
}

#[unsafe(no_mangle)]
pub extern "C" fn set_freeze(freeverb: &mut Freeverb, value: bool) {
    freeverb.set_freeze(value)
}

#[unsafe(no_mangle)]
pub extern "C" fn set_wet(freeverb: &mut Freeverb, value: f64) {
    freeverb.set_wet(value)
}

#[unsafe(no_mangle)]
pub extern "C" fn set_width(freeverb: &mut Freeverb, value: f64) {
    freeverb.set_width(value)
}

#[unsafe(no_mangle)]
pub extern "C" fn set_dry(freeverb: &mut Freeverb, value: f64) {
    freeverb.set_dry(value)
}

#[unsafe(no_mangle)]
pub extern "C" fn set_room_size(freeverb: &mut Freeverb, value: f64) {
    freeverb.set_room_size(value)
}
