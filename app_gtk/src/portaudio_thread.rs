use super::*;

use portaudio::{error::Error, PortAudio};

pub fn run_audio<Module: AudioModule>(
    command_receiver: crossbeam_channel::Receiver<Command>,
    sample_rate: usize,
) {
    let mut processor = Module::create_processor(sample_rate);

    const CHANNELS: usize = 2;
    const FRAMES_PER_BUFFER: usize = 128;
    const SAMPLES_PER_BUFFER: usize = FRAMES_PER_BUFFER * CHANNELS;

    audio_thread_priority::promote_current_thread_to_real_time(
        SAMPLES_PER_BUFFER as u32,
        sample_rate as u32,
    )
    .unwrap();

    let port_audio = PortAudio::new().unwrap();
    let settings = port_audio
        .default_duplex_stream_settings::<f32, f32>(
            CHANNELS as i32,
            CHANNELS as i32,
            sample_rate as f64,
            FRAMES_PER_BUFFER as u32,
        )
        .unwrap();

    let mut stream = port_audio.open_blocking_stream(settings).unwrap();
    stream.start().unwrap();

    let mut input_buffer = [0.0f32; SAMPLES_PER_BUFFER];

    loop {
        while let Ok(command) = command_receiver.try_recv() {
            processor.handle_command(command);
        }

        match stream.read(FRAMES_PER_BUFFER as u32) {
            Err(Error::InputOverflowed) => println!("Input underflowed"),
            Err(err) => println!("Read from stream failed - {:?}", err),
            Ok(input) => {
                assert_eq!(input.len(), SAMPLES_PER_BUFFER);
                input_buffer.copy_from_slice(input);
            }
        }

        match stream.write(FRAMES_PER_BUFFER as u32, |output| {
            assert_eq!(output.len(), SAMPLES_PER_BUFFER);
            processor.process_stereo(&input_buffer[0..output.len()], output);
        }) {
            Err(Error::OutputUnderflowed) => println!("Output underflowed"),
            Err(err) => println!("Write to stream failed - {:?}", err),
            _ => (),
        };
    }
}
