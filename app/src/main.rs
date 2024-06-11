mod bmp_can;
mod can;
mod window;

use bmp_can::{BmpCan, SensorData};
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
    let (data_sender, data_receiver) = async_channel::unbounded::<SensorData>();
    let (status_sender, status_receiver) = async_channel::unbounded::<String>();
    let is_running = Arc::new(AtomicBool::new(false));

    // When button is clicked, either stop or run long running BmpCan task
    window
        .start_stop_button()
        .connect_clicked(clone!(@weak window => move |_| {
            let was_running = is_running.load(Ordering::SeqCst);
            is_running.store(!was_running, Ordering::SeqCst);

            if !was_running {
                window.start_stop_button().set_label("Stop");

                // Wasn't running; spawn thread
                gio::spawn_blocking(clone!(@strong is_running, @strong data_sender, @strong status_sender => move || {
                    match BmpCan::new(data_sender) {
                        Ok(mut bmp_can) => {
                            status_sender.send_blocking("Running...".to_string()).unwrap();

                            bmp_can.run(|| {
                                is_running.load(Ordering::SeqCst)
                            });
                        },
                        Err(error) => status_sender.send_blocking(error.to_string()).unwrap()
                    }
                }));
            } else {
                // Was running, now stopped
                window.start_stop_button().set_label("Start");
                status_sender.send_blocking("Ready".to_string()).unwrap();
            }
        }));

    glib::spawn_future_local(clone!(@weak window => async move {
        while let Ok(data) = data_receiver.recv().await {
            window.temp_label().set_text(format!("{} ºC", data.temperature).as_str());
            window.pres_label().set_text(format!("{} Pa", data.pressure).as_str());
            window.alt_label().set_text(format!("{} m", data.altitude).as_str());
        }
    }));

    glib::spawn_future_local(clone!(@weak window => async move {
        while let Ok(status) = status_receiver.recv().await {
            window.status_label().set_text(&status);
        }
    }));

    window.present();
}
