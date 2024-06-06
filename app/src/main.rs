mod window;

use std::thread;
use std::time::Duration;

use glib::clone;
use gtk::prelude::*;
use gtk::{gio, glib, Application};
use window::Window;

const APP_ID: &str = "com.marti157.BmpCanDemo";

fn main() -> glib::ExitCode {
    // Register and include resources
    gio::resources_register_include!("app.gresource").expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create window
    let window = Window::new(app);

    // Create channel that can hold at most 1 message at a time
    let (sender, receiver) = async_channel::bounded(1);

    window.button().connect_clicked(move |_| {
        let sender = sender.clone();
        // The long running operation runs now in a separate thread
        gio::spawn_blocking(move || {
            // Deactivate the button until the operation is done
            sender
                .send_blocking(false)
                .expect("The channel needs to be open.");
            let five_seconds = Duration::from_secs(5);
            thread::sleep(five_seconds);
            // Activate the button again
            sender
                .send_blocking(true)
                .expect("The channel needs to be open.");
        });
    });

    // The main loop executes the asynchronous block
    glib::spawn_future_local(clone!(@weak window => async move {
        while let Ok(enable_button) = receiver.recv().await {
            window.button().set_sensitive(enable_button);
        }
    }));

    window.present();
}
