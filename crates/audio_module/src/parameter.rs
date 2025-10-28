use crate::{
    string_converter::{
        DefaultStringConverter, FloatStringConverter, PercentStringConverter, StringConverter,
    },
    value_converter::{
        DefaultValueConverter, LinearValueConverter, LogValueConverter, ValueConverter,
    },
};

pub enum ValueType {
    Float,
    Bool,
}

pub trait Parameter {
    fn name(&self) -> String;
    fn default_user_value(&self) -> f32;

    fn value_type(&self) -> ValueType {
        ValueType::Float
    }

    fn make_value_converter(&self) -> Box<dyn ValueConverter> {
        Box::new(DefaultValueConverter {})
    }

    fn make_string_converter(&self) -> Box<dyn StringConverter> {
        Box::new(DefaultStringConverter {})
    }
}

pub struct BoolParameter {
    pub name: String,
    pub default_user_value: bool,
}

impl BoolParameter {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            default_user_value: false,
        }
    }

    pub fn default_user_value(mut self, default: bool) -> Self {
        self.default_user_value = default;
        self
    }
}

impl Parameter for BoolParameter {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn default_user_value(&self) -> f32 {
        if self.default_user_value { 1.0 } else { 0.0 }
    }

    fn value_type(&self) -> ValueType {
        ValueType::Bool
    }
}

pub struct FloatParameter {
    pub name: String,
    pub unit: String,
    pub min_user_value: f32,
    pub max_user_value: f32,
    pub default_user_value: f32,
    pub value_converter_maker: fn(&FloatParameter) -> Box<dyn ValueConverter>,
    pub string_converter_maker: fn(&FloatParameter) -> Box<dyn StringConverter>,
}

impl FloatParameter {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            unit: String::default(),
            min_user_value: 0.0,
            max_user_value: 1.0,
            default_user_value: 0.0,
            value_converter_maker: linear_value_converter,
            string_converter_maker: float_string_converter,
        }
    }

    pub fn unit(mut self, unit: &str) -> Self {
        self.unit = unit.to_string();
        self
    }

    pub fn range(mut self, min: f32, max: f32) -> Self {
        self.min_user_value = min;
        self.max_user_value = max;
        self
    }

    pub fn default_user_value(mut self, default: f32) -> Self {
        self.default_user_value = default;
        self
    }

    pub fn value_converter(
        mut self,
        converter: fn(&FloatParameter) -> Box<dyn ValueConverter>,
    ) -> Self {
        self.value_converter_maker = converter;
        self
    }

    pub fn string_converter(
        mut self,
        converter: fn(&FloatParameter) -> Box<dyn StringConverter>,
    ) -> Self {
        self.string_converter_maker = converter;
        self
    }
}

impl Parameter for FloatParameter {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn default_user_value(&self) -> f32 {
        self.default_user_value
    }

    fn make_value_converter(&self) -> Box<dyn ValueConverter> {
        (self.value_converter_maker)(self)
    }

    fn make_string_converter(&self) -> Box<dyn StringConverter> {
        (self.string_converter_maker)(self)
    }
}

pub fn linear_value_converter(parameter: &FloatParameter) -> Box<dyn ValueConverter> {
    Box::new(LinearValueConverter::new(
        parameter.min_user_value,
        parameter.max_user_value,
    ))
}

pub fn log_value_converter(parameter: &FloatParameter) -> Box<dyn ValueConverter> {
    Box::new(LogValueConverter::new(
        parameter.min_user_value,
        parameter.max_user_value,
    ))
}

pub fn float_string_converter(parameter: &FloatParameter) -> Box<dyn StringConverter> {
    Box::new(FloatStringConverter::new(parameter.unit.clone()))
}

pub fn percent_string_converter(_: &FloatParameter) -> Box<dyn StringConverter> {
    Box::new(PercentStringConverter {})
}

pub trait ParameterProvider {
    fn parameter_count() -> usize;
    fn parameter(id: usize) -> Box<dyn Parameter>;
}
