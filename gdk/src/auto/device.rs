// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Atom;
#[cfg(any(feature = "v3_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
use crate::AxisFlags;
use crate::AxisUse;
use crate::Cursor;
use crate::DeviceManager;
#[cfg(any(feature = "v3_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
use crate::DeviceTool;
use crate::DeviceType;
use crate::Display;
use crate::EventMask;
use crate::GrabOwnership;
use crate::GrabStatus;
use crate::InputMode;
use crate::InputSource;
use crate::ModifierType;
use crate::Screen;
#[cfg(any(feature = "v3_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
use crate::Seat;
use crate::Window;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib::wrapper! {
    pub struct Device(Object<ffi::GdkDevice>);

    match fn {
        type_ => || ffi::gdk_device_get_type(),
    }
}

impl Device {
    #[doc(alias = "gdk_device_get_associated_device")]
    pub fn associated_device(&self) -> Option<Device> {
        unsafe { from_glib_none(ffi::gdk_device_get_associated_device(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    #[doc(alias = "gdk_device_get_axes")]
    pub fn axes(&self) -> AxisFlags {
        unsafe { from_glib(ffi::gdk_device_get_axes(self.to_glib_none().0)) }
    }

    //#[doc(alias = "gdk_device_get_axis")]
    //pub fn axis(&self, axes: &[f64], use_: AxisUse) -> Option<f64> {
    //    unsafe { TODO: call ffi:gdk_device_get_axis() }
    //}

    #[doc(alias = "gdk_device_get_axis_use")]
    pub fn axis_use(&self, index_: u32) -> AxisUse {
        unsafe { from_glib(ffi::gdk_device_get_axis_use(self.to_glib_none().0, index_)) }
    }

    //#[doc(alias = "gdk_device_get_axis_value")]
    //pub fn axis_value(&self, axes: &[f64], axis_label: &Atom) -> Option<f64> {
    //    unsafe { TODO: call ffi:gdk_device_get_axis_value() }
    //}

    #[doc(alias = "gdk_device_get_device_type")]
    pub fn device_type(&self) -> DeviceType {
        unsafe { from_glib(ffi::gdk_device_get_device_type(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_device_get_display")]
    pub fn display(&self) -> Display {
        unsafe { from_glib_none(ffi::gdk_device_get_display(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_device_get_has_cursor")]
    pub fn has_cursor(&self) -> bool {
        unsafe { from_glib(ffi::gdk_device_get_has_cursor(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_device_get_key")]
    pub fn key(&self, index_: u32) -> Option<(u32, ModifierType)> {
        unsafe {
            let mut keyval = mem::MaybeUninit::uninit();
            let mut modifiers = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gdk_device_get_key(
                self.to_glib_none().0,
                index_,
                keyval.as_mut_ptr(),
                modifiers.as_mut_ptr(),
            ));
            let keyval = keyval.assume_init();
            let modifiers = modifiers.assume_init();
            if ret {
                Some((keyval, from_glib(modifiers)))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gdk_device_get_last_event_window")]
    pub fn last_event_window(&self) -> Option<Window> {
        unsafe { from_glib_none(ffi::gdk_device_get_last_event_window(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_device_get_mode")]
    pub fn mode(&self) -> InputMode {
        unsafe { from_glib(ffi::gdk_device_get_mode(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_device_get_n_axes")]
    pub fn n_axes(&self) -> i32 {
        unsafe { ffi::gdk_device_get_n_axes(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_device_get_n_keys")]
    pub fn n_keys(&self) -> i32 {
        unsafe { ffi::gdk_device_get_n_keys(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_device_get_name")]
    pub fn name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gdk_device_get_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_device_get_position")]
    pub fn position(&self) -> (Screen, i32, i32) {
        unsafe {
            let mut screen = ptr::null_mut();
            let mut x = mem::MaybeUninit::uninit();
            let mut y = mem::MaybeUninit::uninit();
            ffi::gdk_device_get_position(
                self.to_glib_none().0,
                &mut screen,
                x.as_mut_ptr(),
                y.as_mut_ptr(),
            );
            let x = x.assume_init();
            let y = y.assume_init();
            (from_glib_none(screen), x, y)
        }
    }

    #[doc(alias = "gdk_device_get_position_double")]
    pub fn position_double(&self) -> (Screen, f64, f64) {
        unsafe {
            let mut screen = ptr::null_mut();
            let mut x = mem::MaybeUninit::uninit();
            let mut y = mem::MaybeUninit::uninit();
            ffi::gdk_device_get_position_double(
                self.to_glib_none().0,
                &mut screen,
                x.as_mut_ptr(),
                y.as_mut_ptr(),
            );
            let x = x.assume_init();
            let y = y.assume_init();
            (from_glib_none(screen), x, y)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_16")))]
    #[doc(alias = "gdk_device_get_product_id")]
    pub fn product_id(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gdk_device_get_product_id(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
    #[doc(alias = "gdk_device_get_seat")]
    pub fn seat(&self) -> Option<Seat> {
        unsafe { from_glib_none(ffi::gdk_device_get_seat(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_device_get_source")]
    pub fn source(&self) -> InputSource {
        unsafe { from_glib(ffi::gdk_device_get_source(self.to_glib_none().0)) }
    }

    //#[doc(alias = "gdk_device_get_state")]
    //pub fn state(&self, window: &Window, axes: &[f64]) -> ModifierType {
    //    unsafe { TODO: call ffi:gdk_device_get_state() }
    //}

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_16")))]
    #[doc(alias = "gdk_device_get_vendor_id")]
    pub fn vendor_id(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gdk_device_get_vendor_id(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_device_get_window_at_position")]
    pub fn window_at_position(&self) -> (Option<Window>, i32, i32) {
        unsafe {
            let mut win_x = mem::MaybeUninit::uninit();
            let mut win_y = mem::MaybeUninit::uninit();
            let ret = from_glib_none(ffi::gdk_device_get_window_at_position(
                self.to_glib_none().0,
                win_x.as_mut_ptr(),
                win_y.as_mut_ptr(),
            ));
            let win_x = win_x.assume_init();
            let win_y = win_y.assume_init();
            (ret, win_x, win_y)
        }
    }

    #[doc(alias = "gdk_device_get_window_at_position_double")]
    pub fn window_at_position_double(&self) -> (Option<Window>, f64, f64) {
        unsafe {
            let mut win_x = mem::MaybeUninit::uninit();
            let mut win_y = mem::MaybeUninit::uninit();
            let ret = from_glib_none(ffi::gdk_device_get_window_at_position_double(
                self.to_glib_none().0,
                win_x.as_mut_ptr(),
                win_y.as_mut_ptr(),
            ));
            let win_x = win_x.assume_init();
            let win_y = win_y.assume_init();
            (ret, win_x, win_y)
        }
    }

    #[cfg_attr(feature = "v3_20", deprecated = "Since 3.20")]
    #[doc(alias = "gdk_device_grab")]
    pub fn grab(
        &self,
        window: &Window,
        grab_ownership: GrabOwnership,
        owner_events: bool,
        event_mask: EventMask,
        cursor: Option<&Cursor>,
        time_: u32,
    ) -> GrabStatus {
        unsafe {
            from_glib(ffi::gdk_device_grab(
                self.to_glib_none().0,
                window.to_glib_none().0,
                grab_ownership.to_glib(),
                owner_events.to_glib(),
                event_mask.to_glib(),
                cursor.to_glib_none().0,
                time_,
            ))
        }
    }

    #[doc(alias = "gdk_device_list_axes")]
    pub fn list_axes(&self) -> Vec<Atom> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gdk_device_list_axes(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_device_list_slave_devices")]
    pub fn list_slave_devices(&self) -> Vec<Device> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gdk_device_list_slave_devices(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_device_set_axis_use")]
    pub fn set_axis_use(&self, index_: u32, use_: AxisUse) {
        unsafe {
            ffi::gdk_device_set_axis_use(self.to_glib_none().0, index_, use_.to_glib());
        }
    }

    #[doc(alias = "gdk_device_set_key")]
    pub fn set_key(&self, index_: u32, keyval: u32, modifiers: ModifierType) {
        unsafe {
            ffi::gdk_device_set_key(self.to_glib_none().0, index_, keyval, modifiers.to_glib());
        }
    }

    #[doc(alias = "gdk_device_set_mode")]
    pub fn set_mode(&self, mode: InputMode) -> bool {
        unsafe {
            from_glib(ffi::gdk_device_set_mode(
                self.to_glib_none().0,
                mode.to_glib(),
            ))
        }
    }

    #[cfg_attr(feature = "v3_20", deprecated = "Since 3.20")]
    #[doc(alias = "gdk_device_ungrab")]
    pub fn ungrab(&self, time_: u32) {
        unsafe {
            ffi::gdk_device_ungrab(self.to_glib_none().0, time_);
        }
    }

    #[doc(alias = "gdk_device_warp")]
    pub fn warp(&self, screen: &Screen, x: i32, y: i32) {
        unsafe {
            ffi::gdk_device_warp(self.to_glib_none().0, screen.to_glib_none().0, x, y);
        }
    }

    #[doc(alias = "get_property_device_manager")]
    pub fn device_manager(&self) -> Option<DeviceManager> {
        unsafe {
            let mut value = glib::Value::from_type(<DeviceManager as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"device-manager\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `device-manager` getter")
        }
    }

    #[doc(alias = "get_property_input_mode")]
    pub fn input_mode(&self) -> InputMode {
        unsafe {
            let mut value = glib::Value::from_type(<InputMode as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"input-mode\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `input-mode` getter")
        }
    }

    #[doc(alias = "set_property_input_mode")]
    pub fn set_input_mode(&self, input_mode: InputMode) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"input-mode\0".as_ptr() as *const _,
                input_mode.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "get_property_input_source")]
    pub fn input_source(&self) -> InputSource {
        unsafe {
            let mut value = glib::Value::from_type(<InputSource as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"input-source\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `input-source` getter")
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
    #[doc(alias = "get_property_num_touches")]
    pub fn num_touches(&self) -> u32 {
        unsafe {
            let mut value = glib::Value::from_type(<u32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"num-touches\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `num-touches` getter")
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
    #[doc(alias = "set_property_seat")]
    pub fn set_seat(&self, seat: Option<&Seat>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"seat\0".as_ptr() as *const _,
                seat.to_value().to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    #[doc(alias = "get_property_tool")]
    pub fn tool(&self) -> Option<DeviceTool> {
        unsafe {
            let mut value = glib::Value::from_type(<DeviceTool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"tool\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `tool` getter")
        }
    }

    #[doc(alias = "get_property_type")]
    pub fn type_(&self) -> DeviceType {
        unsafe {
            let mut value = glib::Value::from_type(<DeviceType as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"type\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `type` getter")
        }
    }

    #[cfg_attr(feature = "v3_16", deprecated = "Since 3.16")]
    #[doc(alias = "gdk_device_grab_info_libgtk_only")]
    pub fn grab_info_libgtk_only(display: &Display, device: &Device) -> Option<(Window, bool)> {
        skip_assert_initialized!();
        unsafe {
            let mut grab_window = ptr::null_mut();
            let mut owner_events = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gdk_device_grab_info_libgtk_only(
                display.to_glib_none().0,
                device.to_glib_none().0,
                &mut grab_window,
                owner_events.as_mut_ptr(),
            ));
            let owner_events = owner_events.assume_init();
            if ret {
                Some((from_glib_none(grab_window), from_glib(owner_events)))
            } else {
                None
            }
        }
    }

    pub fn connect_changed<F: Fn(&Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<F: Fn(&Device) + 'static>(
            this: *mut ffi::GdkDevice,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    pub fn connect_tool_changed<F: Fn(&Device, &DeviceTool) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn tool_changed_trampoline<F: Fn(&Device, &DeviceTool) + 'static>(
            this: *mut ffi::GdkDevice,
            tool: *mut ffi::GdkDeviceTool,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(tool))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"tool-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    tool_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_associated_device_notify<F: Fn(&Device) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_associated_device_trampoline<F: Fn(&Device) + 'static>(
            this: *mut ffi::GdkDevice,
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
                b"notify::associated-device\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_associated_device_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    pub fn connect_property_axes_notify<F: Fn(&Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_axes_trampoline<F: Fn(&Device) + 'static>(
            this: *mut ffi::GdkDevice,
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
                b"notify::axes\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_axes_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_input_mode_notify<F: Fn(&Device) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_input_mode_trampoline<F: Fn(&Device) + 'static>(
            this: *mut ffi::GdkDevice,
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
                b"notify::input-mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_input_mode_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_n_axes_notify<F: Fn(&Device) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_n_axes_trampoline<F: Fn(&Device) + 'static>(
            this: *mut ffi::GdkDevice,
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
                b"notify::n-axes\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_n_axes_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
    pub fn connect_property_seat_notify<F: Fn(&Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_seat_trampoline<F: Fn(&Device) + 'static>(
            this: *mut ffi::GdkDevice,
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
                b"notify::seat\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_seat_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    pub fn connect_property_tool_notify<F: Fn(&Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tool_trampoline<F: Fn(&Device) + 'static>(
            this: *mut ffi::GdkDevice,
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
                b"notify::tool\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_tool_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_type_notify<F: Fn(&Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_type_trampoline<F: Fn(&Device) + 'static>(
            this: *mut ffi::GdkDevice,
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
                b"notify::type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_type_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Device")
    }
}
