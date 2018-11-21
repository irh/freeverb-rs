pub trait StringConverter {
    fn to_string(&self, value: f32) -> String;
}

#[derive(Clone)]
pub struct DefaultStringConverter {}

impl StringConverter for DefaultStringConverter {
    fn to_string(&self, value: f32) -> String {
        format!("{:.0}", value)
    }
}

#[derive(Clone)]
pub struct BoolStringConverter {}

impl StringConverter for BoolStringConverter {
    fn to_string(&self, value: f32) -> String {
        if value == 0.0 { "off" } else { "on" }.to_string()
    }
}

#[derive(Clone)]
pub struct FloatStringConverter {
    unit: String,
}

impl FloatStringConverter {
    pub fn new(unit: String) -> Self {
        Self { unit }
    }
}

impl StringConverter for FloatStringConverter {
    fn to_string(&self, value: f32) -> String {
        format!("{:.0} {1}", value, self.unit)
    }
}

#[derive(Clone)]
pub struct PercentStringConverter {}

impl StringConverter for PercentStringConverter {
    fn to_string(&self, value: f32) -> String {
        format!("{:.0} %", value * 100.0)
    }
}
