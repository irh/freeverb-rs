use crate::float::Float;

pub struct DelayLine<T> {
    buffer: Vec<T>,
    index: usize,
}

impl<T: Float> DelayLine<T> {
    pub fn new(length: usize) -> Self {
        Self {
            buffer: vec![T::from(0.0); length],
            index: 0,
        }
    }

    pub fn read(&self) -> T {
        self.buffer[self.index]
    }

    pub fn write_and_advance(&mut self, value: T) {
        self.buffer[self.index] = value;

        if self.index == self.buffer.len() - 1 {
            self.index = 0;
        } else {
            self.index += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    macro_rules! delay_line_test {
        ($name:ident, $length:expr) => {
            #[test]
            fn $name() {
                let mut line = super::DelayLine::new($length);
                for i in 0..$length {
                    assert_eq!(line.read(), 0.0);
                    line.write_and_advance(i as f32);
                }
                for i in 0..$length {
                    assert_eq!(line.read(), i as f32);
                    line.write_and_advance(0.0);
                }
            }
        };
    }

    delay_line_test!(length_1, 1);
    delay_line_test!(length_3, 3);
    delay_line_test!(length_10, 10);
}
