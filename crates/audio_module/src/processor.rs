use crate::CommandHandler;

pub trait AudioProcessor: CommandHandler + Send + Sync + 'static {
    fn process_stereo(&mut self, input: &[f32], output: &mut [f32]);
}
