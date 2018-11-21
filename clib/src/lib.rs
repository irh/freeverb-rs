extern crate freeverb;
pub use freeverb::Freeverb;

#[no_mangle]
pub extern fn create(sample_rate: usize) -> *mut Freeverb {
    Box::into_raw(Box::new(Freeverb::new(sample_rate)))
}

#[no_mangle]
pub unsafe extern fn destroy(freeverb: *mut Freeverb) {
    assert!(!freeverb.is_null());
    Box::from_raw(freeverb);
}

#[no_mangle]
pub unsafe extern fn process(
    freeverb: &mut Freeverb,
    input_l: *const f32,
    input_r: *const f32,
    output_l: *mut f32,
    output_r: *mut f32,
    sample_count: usize,
) {
    for i in 0..sample_count as isize {
        let out = freeverb.tick((*input_l.offset(i) as f64, *input_r.offset(i) as f64));
        *output_l.offset(i) = out.0 as f32;
        *output_r.offset(i) = out.1 as f32;
    }
}

#[no_mangle]
pub extern fn set_dampening(freeverb: &mut Freeverb, value: f64) {
    freeverb.set_dampening(value)
}

#[no_mangle]
pub extern fn set_freeze(freeverb: &mut Freeverb, value: bool) {
    freeverb.set_freeze(value)
}

#[no_mangle]
pub extern fn set_wet(freeverb: &mut Freeverb, value: f64) {
    freeverb.set_wet(value)
}

#[no_mangle]
pub extern fn set_width(freeverb: &mut Freeverb, value: f64) {
    freeverb.set_width(value)
}

#[no_mangle]
pub extern fn set_dry(freeverb: &mut Freeverb, value: f64) {
    freeverb.set_dry(value)
}

#[no_mangle]
pub extern fn set_room_size(freeverb: &mut Freeverb, value: f64) {
    freeverb.set_room_size(value)
}
