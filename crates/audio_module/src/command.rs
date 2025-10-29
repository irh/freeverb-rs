pub enum Command {
    SetParameter(usize, f32),
}

pub trait CommandHandler {
    fn handle_command(&mut self, command: Command);
}
