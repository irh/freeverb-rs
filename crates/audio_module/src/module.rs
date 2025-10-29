use crate::{AudioProcessor, Parameter};

pub trait AudioModule: ParameterProvider {
    type Processor: AudioProcessor;

    fn create_processor(sample_rate: usize) -> Self::Processor;
}

pub trait ParameterProvider {
    fn parameter_count() -> usize;
    fn parameter(id: usize) -> Box<dyn Parameter>;
}
