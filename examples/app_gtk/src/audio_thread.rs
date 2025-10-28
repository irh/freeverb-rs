use {
    audio_module::{AudioModule, AudioProcessor, Command, CommandHandler},
    cpal::traits::{DeviceTrait, HostTrait, StreamTrait},
    ringbuf::{
        HeapRb,
        traits::{Consumer, Producer, Split},
    },
};

#[allow(unused)]
pub struct AudioStreams {
    pub output: cpal::Stream,
    pub input: cpal::Stream,
}

pub fn start_audio<Module: AudioModule>(
    command_receiver: crossbeam_channel::Receiver<Command>,
    sample_rate: usize,
) -> Result<AudioStreams, ()> {
    let mut processor = Module::create_processor(sample_rate);

    const CHANNELS: usize = 2;
    const FRAMES_PER_BUFFER: usize = 128;
    const SAMPLES_PER_BUFFER: usize = FRAMES_PER_BUFFER * CHANNELS;

    let host = cpal::default_host();

    let input_device = host
        .default_input_device()
        .expect("failed to find a default input device");

    let output_device = host
        .default_output_device()
        .expect("failed to find a default output device");

    let stream_config = cpal::StreamConfig {
        channels: CHANNELS as u16,
        sample_rate: cpal::SampleRate(sample_rate as u32),
        buffer_size: cpal::BufferSize::Fixed(FRAMES_PER_BUFFER as u32),
    };

    let mut process_buffer = [0.0f32; SAMPLES_PER_BUFFER];
    let ring_buffer = HeapRb::new(SAMPLES_PER_BUFFER * 2);
    let (mut to_output, mut from_input) = ring_buffer.split();

    let input = input_device
        .build_input_stream(
            &stream_config,
            move |data: &[f32], _info: &cpal::InputCallbackInfo| {
                debug_assert!(data.len() == SAMPLES_PER_BUFFER);

                while let Ok(command) = command_receiver.try_recv() {
                    processor.handle_command(command);
                }

                processor.process_stereo(data, &mut process_buffer);

                to_output.push_slice(&process_buffer);
            },
            move |err| eprintln!("Error on audio input stream: {}", err),
            None,
        )
        .expect("Failed to create input audio stream");

    let output = output_device
        .build_output_stream(
            &stream_config,
            move |data: &mut [f32], _info: &cpal::OutputCallbackInfo| {
                // println!("output buffer");
                debug_assert!(data.len() == SAMPLES_PER_BUFFER);

                let consumed = from_input.pop_slice(data);

                if consumed < SAMPLES_PER_BUFFER {
                    println!("output underflowed");
                }
            },
            move |err| eprintln!("Error on audio output stream: {}", err),
            None,
        )
        .expect("Failed to create input audio stream");

    if let Err(error) = input.play() {
        eprintln!("Error while starting input audio stream: {}", error);
        return Err(());
    }

    if let Err(error) = output.play() {
        eprintln!("Error while starting output audio stream: {}", error);
        return Err(());
    }

    println!("Started audio i/o");
    Ok(AudioStreams { input, output })
}
