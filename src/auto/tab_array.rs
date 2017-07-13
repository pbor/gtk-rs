// This file was generated by gir (4b09025) from gir-files (71d73f0)
// DO NOT EDIT

use TabAlign;
use ffi;
use glib::translate::*;
use std::mem;

glib_wrapper! {
    pub struct TabArray(Boxed<ffi::PangoTabArray>);

    match fn {
        copy => |ptr| ffi::pango_tab_array_copy(mut_override(ptr)),
        free => |ptr| ffi::pango_tab_array_free(ptr),
    }
}

impl TabArray {
    pub fn new(initial_size: i32, positions_in_pixels: bool) -> TabArray {
        unsafe {
            from_glib_full(ffi::pango_tab_array_new(initial_size, positions_in_pixels.to_glib()))
        }
    }

    //pub fn new_with_positions(size: i32, positions_in_pixels: bool, first_alignment: TabAlign, first_position: i32, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> TabArray {
    //    unsafe { TODO: call ffi::pango_tab_array_new_with_positions() }
    //}

    pub fn get_positions_in_pixels(&mut self) -> bool {
        unsafe {
            from_glib(ffi::pango_tab_array_get_positions_in_pixels(self.to_glib_none_mut().0))
        }
    }

    pub fn get_size(&mut self) -> i32 {
        unsafe {
            ffi::pango_tab_array_get_size(self.to_glib_none_mut().0)
        }
    }

    pub fn get_tab(&mut self, tab_index: i32) -> (TabAlign, i32) {
        unsafe {
            let mut alignment = mem::uninitialized();
            let mut location = mem::uninitialized();
            ffi::pango_tab_array_get_tab(self.to_glib_none_mut().0, tab_index, &mut alignment, &mut location);
            (from_glib(alignment), location)
        }
    }

    //pub fn get_tabs(&mut self, locations: /*Unimplemented*/CArray TypeId { ns_id: 0, id: 14 }) -> TabAlign {
    //    unsafe { TODO: call ffi::pango_tab_array_get_tabs() }
    //}

    pub fn resize(&mut self, new_size: i32) {
        unsafe {
            ffi::pango_tab_array_resize(self.to_glib_none_mut().0, new_size);
        }
    }

    pub fn set_tab(&mut self, tab_index: i32, alignment: TabAlign, location: i32) {
        unsafe {
            ffi::pango_tab_array_set_tab(self.to_glib_none_mut().0, tab_index, alignment.to_glib(), location);
        }
    }
}
