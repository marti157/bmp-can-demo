mod imp;

use glib::Object;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::{gio, glib, Application};

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        // Create new window
        Object::builder().property("application", app).build()
    }

    pub fn start_stop_button(&self) -> &gtk::Button {
        &self.imp().start_stop_button
    }

    pub fn temp_label(&self) -> &gtk::Label {
        &self.imp().temp_label
    }

    pub fn pres_label(&self) -> &gtk::Label {
        &self.imp().pres_label
    }

    pub fn alt_label(&self) -> &gtk::Label {
        &self.imp().alt_label
    }

    pub fn status_label(&self) -> &gtk::Label {
        &self.imp().status_label
    }
}
