use {
    audio_module::{Command, Parameter},
    gtk::{prelude::*, Label, Orientation, PositionType, Scale},
};

pub fn make_slider(
    parameter: Box<dyn Parameter>,
    id: usize,
    command_sender: crossbeam_channel::Sender<Command>,
) -> gtk::Box {
    let value_converter = parameter.make_value_converter();
    let string_converter = parameter.make_string_converter();

    let label = Label::new(Some(parameter.name().as_str()));

    let adjustment = gtk::Adjustment::builder()
        .lower(0.0)
        .upper(1.0)
        .value(value_converter.user_to_linear(parameter.default_user_value()) as f64)
        .step_increment(0.01)
        .page_increment(0.1)
        .build();

    let scale = Scale::builder()
        .adjustment(&adjustment)
        .inverted(true)
        .value_pos(PositionType::Bottom)
        .orientation(Orientation::Vertical)
        .draw_value(true)
        .digits(2)
        .vexpand(true)
        .hexpand(true)
        .build();

    scale.set_increments(0.01, 0.001);
    scale.set_format_value_func(move |_, x| {
        string_converter.to_string(value_converter.linear_to_user(x as f32))
    });

    scale.connect_value_changed(move |scale| {
        command_sender
            .send(Command::SetParameter(id, scale.value() as f32))
            .unwrap();
    });

    let container = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(2)
        .build();

    container.append(&label);
    container.append(&scale);

    container
}
