// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::AsyncResult;
use crate::Cancellable;
use glib::object::IsA;
#[cfg(any(feature = "v2_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
use glib::object::ObjectType as ObjectType_;
#[cfg(any(feature = "v2_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v2_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v2_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v2_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
use std::mem::transmute;
use std::ptr;

glib::wrapper! {
    pub struct Task(Object<ffi::GTask, ffi::GTaskClass>) @implements AsyncResult;

    match fn {
        get_type => || ffi::g_task_get_type(),
    }
}

impl Task {
    //#[doc(alias = "g_task_new")]
    //pub fn new<P: IsA<Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(source_object: Option<&glib::Object>, cancellable: Option<&P>, callback: Q) -> Task {
    //    unsafe { TODO: call ffi:g_task_new() }
    //}

    //#[doc(alias = "g_task_attach_source")]
    //pub fn attach_source<P: Fn() -> bool + 'static>(&self, source: &glib::Source, callback: P) {
    //    unsafe { TODO: call ffi:g_task_attach_source() }
    //}

    #[doc(alias = "g_task_get_cancellable")]
    pub fn get_cancellable(&self) -> Option<Cancellable> {
        unsafe { from_glib_none(ffi::g_task_get_cancellable(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_task_get_check_cancellable")]
    pub fn get_check_cancellable(&self) -> bool {
        unsafe { from_glib(ffi::g_task_get_check_cancellable(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
    #[doc(alias = "g_task_get_completed")]
    pub fn get_completed(&self) -> bool {
        unsafe { from_glib(ffi::g_task_get_completed(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_task_get_context")]
    pub fn get_context(&self) -> Option<glib::MainContext> {
        unsafe { from_glib_none(ffi::g_task_get_context(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v2_60", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    #[doc(alias = "g_task_get_name")]
    pub fn get_name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::g_task_get_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_task_get_return_on_cancel")]
    pub fn get_return_on_cancel(&self) -> bool {
        unsafe { from_glib(ffi::g_task_get_return_on_cancel(self.to_glib_none().0)) }
    }

    //#[doc(alias = "g_task_get_source_tag")]
    //pub fn get_source_tag(&self) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call ffi:g_task_get_source_tag() }
    //}

    //#[doc(alias = "g_task_get_task_data")]
    //pub fn get_task_data(&self) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call ffi:g_task_get_task_data() }
    //}

    #[doc(alias = "g_task_had_error")]
    pub fn had_error(&self) -> bool {
        unsafe { from_glib(ffi::g_task_had_error(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_task_propagate_int")]
    pub fn propagate_int(&self) -> Result<isize, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_task_propagate_int(self.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    //#[doc(alias = "g_task_propagate_pointer")]
    //pub fn propagate_pointer(&self) -> Result</*Unimplemented*/Option<Fundamental: Pointer>, glib::Error> {
    //    unsafe { TODO: call ffi:g_task_propagate_pointer() }
    //}

    #[doc(alias = "g_task_return_boolean")]
    pub fn return_boolean(&self, result: bool) {
        unsafe {
            ffi::g_task_return_boolean(self.to_glib_none().0, result.to_glib());
        }
    }

    #[doc(alias = "g_task_return_error_if_cancelled")]
    pub fn return_error_if_cancelled(&self) -> bool {
        unsafe { from_glib(ffi::g_task_return_error_if_cancelled(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_task_return_int")]
    pub fn return_int(&self, result: isize) {
        unsafe {
            ffi::g_task_return_int(self.to_glib_none().0, result);
        }
    }

    //#[doc(alias = "g_task_return_new_error")]
    //pub fn return_new_error(&self, domain: glib::Quark, code: i32, format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:g_task_return_new_error() }
    //}

    //#[doc(alias = "g_task_return_pointer")]
    //pub fn return_pointer(&self, result: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi:g_task_return_pointer() }
    //}

    //#[doc(alias = "g_task_run_in_thread")]
    //pub fn run_in_thread(&self, task_func: /*Unimplemented*/FnOnce(&Task, &glib::Object, /*Unimplemented*/Option<Fundamental: Pointer>, Option<&Cancellable>)) {
    //    unsafe { TODO: call ffi:g_task_run_in_thread() }
    //}

    //#[doc(alias = "g_task_run_in_thread_sync")]
    //pub fn run_in_thread_sync(&self, task_func: /*Unimplemented*/FnOnce(&Task, &glib::Object, /*Unimplemented*/Option<Fundamental: Pointer>, Option<&Cancellable>)) {
    //    unsafe { TODO: call ffi:g_task_run_in_thread_sync() }
    //}

    #[doc(alias = "g_task_set_check_cancellable")]
    pub fn set_check_cancellable(&self, check_cancellable: bool) {
        unsafe {
            ffi::g_task_set_check_cancellable(self.to_glib_none().0, check_cancellable.to_glib());
        }
    }

    #[cfg(any(feature = "v2_60", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    #[doc(alias = "g_task_set_name")]
    pub fn set_name(&self, name: Option<&str>) {
        unsafe {
            ffi::g_task_set_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    #[doc(alias = "g_task_set_return_on_cancel")]
    pub fn set_return_on_cancel(&self, return_on_cancel: bool) -> bool {
        unsafe {
            from_glib(ffi::g_task_set_return_on_cancel(
                self.to_glib_none().0,
                return_on_cancel.to_glib(),
            ))
        }
    }

    //#[doc(alias = "g_task_set_source_tag")]
    //pub fn set_source_tag(&self, source_tag: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi:g_task_set_source_tag() }
    //}

    //#[doc(alias = "g_task_set_task_data")]
    //pub fn set_task_data(&self, task_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi:g_task_set_task_data() }
    //}

    #[doc(alias = "g_task_is_valid")]
    pub fn is_valid<P: IsA<AsyncResult>, Q: IsA<glib::Object>>(
        result: &P,
        source_object: Option<&Q>,
    ) -> bool {
        unsafe {
            from_glib(ffi::g_task_is_valid(
                result.as_ref().to_glib_none().0,
                source_object.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }

    //#[doc(alias = "g_task_report_error")]
    //pub fn report_error<P: FnOnce(Result<(), glib::Error>) + 'static>(source_object: Option<&glib::Object>, callback: P, source_tag: /*Unimplemented*/Option<Fundamental: Pointer>, error: &mut glib::Error) {
    //    unsafe { TODO: call ffi:g_task_report_error() }
    //}

    //#[doc(alias = "g_task_report_new_error")]
    //pub fn report_new_error<P: FnOnce(Result<(), glib::Error>) + 'static>(source_object: Option<&glib::Object>, callback: P, source_tag: /*Unimplemented*/Option<Fundamental: Pointer>, domain: glib::Quark, code: i32, format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:g_task_report_new_error() }
    //}

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
    pub fn connect_property_completed_notify<F: Fn(&Task) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_completed_trampoline<F: Fn(&Task) + 'static>(
            this: *mut ffi::GTask,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::completed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_completed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Task")
    }
}
