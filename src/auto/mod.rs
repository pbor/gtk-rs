// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

mod pixbuf;
pub use self::pixbuf::{Pixbuf, PixbufClass};

mod pixbuf_loader;
pub use self::pixbuf_loader::{PixbufLoader, PixbufLoaderClass, NONE_PIXBUF_LOADER};
pub use self::pixbuf_loader::PixbufLoaderExt;

mod pixbuf_simple_anim;
pub use self::pixbuf_simple_anim::{PixbufSimpleAnim, PixbufSimpleAnimClass};

mod pixbuf_format;
pub use self::pixbuf_format::PixbufFormat;

mod enums;
pub use self::enums::Colorspace;
pub use self::enums::InterpType;
pub use self::enums::PixbufAlphaMode;
pub use self::enums::PixbufError;
pub use self::enums::PixbufRotation;

#[doc(hidden)]
pub mod traits {
    pub use super::PixbufLoaderExt;
}
