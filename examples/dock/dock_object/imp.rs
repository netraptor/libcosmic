use crate::utils::BoxedWindowList;
use gio::DesktopAppInfo;
use glib::{ParamFlags, ParamSpec, Value};
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::subclass::prelude::*;
use once_cell::sync::Lazy;
use std::cell::Cell;
use std::cell::RefCell;

// Object holding the state
#[derive(Default)]
pub struct DockObject {
    appinfo: RefCell<Option<DesktopAppInfo>>,
    active: RefCell<BoxedWindowList>,
    saved: Cell<bool>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for DockObject {
    const NAME: &'static str = "DockObject";
    type Type = super::DockObject;
    type ParentType = glib::Object;
}

// Trait shared by all GObjects
impl ObjectImpl for DockObject {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpec::new_object(
                    // Name
                    "appinfo",
                    // Nickname
                    "appinfo",
                    // Short description
                    "app info",
                    DesktopAppInfo::static_type(),
                    // The property can be read and written to
                    ParamFlags::READWRITE,
                ),
                ParamSpec::new_boxed(
                    // Name
                    "active",
                    // Nickname
                    "active",
                    // Short description
                    "active",
                    BoxedWindowList::static_type(),
                    // The property can be read and written to
                    ParamFlags::READWRITE,
                ),
                ParamSpec::new_boolean(
                    "saved",
                    "saved",
                    "Indicates whether app is saved to the dock",
                    false,
                    ParamFlags::READWRITE,
                ),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "appinfo" => {
                let appinfo = value
                    .get()
                    .expect("Value needs to be Option<DesktopAppInfo>");
                self.appinfo.replace(appinfo);
            }
            "active" => {
                let active = value.get().expect("Value needs to be BoxedWindowList");
                self.active.replace(active);
            }
            "saved" => {
                self.saved
                    .replace(value.get().expect("Value needs to be a boolean"));
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "appinfo" => self.appinfo.borrow().to_value(),
            "active" => self.active.borrow().to_value(),
            "saved" => self.saved.get().to_value(),
            _ => unimplemented!(),
        }
    }
}