//! # GTask Sample
//!
//! This sample demonstrates how to implement a GIO async/finish API based on GTask (see
//! https://developer.gnome.org/gio/stable/GTask.html) in Rust using the GTask generated bindings.
//! This can be useful, for example, when porting to Rust some existing C code exposing such an API.

use glib::subclass;
use glib::subclass::prelude::*;
use glib::translate::ToGlibPtr;
use gtk::glib::translate::FromGlibPtrNone;
use gtk::prelude::AsyncResultExt;
use gtk::prelude::Cast;
use gtk::prelude::FileExt;
use gtk::{gio, glib};
use std::convert::TryInto;

pub mod imp {
    use super::*;

    // FileSize is a simple object that will just contain the read file size.
    // Initially the size field will be initialized to 0.
    pub type FileSize = <FileSizePrivate as ObjectSubclass>::Instance;
    pub type FileSizeClass = <FileSizePrivate as ObjectSubclass>::Class;
    pub struct FileSizePrivate {
        pub size: std::cell::RefCell<i64>,
    }

    impl ObjectSubclass for FileSizePrivate {
        const NAME: &'static str = "FileSize";
        type ParentType = glib::Object;
        type Instance = subclass::simple::InstanceStruct<Self>;
        type Class = subclass::simple::ClassStruct<Self>;
        type Type = super::FileSize;
        glib::object_subclass!();

        fn new() -> Self {
            Self {
                size: std::cell::RefCell::new(0),
            }
        }
    }

    impl ObjectImpl for FileSizePrivate {}

    /// # Safety
    ///
    /// This is the ffi method to asynchronously get the file size. It accepts a callback of type
    /// GAsyncReadyCallback, that will be invoked when the async operation completes. Typically,
    /// this callback will invoke the get_file_size_finish method (implemented below) to get the
    /// Task result and perform some operation with it.
    #[no_mangle]
    pub unsafe extern "C" fn get_file_size_async(
        this: *mut FileSize,
        cancellable: *mut gio::ffi::GCancellable,
        callback: gio::ffi::GAsyncReadyCallback,
        user_data: glib::ffi::gpointer,
    ) {
        let cancellable = gio::Cancellable::from_glib_none(cancellable);
        let closure = move |result: &gio::AsyncResult, source_object: Option<&glib::Object>| {
            let result: *mut gio::ffi::GAsyncResult = result.to_glib_none().0;
            let source_object: *mut glib::object::GObject = source_object.to_glib_none().0;
            callback.unwrap()(source_object, result, user_data)
        };

        let source_object = &super::FileSize::from_glib_none(this);
        let task = gio::Task::new(
            Some(&source_object.clone().upcast::<glib::Object>()),
            Some(&cancellable),
            closure,
        );

        glib::MainContext::default().spawn_local(async move {
            let size = gio::File::new_for_path("Cargo.toml")
                .query_info_async_future("*", gio::FileQueryInfoFlags::NONE, glib::PRIORITY_DEFAULT)
                .await
                .unwrap()
                .get_size();

            let source_object = task
                .upcast_ref::<gio::AsyncResult>()
                .get_source_object()
                .unwrap();

            let source_object = FileSizePrivate::from_instance(
                &source_object.downcast_ref::<super::FileSize>().unwrap(),
            );

            source_object.size.replace(size);
            task.return_int(size.try_into().unwrap());
        });
    }

    /// # Safety
    ///
    /// This method will be typically invoked in the callback passed to get_file_size_async in order
    /// to get the propagate the Task result.
    #[no_mangle]
    pub unsafe extern "C" fn get_file_size_finish(
        _this: *mut FileSize,
        result: *mut gio::ffi::GAsyncResult,
        _error: *mut *mut glib::ffi::GError,
    ) -> isize {
        gio::AsyncResult::from_glib_none(result)
            .downcast_ref::<gio::Task>()
            .unwrap()
            .propagate_int()
            .unwrap()
    }

    /// # Safety
    ///
    /// Simple getter
    #[no_mangle]
    pub unsafe extern "C" fn get_retrieved_size(this: *mut FileSize) -> i64 {
        let simple_object = super::FileSize::from_glib_none(this);
        let simple_object = FileSizePrivate::from_instance(
            &simple_object.downcast_ref::<super::FileSize>().unwrap(),
        );
        let x = *simple_object.size.borrow();
        x
    }
}

glib::wrapper! {
    pub struct FileSize(ObjectSubclass<imp::FileSizePrivate>);
}

impl FileSize {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create FileSize")
    }
}

impl Default for FileSize {
    fn default() -> Self {
        FileSize::new()
    }
}

// This function mimicks what the C code using the exported async/finish API would do. It first defines a
// callback of type GAsyncResult that internally calls get_file_size_finish to retrieve the Task
// result, and simply prints it out. Then it invokes the get_file_size_async method, passing the
// callback to it as parameter.
async fn run() {
    let simple_object = FileSize::new();
    let c = gio::Cancellable::new();

    // The callback to be passed to get_file_size_async
    unsafe extern "C" fn c_callback(
        source_object: *mut glib::object::GObject,
        result: *mut gio::ffi::GAsyncResult,
        _user_data: glib::ffi::gpointer,
    ) {
        let mut error = std::ptr::null_mut();
        let ret =
            imp::get_file_size_finish(source_object as *mut imp::FileSize, result, &mut error);
        if !error.is_null() {
            eprintln!("Task returned error!");
            return;
        }

        println!("Returned value from task: {}", ret);
        println!(
            "FileSize::size: {}",
            imp::get_retrieved_size(source_object as *mut imp::FileSize)
        );
    }

    // The actual call to get_file_size_async
    unsafe {
        imp::get_file_size_async(
            simple_object.to_glib_none().0,
            c.to_glib_none().0,
            Some(c_callback),
            std::ptr::null_mut(),
        );
    }
}

fn main() {
    let c = glib::MainContext::default();
    let l = glib::MainLoop::new(Some(&c), false);

    c.push_thread_default();
    c.spawn_local(run());
    l.run();
    c.pop_thread_default();
}
