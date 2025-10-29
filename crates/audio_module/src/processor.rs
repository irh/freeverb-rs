use crate::CommandHandler;

pub trait AudioProcessor: CommandHandler + Send + Sync + 'static {
    fn process(&mut self, input: &[f32], output: &mut [f32], channels: u32);
}
