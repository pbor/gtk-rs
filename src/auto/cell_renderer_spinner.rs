// This file was generated by gir (3c73dd9) from gir-files (71d73f0)
// DO NOT EDIT

use CellRenderer;
use IconSize;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use gobject_ffi;
use std::mem::transmute;

glib_wrapper! {
    pub struct CellRendererSpinner(Object<ffi::GtkCellRendererSpinner>): CellRenderer;

    match fn {
        get_type => || ffi::gtk_cell_renderer_spinner_get_type(),
    }
}

impl CellRendererSpinner {
    pub fn new() -> CellRendererSpinner {
        assert_initialized_main_thread!();
        unsafe {
            CellRenderer::from_glib_none(ffi::gtk_cell_renderer_spinner_new()).downcast_unchecked()
        }
    }
}

pub trait CellRendererSpinnerExt {
    fn get_property_active(&self) -> bool;

    fn set_property_active(&self, active: bool);

    fn get_property_pulse(&self) -> u32;

    fn set_property_pulse(&self, pulse: u32);

    fn get_property_size(&self) -> IconSize;

    fn set_property_size(&self, size: IconSize);
}

impl<O: IsA<CellRendererSpinner> + IsA<glib::object::Object>> CellRendererSpinnerExt for O {
    fn get_property_active(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "active".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_active(&self, active: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "active".to_glib_none().0, Value::from(&active).to_glib_none().0);
        }
    }

    fn get_property_pulse(&self) -> u32 {
        let mut value = Value::from(&0u32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "pulse".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_pulse(&self, pulse: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "pulse".to_glib_none().0, Value::from(&pulse).to_glib_none().0);
        }
    }

    fn get_property_size(&self) -> IconSize {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "size".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<i32>().unwrap()))
        }
    }

    fn set_property_size(&self, size: IconSize) {
        let size = size.to_glib() as i32;
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "size".to_glib_none().0, Value::from(&size).to_glib_none().0);
        }
    }
}
