use crate::{delay_line::DelayLine, float::Float};

pub struct AllPass<T> {
    delay_line: DelayLine<T>,
}

impl<T: Float> AllPass<T> {
    pub fn new(delay_length: usize) -> Self {
        Self {
            delay_line: DelayLine::new(delay_length),
        }
    }

    pub fn tick(&mut self, input: T) -> T {
        let delayed = self.delay_line.read();
        let output = -input + delayed;

        // in the original version of freeverb this is a member which is never modified
        let feedback = T::from(0.5);

        self.delay_line
            .write_and_advance(input + delayed * feedback);

        output
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic_ticking() {
        let mut allpass = super::AllPass::new(2);
        assert_eq!(allpass.tick(1.0), -1.0);
        assert_eq!(allpass.tick(0.0), 0.0);
        assert_eq!(allpass.tick(0.0), 1.0);
        assert_eq!(allpass.tick(0.0), 0.0);
        assert_eq!(allpass.tick(0.0), 0.5);
        assert_eq!(allpass.tick(0.0), 0.0);
        assert_eq!(allpass.tick(0.0), 0.25);
    }
}
