// This file was generated by gir (9f70278) from gir-files (0bcaef9)
// DO NOT EDIT

use Cancellable;
use Error;
use ffi;
use glib;
use glib::Value;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Permission(Object<ffi::GPermission>);

    match fn {
        get_type => || ffi::g_permission_get_type(),
    }
}

pub trait PermissionExt {
    fn acquire<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<(), Error>;

    //fn acquire_async<'a, 'b, P: Into<Option<&'a Cancellable>>, Q: Into<Option<&'b /*Unimplemented*/AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, cancellable: P, callback: Q, user_data: R);

    //fn acquire_finish<P: IsA</*Ignored*/AsyncResult>>(&self, result: &P) -> Result<(), Error>;

    fn get_allowed(&self) -> bool;

    fn get_can_acquire(&self) -> bool;

    fn get_can_release(&self) -> bool;

    fn impl_update(&self, allowed: bool, can_acquire: bool, can_release: bool);

    fn release<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<(), Error>;

    //fn release_async<'a, 'b, P: Into<Option<&'a Cancellable>>, Q: Into<Option<&'b /*Unimplemented*/AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, cancellable: P, callback: Q, user_data: R);

    //fn release_finish<P: IsA</*Ignored*/AsyncResult>>(&self, result: &P) -> Result<(), Error>;

    fn get_property_allowed(&self) -> bool;

    fn get_property_can_acquire(&self) -> bool;

    fn get_property_can_release(&self) -> bool;
}

impl<O: IsA<Permission> + IsA<glib::object::Object>> PermissionExt for O {
    fn acquire<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<(), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_permission_acquire(self.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    //fn acquire_async<'a, 'b, P: Into<Option<&'a Cancellable>>, Q: Into<Option<&'b /*Unimplemented*/AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, cancellable: P, callback: Q, user_data: R) {
    //    unsafe { TODO: call ffi::g_permission_acquire_async() }
    //}

    //fn acquire_finish<P: IsA</*Ignored*/AsyncResult>>(&self, result: &P) -> Result<(), Error> {
    //    unsafe { TODO: call ffi::g_permission_acquire_finish() }
    //}

    fn get_allowed(&self) -> bool {
        unsafe {
            from_glib(ffi::g_permission_get_allowed(self.to_glib_none().0))
        }
    }

    fn get_can_acquire(&self) -> bool {
        unsafe {
            from_glib(ffi::g_permission_get_can_acquire(self.to_glib_none().0))
        }
    }

    fn get_can_release(&self) -> bool {
        unsafe {
            from_glib(ffi::g_permission_get_can_release(self.to_glib_none().0))
        }
    }

    fn impl_update(&self, allowed: bool, can_acquire: bool, can_release: bool) {
        unsafe {
            ffi::g_permission_impl_update(self.to_glib_none().0, allowed.to_glib(), can_acquire.to_glib(), can_release.to_glib());
        }
    }

    fn release<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<(), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_permission_release(self.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    //fn release_async<'a, 'b, P: Into<Option<&'a Cancellable>>, Q: Into<Option<&'b /*Unimplemented*/AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, cancellable: P, callback: Q, user_data: R) {
    //    unsafe { TODO: call ffi::g_permission_release_async() }
    //}

    //fn release_finish<P: IsA</*Ignored*/AsyncResult>>(&self, result: &P) -> Result<(), Error> {
    //    unsafe { TODO: call ffi::g_permission_release_finish() }
    //}

    fn get_property_allowed(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "allowed".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_can_acquire(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "can-acquire".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_can_release(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "can-release".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }
}
