use {
    audio_module::{AudioModule, Widget},
    freeverb_module::FreeverbModule,
    gtk::{prelude::*, Application, ApplicationWindow, Orientation},
};

mod audio_thread;
mod gtk_parameter_slider;
mod gtk_parameter_toggle;

fn main() {
    run_main::<FreeverbModule>();
}

fn run_main<Module: AudioModule>() {
    if gtk::init().is_err() {
        println!("Error initializing GTK");
        return;
    }

    let (command_sender, command_receiver) = crossbeam_channel::bounded(1024);

    let sample_rate = 44100;
    let _audio_streams = audio_thread::start_audio::<Module>(command_receiver, sample_rate)
        .expect("Failed to start audio");

    let app = Application::builder()
        .application_id("org.example.freeverb-rs")
        .build();

    app.connect_activate(move |app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(350)
            .default_height(300)
            .resizable(false)
            .title("freeverb-rs")
            .build();

        let widgets = gtk::Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(4)
            .height_request(200)
            .vexpand(false)
            .build();

        for id in 0..Module::parameter_count() {
            let parameter = Module::parameter(id);
            let widget = match parameter.widget() {
                Widget::Slider => {
                    gtk_parameter_slider::make_slider(parameter, id, command_sender.clone())
                }
                Widget::Button => {
                    gtk_parameter_toggle::make_toggle(parameter, id, command_sender.clone())
                }
            };
            widgets.append(&widget);
        }

        window.set_child(Some(&widgets));

        window.present();
    });

    app.run();
}
