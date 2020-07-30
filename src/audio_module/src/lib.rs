pub mod parameter;
pub mod string_converter;
pub mod value_converter;

pub use self::parameter::*;
pub use self::string_converter::*;
pub use self::value_converter::*;

pub enum Command {
    SetParameter(usize, f32),
}

pub trait CommandHandler {
    fn handle_command(&mut self, command: Command);
}

pub trait AudioProcessor: CommandHandler + Send + Sync + 'static {
    fn process_stereo(&mut self, input: &[f32], output: &mut [f32]);
}

pub trait AudioModule: ParameterProvider {
    type Processor: AudioProcessor;

    fn create_processor(sample_rate: usize) -> Self::Processor;
}
