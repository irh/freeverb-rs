pub trait ValueConverter {
    fn user_to_linear(&self, value: f32) -> f32;
    fn linear_to_user(&self, value: f32) -> f32;
}

pub struct DefaultValueConverter {}

impl ValueConverter for DefaultValueConverter {
    fn user_to_linear(&self, value: f32) -> f32 {
        value
    }

    fn linear_to_user(&self, value: f32) -> f32 {
        value
    }
}

pub struct LinearValueConverter {
    pub min_user_value: f32,
    pub user_value_range: f32,
}

impl LinearValueConverter {
    pub fn new(min: f32, max: f32) -> Self {
        Self {
            min_user_value: min,
            user_value_range: max - min,
        }
    }
}

impl ValueConverter for LinearValueConverter {
    fn user_to_linear(&self, value: f32) -> f32 {
        (value - self.min_user_value) / self.user_value_range
    }

    fn linear_to_user(&self, value: f32) -> f32 {
        self.min_user_value + value * self.user_value_range
    }
}

pub struct LogValueConverter {
    pub log_min_user_value: f32,
    pub log_user_value_range: f32,
}

impl LogValueConverter {
    pub fn new(min: f32, max: f32) -> Self {
        Self {
            log_min_user_value: min.log2(),
            log_user_value_range: max.log2() - min.log2(),
        }
    }
}

impl ValueConverter for LogValueConverter {
    fn user_to_linear(&self, value: f32) -> f32 {
        (value.log2() - self.log_min_user_value) / self.log_user_value_range
    }

    fn linear_to_user(&self, value: f32) -> f32 {
        (self.log_min_user_value + value * self.log_user_value_range).exp2()
    }
}
