use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Freeverb(freeverb::Freeverb);

impl Default for Freeverb {
    fn default() -> Self {
        Self(freeverb::Freeverb::new(44100))
    }
}

#[wasm_bindgen]
impl Freeverb {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn process(
        &mut self,
        input_l: &[f32],
        input_r: &[f32],
        output_l: &mut [f32],
        output_r: &mut [f32],
    ) {
        for i in 0..input_l.len() {
            let out = self.0.tick((input_l[i] as f64, input_r[i] as f64));
            output_l[i] = out.0 as f32;
            output_r[i] = out.1 as f32;
        }
    }

    pub fn set_dampening(&mut self, value: f64) {
        self.0.set_dampening(value)
    }

    pub fn set_freeze(&mut self, value: bool) {
        self.0.set_freeze(value)
    }

    pub fn set_wet(&mut self, value: f64) {
        self.0.set_wet(value)
    }

    pub fn set_width(&mut self, value: f64) {
        self.0.set_width(value)
    }

    pub fn set_dry(&mut self, value: f64) {
        self.0.set_dry(value)
    }

    pub fn set_room_size(&mut self, value: f64) {
        self.0.set_room_size(value)
    }
}
