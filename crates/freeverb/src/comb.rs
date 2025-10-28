use crate::{delay_line::DelayLine, float::Float};

pub struct Comb<T> {
    delay_line: DelayLine<T>,
    feedback: T,
    filter_state: T,
    dampening: T,
    dampening_inverse: T,
}

impl<T: Float> Comb<T> {
    pub fn new(delay_length: usize) -> Self {
        Self {
            delay_line: DelayLine::new(delay_length),
            feedback: T::from(0.5),
            filter_state: T::from(0.0),
            dampening: T::from(0.5),
            dampening_inverse: T::from(0.5),
        }
    }

    pub fn set_dampening(&mut self, value: T) {
        self.dampening = value;
        self.dampening_inverse = T::from(1.0) - value;
    }

    pub fn set_feedback(&mut self, value: T) {
        self.feedback = value;
    }

    pub fn tick(&mut self, input: T) -> T {
        let output = self.delay_line.read();

        self.filter_state = output * self.dampening_inverse + self.filter_state * self.dampening;

        self.delay_line
            .write_and_advance(input + self.filter_state * self.feedback);

        output
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic_ticking() {
        let mut comb = super::Comb::new(2);
        assert_eq!(comb.tick(1.0), 0.0);
        assert_eq!(comb.tick(0.0), 0.0);
        assert_eq!(comb.tick(0.0), 1.0);
        assert_eq!(comb.tick(0.0), 0.0);
        assert_eq!(comb.tick(0.0), 0.25);
        assert_eq!(comb.tick(0.0), 0.125);
        assert_eq!(comb.tick(0.0), 0.125);
        assert_eq!(comb.tick(0.0), 0.09375);
    }
}
