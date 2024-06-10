mod bmp_can;
mod can;
mod window;

use bmp_can::BmpCan;
use glib::clone;
use gtk::{gio, glib, prelude::*, Application};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
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
                    if let Ok(mut bmp_can) = BmpCan::new(sender) {
                        bmp_can.run(|| {
                            is_running_clone.load(Ordering::SeqCst)
                        });
                    }
                }));
            } else {
                // Was running, now stopped
                window.start_stop_button().set_label("Start");
            }
        }));

    glib::spawn_future_local(clone!(@weak window => async move {
        while let Ok(data) = receiver.recv().await {
            window.temp_label().set_text(format!("Data: {}", &data).as_str() );
        }
    }));

    window.present();
}
