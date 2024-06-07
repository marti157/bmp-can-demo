mod bmp_can;
mod can;
mod window;

use glib::clone;
use gtk::{gio, glib, prelude::*, Application};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use window::Window;

const APP_ID: &str = "com.marti157.BmpCanDemo";

fn main() -> glib::ExitCode {
    // Register and include resources
    gio::resources_register_include!("app.gresource").expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create window
    let window = Window::new(app);

    // Create a channel for communication between threads
    let (sender, receiver) = async_channel::unbounded::<String>();
    let is_running = Arc::new(AtomicBool::new(false));
    let is_running_clone = is_running.clone();

    // When button is clicked, either stop or run long running BmpCan task
    window
        .start_stop_button()
        .connect_clicked(clone!(@weak window => move |_| {
            let was_running = is_running_clone.load(Ordering::SeqCst);
            is_running_clone.store(!was_running, Ordering::SeqCst);

            if !was_running {
                let sender = sender.clone();
                window.start_stop_button().set_label("Stop");

                // Wasn't running; spawn thread
                gio::spawn_blocking(clone!(@strong is_running_clone => move || {
                    is_running_clone.store(true, Ordering::SeqCst);

                    let mut i = 0;
                    while is_running_clone.load(Ordering::SeqCst) {
                        // Simulate long running task
                        thread::sleep(std::time::Duration::from_secs(1));
                        i += 1;

                        // Send message to the main thread
                        sender
                            .send_blocking(format!("Running... {i}"))
                            .expect("The channel needs to be open.");
                    }
                }));
            } else {
                // Was running, now stopped
                window.start_stop_button().set_label("Start");
            }
        }));

    glib::spawn_future_local(clone!(@weak window => async move {
        while let Ok(text) = receiver.recv().await {
            window.temp_label().set_text(&text);
        }
    }));

    window.present();
}
