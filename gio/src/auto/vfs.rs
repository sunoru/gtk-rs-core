// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::File;
use glib::object::IsA;
use glib::translate::*;
#[cfg(any(feature = "v2_50", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
use std::boxed::Box as Box_;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GVfs")]
    pub struct Vfs(Object<ffi::GVfs, ffi::GVfsClass>);

    match fn {
        type_ => || ffi::g_vfs_get_type(),
    }
}

impl Vfs {
    #[doc(alias = "g_vfs_get_default")]
    #[doc(alias = "get_default")]
    pub fn default() -> Vfs {
        unsafe { from_glib_none(ffi::g_vfs_get_default()) }
    }

    #[doc(alias = "g_vfs_get_local")]
    #[doc(alias = "get_local")]
    pub fn local() -> Vfs {
        unsafe { from_glib_none(ffi::g_vfs_get_local()) }
    }
}

unsafe impl Send for Vfs {}
unsafe impl Sync for Vfs {}

impl Vfs {
    pub const NONE: Option<&'static Vfs> = None;
}

pub trait VfsExt: 'static {
    #[doc(alias = "g_vfs_get_file_for_path")]
    #[doc(alias = "get_file_for_path")]
    fn file_for_path(&self, path: &str) -> File;

    #[doc(alias = "g_vfs_get_file_for_uri")]
    #[doc(alias = "get_file_for_uri")]
    fn file_for_uri(&self, uri: &str) -> File;

    #[doc(alias = "g_vfs_get_supported_uri_schemes")]
    #[doc(alias = "get_supported_uri_schemes")]
    fn supported_uri_schemes(&self) -> Vec<glib::GString>;

    #[doc(alias = "g_vfs_is_active")]
    fn is_active(&self) -> bool;

    #[doc(alias = "g_vfs_parse_name")]
    fn parse_name(&self, parse_name: &str) -> File;

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
    #[doc(alias = "g_vfs_register_uri_scheme")]
    fn register_uri_scheme(
        &self,
        scheme: &str,
        uri_func: Option<Box_<dyn Fn(&Vfs, &str) -> File + 'static>>,
        parse_name_func: Option<Box_<dyn Fn(&Vfs, &str) -> File + 'static>>,
    ) -> bool;

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
    #[doc(alias = "g_vfs_unregister_uri_scheme")]
    fn unregister_uri_scheme(&self, scheme: &str) -> bool;
}

impl<O: IsA<Vfs>> VfsExt for O {
    fn file_for_path(&self, path: &str) -> File {
        unsafe {
            from_glib_full(ffi::g_vfs_get_file_for_path(
                self.as_ref().to_glib_none().0,
                path.to_glib_none().0,
            ))
        }
    }

    fn file_for_uri(&self, uri: &str) -> File {
        unsafe {
            from_glib_full(ffi::g_vfs_get_file_for_uri(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
            ))
        }
    }

    fn supported_uri_schemes(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::g_vfs_get_supported_uri_schemes(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_active(&self) -> bool {
        unsafe { from_glib(ffi::g_vfs_is_active(self.as_ref().to_glib_none().0)) }
    }

    fn parse_name(&self, parse_name: &str) -> File {
        unsafe {
            from_glib_full(ffi::g_vfs_parse_name(
                self.as_ref().to_glib_none().0,
                parse_name.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
    fn register_uri_scheme(
        &self,
        scheme: &str,
        uri_func: Option<Box_<dyn Fn(&Vfs, &str) -> File + 'static>>,
        parse_name_func: Option<Box_<dyn Fn(&Vfs, &str) -> File + 'static>>,
    ) -> bool {
        let uri_func_data: Box_<Option<Box_<dyn Fn(&Vfs, &str) -> File + 'static>>> =
            Box_::new(uri_func);
        unsafe extern "C" fn uri_func_func(
            vfs: *mut ffi::GVfs,
            identifier: *const libc::c_char,
            user_data: glib::ffi::gpointer,
        ) -> *mut ffi::GFile {
            let vfs = from_glib_borrow(vfs);
            let identifier: Borrowed<glib::GString> = from_glib_borrow(identifier);
            let callback: &Option<Box_<dyn Fn(&Vfs, &str) -> File + 'static>> =
                &*(user_data as *mut _);
            let res = if let Some(ref callback) = *callback {
                callback(&vfs, identifier.as_str())
            } else {
                panic!("cannot get closure...")
            };
            res.to_glib_full()
        }
        let uri_func = if uri_func_data.is_some() {
            Some(uri_func_func as _)
        } else {
            None
        };
        let parse_name_func_data: Box_<Option<Box_<dyn Fn(&Vfs, &str) -> File + 'static>>> =
            Box_::new(parse_name_func);
        unsafe extern "C" fn parse_name_func_func(
            vfs: *mut ffi::GVfs,
            identifier: *const libc::c_char,
            user_data: glib::ffi::gpointer,
        ) -> *mut ffi::GFile {
            let vfs = from_glib_borrow(vfs);
            let identifier: Borrowed<glib::GString> = from_glib_borrow(identifier);
            let callback: &Option<Box_<dyn Fn(&Vfs, &str) -> File + 'static>> =
                &*(user_data as *mut _);
            let res = if let Some(ref callback) = *callback {
                callback(&vfs, identifier.as_str())
            } else {
                panic!("cannot get closure...")
            };
            res.to_glib_full()
        }
        let parse_name_func = if parse_name_func_data.is_some() {
            Some(parse_name_func_func as _)
        } else {
            None
        };
        unsafe extern "C" fn uri_destroy_func(data: glib::ffi::gpointer) {
            let _callback: Box_<Option<Box_<dyn Fn(&Vfs, &str) -> File + 'static>>> =
                Box_::from_raw(data as *mut _);
        }
        let destroy_call4 = Some(uri_destroy_func as _);
        unsafe extern "C" fn parse_name_destroy_func(data: glib::ffi::gpointer) {
            let _callback: Box_<Option<Box_<dyn Fn(&Vfs, &str) -> File + 'static>>> =
                Box_::from_raw(data as *mut _);
        }
        let destroy_call7 = Some(parse_name_destroy_func as _);
        let super_callback0: Box_<Option<Box_<dyn Fn(&Vfs, &str) -> File + 'static>>> =
            uri_func_data;
        let super_callback1: Box_<Option<Box_<dyn Fn(&Vfs, &str) -> File + 'static>>> =
            parse_name_func_data;
        unsafe {
            from_glib(ffi::g_vfs_register_uri_scheme(
                self.as_ref().to_glib_none().0,
                scheme.to_glib_none().0,
                uri_func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call4,
                parse_name_func,
                Box_::into_raw(super_callback1) as *mut _,
                destroy_call7,
            ))
        }
    }

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
    fn unregister_uri_scheme(&self, scheme: &str) -> bool {
        unsafe {
            from_glib(ffi::g_vfs_unregister_uri_scheme(
                self.as_ref().to_glib_none().0,
                scheme.to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for Vfs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Vfs")
    }
}
