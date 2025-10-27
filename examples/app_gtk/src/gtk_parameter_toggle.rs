use {
    audio_module::{Command, Parameter},
    gtk::{prelude::*, Align, Orientation, ToggleButton},
};

pub fn make_toggle(
    parameter: Box<dyn Parameter>,
    id: usize,
    command_sender: crossbeam_channel::Sender<Command>,
) -> gtk::Box {
    let button = ToggleButton::with_label(parameter.name().as_str());
    button.set_active(parameter.default_user_value() != 0.0);
    button.connect_toggled(move |button| {
        command_sender
            .send(Command::SetParameter(
                id,
                if button.is_active() { 1.0f32 } else { 0.0f32 },
            ))
            .unwrap();
    });

    let container = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(2)
        .baseline_position(gtk::BaselinePosition::Center)
        // .vexpand(true)
        .valign(Align::Center)
        .build();
    container.append(&button);
    container
}
