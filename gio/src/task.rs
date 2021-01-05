// Take a look at the license at the top of the repository in the LICENSE file.

use crate::AsyncResult;
use crate::Cancellable;
use crate::Task;
use glib::object::IsA;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::ptr;

impl Task {
    pub fn new<P: IsA<Cancellable>, Q: FnOnce(&AsyncResult, Option<&glib::Object>) + 'static>(
        source_object: Option<&glib::Object>,
        cancellable: Option<&P>,
        callback: Q,
    ) -> Task {
        let callback_data = Box_::new(callback);
        unsafe extern "C" fn trampoline<
            Q: FnOnce(&AsyncResult, Option<&glib::Object>) + 'static,
        >(
            source_object: *mut glib::gobject_ffi::GObject,
            res: *mut ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let source_object = Option::<glib::Object>::from_glib_borrow(source_object);
            let res = AsyncResult::from_glib_borrow(res);
            let callback: Box_<Q> = Box::from_raw(user_data as *mut _);
            callback(&res, source_object.as_ref().as_ref());
        };
        let callback = trampoline::<Q>;
        unsafe {
            from_glib_full(ffi::g_task_new(
                source_object.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(callback_data) as *mut _,
            ))
        }
    }

    pub fn return_error(&self, error: glib::Error) {
        unsafe {
            ffi::g_task_return_error(self.to_glib_none().0, error.to_glib_full() as *mut _);
        }
    }

    pub fn get_priority(&self) -> glib::source::Priority {
        unsafe { FromGlib::from_glib(ffi::g_task_get_priority(self.to_glib_none().0)) }
    }

    pub fn set_priority(&self, priority: glib::source::Priority) {
        unsafe {
            ffi::g_task_set_priority(self.to_glib_none().0, priority.to_glib());
        }
    }

    pub fn propagate_boolean(&self) -> Result<bool, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let result = ffi::g_task_propagate_boolean(self.to_glib_none().0, &mut error) != 0;
            if error.is_null() {
                Ok(result)
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_util::run_async;
    use crate::CancellableExt;
    use glib::Cast;

    #[test]
    fn test_int_async_result() {
        match run_async(|tx, l| {
            let c = crate::Cancellable::new();
            let t = crate::Task::new(
                None,
                Some(&c),
                move |a: &AsyncResult, _b: Option<&glib::Object>| {
                    let t = a.downcast_ref::<crate::Task>().unwrap();
                    tx.send(t.propagate_int()).unwrap();
                    l.quit();
                },
            );
            t.return_int(100);
        }) {
            Err(_) => panic!(),
            Ok(i) => assert!(i == 100),
        }
    }

    #[test]
    fn test_error() {
        match run_async(|tx, l| {
            let c = crate::Cancellable::new();
            let t = crate::Task::new(
                None,
                Some(&c),
                move |a: &AsyncResult, _b: Option<&glib::Object>| {
                    let t = a.downcast_ref::<crate::Task>().unwrap();
                    tx.send(t.propagate_int()).unwrap();
                    l.quit();
                },
            );
            t.return_error(glib::Error::new(
                crate::IOErrorEnum::WouldBlock,
                "WouldBlock",
            ));
        }) {
            Err(e) => match e.kind().unwrap() {
                crate::IOErrorEnum::WouldBlock => {}
                _ => panic!(),
            },
            Ok(_) => panic!(),
        }
    }

    #[test]
    fn test_cancelled() {
        match run_async(|tx, l| {
            let c = crate::Cancellable::new();
            let t = crate::Task::new(
                None,
                Some(&c),
                move |a: &AsyncResult, _b: Option<&glib::Object>| {
                    let t = a.downcast_ref::<crate::Task>().unwrap();
                    tx.send(t.propagate_int()).unwrap();
                    l.quit();
                },
            );
            c.cancel();
            t.return_error_if_cancelled();
        }) {
            Err(e) => match e.kind().unwrap() {
                crate::IOErrorEnum::Cancelled => {}
                _ => panic!(),
            },
            Ok(_) => panic!(),
        }
    }
}
