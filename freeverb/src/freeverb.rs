use super::{all_pass::AllPass, comb::Comb};

const FIXED_GAIN: f64 = 0.015;

const SCALE_WET: f64 = 3.0;
const SCALE_DAMPENING: f64 = 0.4;

const SCALE_ROOM: f64 = 0.28;
const OFFSET_ROOM: f64 = 0.7;

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

pub struct Freeverb {
    combs: [(Comb, Comb); 8],
    allpasses: [(AllPass, AllPass); 4],
    wet_gains: (f64, f64),
    wet: f64,
    width: f64,
    dry: f64,
    input_gain: f64,
    dampening: f64,
    room_size: f64,
    frozen: bool,
}

fn adjust_length(length: usize, sr: usize) -> usize {
    (length as f64 * sr as f64 / 44100.0) as usize
}

impl Freeverb {
    pub fn new(sr: usize) -> Self {
        let mut freeverb = Freeverb {
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
            wet_gains: (0.0, 0.0),
            wet: 0.0,
            dry: 0.0,
            input_gain: 0.0,
            width: 0.0,
            dampening: 0.0,
            room_size: 0.0,
            frozen: false,
        };

        freeverb.set_wet(1.0);
        freeverb.set_width(0.5);
        freeverb.set_dampening(0.5);
        freeverb.set_room_size(0.5);
        freeverb.set_frozen(false);

        freeverb
    }

    pub fn tick(&mut self, input: (f64, f64)) -> (f64, f64) {
        let input_mixed = (input.0 + input.1) * FIXED_GAIN * self.input_gain;

        let mut out = (0.0, 0.0);

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

    pub fn set_dampening(&mut self, value: f64) {
        self.dampening = value * SCALE_DAMPENING;
        self.update_combs();
    }

    pub fn set_freeze(&mut self, frozen: bool) {
        self.frozen = frozen;
        self.update_combs();
    }

    pub fn set_wet(&mut self, value: f64) {
        self.wet = value * SCALE_WET;
        self.update_wet_gains();
    }

    pub fn set_width(&mut self, value: f64) {
        self.width = value;
        self.update_wet_gains();
    }

    fn update_wet_gains(&mut self) {
        self.wet_gains = (
            self.wet * (self.width / 2.0 + 0.5),
            self.wet * ((1.0 - self.width) / 2.0),
        )
    }

    fn set_frozen(&mut self, frozen: bool) {
        self.frozen = frozen;
        self.input_gain = if frozen { 0.0 } else { 1.0 };
        self.update_combs();
    }

    pub fn set_room_size(&mut self, value: f64) {
        self.room_size = value * SCALE_ROOM + OFFSET_ROOM;
        self.update_combs();
    }

    fn update_combs(&mut self) {
        let (feedback, dampening) = if self.frozen {
            (1.0, 0.0)
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

    pub fn set_dry(&mut self, value: f64) {
        self.dry = value;
    }
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
