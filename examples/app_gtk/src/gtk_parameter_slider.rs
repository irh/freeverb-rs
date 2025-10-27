use {
    audio_module::{Command, Parameter},
    gtk::prelude::*,
    gtk::{Label, Orientation, PositionType, Scale},
};

pub fn make_slider(
    parameter: Box<dyn Parameter>,
    id: usize,
    command_sender: crossbeam_channel::Sender<Command>,
) -> gtk::Box {
    let container = gtk::Box::new(Orientation::Vertical, 2);

    let label = Label::new(Some(parameter.name().as_str()));
    container.pack_start(&label, false, false, 0);

    let scale = Scale::with_range(Orientation::Vertical, 0.0, 1.0, 0.01);
    scale.set_inverted(true);
    scale.set_value_pos(PositionType::Bottom);

    let value_converter = parameter.make_value_converter();
    let string_converter = parameter.make_string_converter();

    scale.set_value(value_converter.user_to_linear(parameter.default_user_value()) as f64);
    scale.connect_format_value({
        move |_, x| string_converter.to_string(value_converter.linear_to_user(x as f32))
    });

    container.pack_start(&scale, true, true, 0);

    scale.connect_value_changed(move |scale| {
        command_sender
            .send(Command::SetParameter(id, scale.value() as f32))
            .unwrap();
    });

    container
}
