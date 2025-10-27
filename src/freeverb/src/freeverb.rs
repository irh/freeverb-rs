use crate::{all_pass::AllPass, comb::Comb, float::Float};

const FIXED_GAIN: f32 = 0.015;

const SCALE_WET: f32 = 3.0;
const SCALE_DAMPENING: f32 = 0.4;

const SCALE_ROOM: f32 = 0.28;
const OFFSET_ROOM: f32 = 0.7;

const STEREO_SPREAD: usize = 23;

const COMB_TUNING_L1: usize = 1116;
const COMB_TUNING_R1: usize = 1116 + STEREO_SPREAD;
const COMB_TUNING_L2: usize = 1188;
const COMB_TUNING_R2: usize = 1188 + STEREO_SPREAD;
const COMB_TUNING_L3: usize = 1277;
const COMB_TUNING_R3: usize = 1277 + STEREO_SPREAD;
const COMB_TUNING_L4: usize = 1356;
const COMB_TUNING_R4: usize = 1356 + STEREO_SPREAD;
const COMB_TUNING_L5: usize = 1422;
const COMB_TUNING_R5: usize = 1422 + STEREO_SPREAD;
const COMB_TUNING_L6: usize = 1491;
const COMB_TUNING_R6: usize = 1491 + STEREO_SPREAD;
const COMB_TUNING_L7: usize = 1557;
const COMB_TUNING_R7: usize = 1557 + STEREO_SPREAD;
const COMB_TUNING_L8: usize = 1617;
const COMB_TUNING_R8: usize = 1617 + STEREO_SPREAD;

const ALLPASS_TUNING_L1: usize = 556;
const ALLPASS_TUNING_R1: usize = 556 + STEREO_SPREAD;
const ALLPASS_TUNING_L2: usize = 441;
const ALLPASS_TUNING_R2: usize = 441 + STEREO_SPREAD;
const ALLPASS_TUNING_L3: usize = 341;
const ALLPASS_TUNING_R3: usize = 341 + STEREO_SPREAD;
const ALLPASS_TUNING_L4: usize = 225;
const ALLPASS_TUNING_R4: usize = 225 + STEREO_SPREAD;

pub struct Freeverb<T: Float = f64> {
    combs: [(Comb<T>, Comb<T>); 8],
    allpasses: [(AllPass<T>, AllPass<T>); 4],
    wet_gains: (T, T),
    wet: T,
    width: T,
    dry: T,
    input_gain: T,
    dampening: T,
    room_size: T,
    frozen: bool,
}

impl<T: Float> Freeverb<T> {
    pub fn new(sr: usize) -> Self {
        let mut freeverb = Freeverb::<T> {
            combs: [
                (
                    Comb::new(adjust_length(COMB_TUNING_L1, sr)),
                    Comb::new(adjust_length(COMB_TUNING_R1, sr)),
                ),
                (
                    Comb::new(adjust_length(COMB_TUNING_L2, sr)),
                    Comb::new(adjust_length(COMB_TUNING_R2, sr)),
                ),
                (
                    Comb::new(adjust_length(COMB_TUNING_L3, sr)),
                    Comb::new(adjust_length(COMB_TUNING_R3, sr)),
                ),
                (
                    Comb::new(adjust_length(COMB_TUNING_L4, sr)),
                    Comb::new(adjust_length(COMB_TUNING_R4, sr)),
                ),
                (
                    Comb::new(adjust_length(COMB_TUNING_L5, sr)),
                    Comb::new(adjust_length(COMB_TUNING_R5, sr)),
                ),
                (
                    Comb::new(adjust_length(COMB_TUNING_L6, sr)),
                    Comb::new(adjust_length(COMB_TUNING_R6, sr)),
                ),
                (
                    Comb::new(adjust_length(COMB_TUNING_L7, sr)),
                    Comb::new(adjust_length(COMB_TUNING_R7, sr)),
                ),
                (
                    Comb::new(adjust_length(COMB_TUNING_L8, sr)),
                    Comb::new(adjust_length(COMB_TUNING_R8, sr)),
                ),
            ],
            allpasses: [
                (
                    AllPass::new(adjust_length(ALLPASS_TUNING_L1, sr)),
                    AllPass::new(adjust_length(ALLPASS_TUNING_R1, sr)),
                ),
                (
                    AllPass::new(adjust_length(ALLPASS_TUNING_L2, sr)),
                    AllPass::new(adjust_length(ALLPASS_TUNING_R2, sr)),
                ),
                (
                    AllPass::new(adjust_length(ALLPASS_TUNING_L3, sr)),
                    AllPass::new(adjust_length(ALLPASS_TUNING_R3, sr)),
                ),
                (
                    AllPass::new(adjust_length(ALLPASS_TUNING_L4, sr)),
                    AllPass::new(adjust_length(ALLPASS_TUNING_R4, sr)),
                ),
            ],
            wet_gains: (T::default(), T::default()),
            wet: T::default(),
            dry: T::default(),
            input_gain: T::default(),
            width: T::default(),
            dampening: T::default(),
            room_size: T::default(),
            frozen: false,
        };

        freeverb.set_wet(T::from(1.0));
        freeverb.set_width(T::from(0.5));
        freeverb.set_dampening(T::from(0.5));
        freeverb.set_room_size(T::from(0.5));
        freeverb.set_frozen(false);

        freeverb
    }

