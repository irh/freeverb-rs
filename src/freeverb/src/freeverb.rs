use crate::{all_pass::AllPass, comb::Comb, float::Float, tuning::*};

/// A processor for the Freeverb reverb algorithm.
///
/// 64-bit processing is enabled by default.
/// 32-bit processing can be optionally enabled by using `f32` as the generic `T` parameter.
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
    /// Produces a new processor with the given sample rate.
    ///
    /// The algorithm's tuning constants were designed for a sample rate of 44.1kHz,
    /// with a note that they will 'probably be OK' for 48kHz, but would require scaling for other
    /// sample rates. In this implementation the constants are scaled when using any sample rate,
    /// including 48kHz.
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

    /// Processes a single pair of values.
    ///
    /// The pair's values are the left/right channels of a single processing frame.
    ///
    /// To process a buffer of frames this function should be called repeatedly.
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

    /// Sets the processors dampening value.
    ///
    /// The value should be in the range `0..=1`.
    pub fn set_dampening(&mut self, value: T) {
        self.dampening = value * T::from(SCALE_DAMPENING);
        self.update_combs();
    }

    /// Enables or disables the reverb's 'freeze' feature.
    pub fn set_freeze(&mut self, frozen: bool) {
        self.frozen = frozen;
        self.update_combs();
    }

    /// Sets the amount of the 'dry' signal to include in the processor's output.
    ///
    /// The dry signal is the unmodified input, without any of the reverb's output.
    ///
    /// The value should be in the range `0..=1`.
    pub fn set_dry(&mut self, value: T) {
        self.dry = value;
    }

    /// Sets the amount of the 'wet' signal to include in the processor's output.
    ///
    /// The wet signal is the reverb's output, without any of the unmodified input.
    ///
    /// The value should be in the range `0..=1`.
    pub fn set_wet(&mut self, value: T) {
        self.wet = value * T::from(SCALE_WET);
        self.update_wet_gains();
    }

    /// Sets the processor's stereo width.
    ///
    /// The value should be in the range `0..=1`.
    pub fn set_width(&mut self, value: T) {
        self.width = value;
        self.update_wet_gains();
    }

    /// Sets the processor's 'room size'.
    ///
    /// The value should be in the range `0..=1`.
    pub fn set_room_size(&mut self, value: T) {
        self.room_size = value * T::from(SCALE_ROOM) + T::from(OFFSET_ROOM);
        self.update_combs();
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
