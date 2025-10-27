#[macro_use]
extern crate num_derive;

use {
    audio_module::{
        AudioModule, AudioProcessor, BoolParameter, Command, CommandHandler, FloatParameter,
        Parameter, ParameterProvider, percent_string_converter,
    },
    freeverb::{Float, Freeverb},
    num_traits::FromPrimitive,
};

#[derive(FromPrimitive)]
pub enum Parameters {
    Dampening,
    Width,
    RoomSize,
    Freeze,
    Dry,
    Wet,
}

pub struct FreeverbProcessor<T: Float = f64> {
    freeverb: Freeverb<T>,
}

impl FreeverbProcessor {
    fn new(sample_rate: usize) -> Self {
        Self {
            freeverb: Freeverb::new(sample_rate),
        }
    }
}

impl<T: Float> CommandHandler for FreeverbProcessor<T> {
    fn handle_command(&mut self, command: Command) {
        match command {
            Command::SetParameter(id, value) => match Parameters::from_usize(id).unwrap() {
                Parameters::Dampening => {
                    self.freeverb.set_dampening(value.into());
                }
                Parameters::Width => {
                    self.freeverb.set_width(value.into());
                }
                Parameters::RoomSize => {
                    self.freeverb.set_room_size(value.into());
                }
                Parameters::Freeze => {
                    self.freeverb.set_freeze(value != 0.0);
                }
                Parameters::Dry => {
                    self.freeverb.set_dry(value.into());
                }
                Parameters::Wet => {
                    self.freeverb.set_wet(value.into());
                }
            },
        }
    }
}

impl<T: Float> AudioProcessor for FreeverbProcessor<T> {
    fn process_stereo(&mut self, input: &[f32], output: &mut [f32]) {
        assert!(input.len() == output.len());

        for i in (0..input.len()).step_by(2) {
            let result = self.freeverb.tick((input[i].into(), input[i + 1].into()));

            output[i] = result.0.to_f32();
            output[i + 1] = result.1.to_f32();
        }
    }
}

pub struct FreeverbModule {}

impl AudioModule for FreeverbModule {
    type Processor = FreeverbProcessor;

    fn create_processor(sample_rate: usize) -> Self::Processor {
        FreeverbProcessor::new(sample_rate)
    }
}

impl ParameterProvider for FreeverbModule {
    fn parameter_count() -> usize {
        (0..usize::max_value())
            .take_while(|&x| Parameters::from_usize(x).is_some())
            .count()
    }

    fn parameter(id: usize) -> Box<dyn Parameter> {
        match Parameters::from_usize(id).unwrap() {
            Parameters::Dampening => Box::new(
                FloatParameter::new("Dampening")
                    .string_converter(percent_string_converter)
                    .default_user_value(0.5),
            ),
            Parameters::Width => Box::new(
                FloatParameter::new("Width")
                    .string_converter(percent_string_converter)
                    .default_user_value(0.5),
            ),
            Parameters::RoomSize => Box::new(
                FloatParameter::new("Room Size")
                    .string_converter(percent_string_converter)
                    .default_user_value(0.5),
            ),
            Parameters::Freeze => Box::new(BoolParameter::new("Freeze")),
            Parameters::Dry => Box::new(
                FloatParameter::new("Dry")
                    .string_converter(percent_string_converter)
                    .default_user_value(0.0),
            ),
            Parameters::Wet => Box::new(
                FloatParameter::new("Wet")
                    .string_converter(percent_string_converter)
                    .default_user_value(1.0),
            ),
        }
    }
}
