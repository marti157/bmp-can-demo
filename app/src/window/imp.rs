use glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate, Label};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/window.ui")]
pub struct Window {
    #[template_child]
    pub start_stop_button: TemplateChild<Button>,
    #[template_child]
    pub temp_label: TemplateChild<Label>,
    #[template_child]
    pub pres_label: TemplateChild<Label>,
    #[template_child]
    pub alt_label: TemplateChild<Label>,
    #[template_child]
    pub status_label: TemplateChild<Label>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "MainWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for Window {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();
    }
}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}
