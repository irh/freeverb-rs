use audio_module::{Command, Parameter};

use gtk::prelude::*;
use gtk::{Orientation, ToggleButton};

pub fn make_toggle(
    parameter: Box<dyn Parameter>,
    id: usize,
    command_sender: crossbeam_channel::Sender<Command>,
) -> gtk::Box {
    let button = ToggleButton::new_with_label(parameter.name().as_str());
    button.set_active(parameter.default_user_value() != 0.0);
    button.connect_toggled(move |button| {
        command_sender
            .send(Command::SetParameter(
                id,
                if button.get_active() { 1.0f32 } else { 0.0f32 },
            ))
            .unwrap();
    });

    let container = gtk::Box::new(Orientation::Vertical, 2);
    container.pack_start(&button, true, false, 0);
    container
}