    pub fn tick(&mut self, input: (T, T)) -> (T, T) {
        let input_mixed = (input.0 + input.1) * T::from(FIXED_GAIN) * self.input_gain;

        let mut out = (T::from(0.0), T::from(0.0));

        for combs in self.combs.iter_mut() {
            out.0 += combs.0.tick(input_mixed);
            out.1 += combs.1.tick(input_mixed);
        }

        for allpasses in self.allpasses.iter_mut() {
            out.0 = allpasses.0.tick(out.0);
            out.1 = allpasses.1.tick(out.1);
        }

        (
            out.0 * self.wet_gains.0 + out.1 * self.wet_gains.1 + input.0 * self.dry,
            out.1 * self.wet_gains.0 + out.0 * self.wet_gains.1 + input.1 * self.dry,
        )
    }

    pub fn set_dampening(&mut self, value: T) {
        self.dampening = value * T::from(SCALE_DAMPENING);
        self.update_combs();
    }

    pub fn set_freeze(&mut self, frozen: bool) {
        self.frozen = frozen;
        self.update_combs();
    }

    pub fn set_wet(&mut self, value: T) {
        self.wet = value * T::from(SCALE_WET);
        self.update_wet_gains();
    }

    pub fn set_width(&mut self, value: T) {
        self.width = value;
        self.update_wet_gains();
    }

    fn update_wet_gains(&mut self) {
        self.wet_gains = (
            self.wet * (self.width / T::from(2.0) + T::from(0.5)),
            self.wet * ((T::from(1.0) - self.width) / T::from(2.0)),
        )
    }

    fn set_frozen(&mut self, frozen: bool) {
        self.frozen = frozen;
        self.input_gain = if frozen { T::from(0.0) } else { T::from(1.0) };
        self.update_combs();
    }

    pub fn set_room_size(&mut self, value: T) {
        self.room_size = value * T::from(SCALE_ROOM) + T::from(OFFSET_ROOM);
        self.update_combs();
    }

    fn update_combs(&mut self) {
        let (feedback, dampening) = if self.frozen {
            (T::from(1.0), T::from(0.0))
        } else {
            (self.room_size, self.dampening)
        };

        for combs in self.combs.iter_mut() {
            combs.0.set_feedback(feedback);
            combs.1.set_feedback(feedback);

            combs.0.set_dampening(dampening);
            combs.1.set_dampening(dampening);
        }
    }

    pub fn set_dry(&mut self, value: T) {
        self.dry = value;
    }
}

fn adjust_length(length: usize, sr: usize) -> usize {
    (length as f64 * sr as f64 / 44100.0) as usize
}

#[cfg(test)]
mod tests {
    #[test]
    fn ticking_does_something() {
        let mut freeverb = super::Freeverb::new(44100);
        assert_eq!(freeverb.tick((1.0, 1.0)), (0.0, 0.0));
        for _ in 0..super::COMB_TUNING_R8 * 2 {
            freeverb.tick((0.0, 0.0));
        }
        assert_ne!(freeverb.tick((0.0, 0.0)), (0.0, 0.0));
    }
}
