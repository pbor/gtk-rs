// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::AsyncResult;
use crate::Cancellable;
use crate::FilterInputStream;
use crate::InputStream;
use crate::Seekable;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    pub struct BufferedInputStream(Object<ffi::GBufferedInputStream, ffi::GBufferedInputStreamClass>) @extends FilterInputStream, InputStream, @implements Seekable;

    match fn {
        get_type => || ffi::g_buffered_input_stream_get_type(),
    }
}

impl BufferedInputStream {
    #[doc(alias = "g_buffered_input_stream_new")]
    pub fn new<P: IsA<InputStream>>(base_stream: &P) -> BufferedInputStream {
        unsafe {
            InputStream::from_glib_full(ffi::g_buffered_input_stream_new(
                base_stream.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "g_buffered_input_stream_new_sized")]
    pub fn new_sized<P: IsA<InputStream>>(base_stream: &P, size: usize) -> BufferedInputStream {
        unsafe {
            InputStream::from_glib_full(ffi::g_buffered_input_stream_new_sized(
                base_stream.as_ref().to_glib_none().0,
                size,
            ))
            .unsafe_cast()
        }
    }
}

#[derive(Clone, Default)]
pub struct BufferedInputStreamBuilder {
    buffer_size: Option<u32>,
    base_stream: Option<InputStream>,
    close_base_stream: Option<bool>,
}

impl BufferedInputStreamBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> BufferedInputStream {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref buffer_size) = self.buffer_size {
            properties.push(("buffer-size", buffer_size));
        }
        if let Some(ref base_stream) = self.base_stream {
            properties.push(("base-stream", base_stream));
        }
        if let Some(ref close_base_stream) = self.close_base_stream {
            properties.push(("close-base-stream", close_base_stream));
        }
        let ret = glib::Object::new::<BufferedInputStream>(&properties).expect("object new");
        ret
    }

    pub fn buffer_size(mut self, buffer_size: u32) -> Self {
        self.buffer_size = Some(buffer_size);
        self
    }

    pub fn base_stream<P: IsA<InputStream>>(mut self, base_stream: &P) -> Self {
        self.base_stream = Some(base_stream.clone().upcast());
        self
    }

    pub fn close_base_stream(mut self, close_base_stream: bool) -> Self {
        self.close_base_stream = Some(close_base_stream);
        self
    }
}

pub const NONE_BUFFERED_INPUT_STREAM: Option<&BufferedInputStream> = None;

pub trait BufferedInputStreamExt: 'static {
    #[doc(alias = "g_buffered_input_stream_fill")]
    fn fill<P: IsA<Cancellable>>(
        &self,
        count: isize,
        cancellable: Option<&P>,
    ) -> Result<isize, glib::Error>;

    #[doc(alias = "g_buffered_input_stream_fill_async")]
    fn fill_async<P: IsA<Cancellable>, Q: FnOnce(Result<isize, glib::Error>) + Send + 'static>(
        &self,
        count: isize,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    );

    fn fill_async_future(
        &self,
        count: isize,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<isize, glib::Error>> + 'static>>;

    #[doc(alias = "g_buffered_input_stream_get_available")]
    fn get_available(&self) -> usize;

    #[doc(alias = "g_buffered_input_stream_get_buffer_size")]
    fn get_buffer_size(&self) -> usize;

    #[doc(alias = "g_buffered_input_stream_peek_buffer")]
    fn peek_buffer(&self) -> Vec<u8>;

    #[doc(alias = "g_buffered_input_stream_read_byte")]
    fn read_byte<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<i32, glib::Error>;

    #[doc(alias = "g_buffered_input_stream_set_buffer_size")]
    fn set_buffer_size(&self, size: usize);

    fn connect_property_buffer_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<BufferedInputStream>> BufferedInputStreamExt for O {
    fn fill<P: IsA<Cancellable>>(
        &self,
        count: isize,
        cancellable: Option<&P>,
    ) -> Result<isize, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_buffered_input_stream_fill(
                self.as_ref().to_glib_none().0,
                count,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn fill_async<P: IsA<Cancellable>, Q: FnOnce(Result<isize, glib::Error>) + Send + 'static>(
        &self,
        count: isize,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn fill_async_trampoline<
            Q: FnOnce(Result<isize, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret =
                ffi::g_buffered_input_stream_fill_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = fill_async_trampoline::<Q>;
        unsafe {
            ffi::g_buffered_input_stream_fill_async(
                self.as_ref().to_glib_none().0,
                count,
                io_priority.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn fill_async_future(
        &self,
        count: isize,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<isize, glib::Error>> + 'static>> {
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.fill_async(count, io_priority, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    fn get_available(&self) -> usize {
        unsafe { ffi::g_buffered_input_stream_get_available(self.as_ref().to_glib_none().0) }
    }

    fn get_buffer_size(&self) -> usize {
        unsafe { ffi::g_buffered_input_stream_get_buffer_size(self.as_ref().to_glib_none().0) }
    }

    fn peek_buffer(&self) -> Vec<u8> {
        unsafe {
            let mut count = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_none_num(
                ffi::g_buffered_input_stream_peek_buffer(
                    self.as_ref().to_glib_none().0,
                    count.as_mut_ptr(),
                ),
                count.assume_init() as usize,
            );
            ret
        }
    }

    fn read_byte<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<i32, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_buffered_input_stream_read_byte(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_buffer_size(&self, size: usize) {
        unsafe {
            ffi::g_buffered_input_stream_set_buffer_size(self.as_ref().to_glib_none().0, size);
        }
    }

    fn connect_property_buffer_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_buffer_size_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GBufferedInputStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<BufferedInputStream>,
        {
            let f: &F = &*(f as *const F);
            f(&BufferedInputStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::buffer-size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_buffer_size_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for BufferedInputStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("BufferedInputStream")
    }
}
