// This file was generated by gir (e0b4c3b) from gir-files (11e0e6d)
// DO NOT EDIT

use glib::translate::*;
use glib::types;
use ffi;

use object::*;
use Buildable;
use LevelBarMode;
use Orientable;
use Widget;

pub type LevelBar = Object<ffi::GtkLevelBar>;

unsafe impl Upcast<Widget> for LevelBar { }
unsafe impl Upcast<Buildable> for LevelBar { }
unsafe impl Upcast<Orientable> for LevelBar { }

impl LevelBar {
    #[cfg(gtk_3_6)]
    pub fn new() -> LevelBar {
        unsafe {
            Widget::from_glib_none(ffi::gtk_level_bar_new()).downcast_unchecked()
        }
    }

    #[cfg(gtk_3_6)]
    pub fn new_for_interval(min_value: f64, max_value: f64) -> LevelBar {
        unsafe {
            Widget::from_glib_none(ffi::gtk_level_bar_new_for_interval(min_value, max_value)).downcast_unchecked()
        }
    }

    #[cfg(gtk_3_6)]
    pub fn add_offset_value(&self, name: &str, value: f64) {
        unsafe {
            ffi::gtk_level_bar_add_offset_value(self.to_glib_none().0, name.to_glib_none().0, value);
        }
    }

    #[cfg(gtk_3_8)]
    pub fn get_inverted(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_level_bar_get_inverted(self.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_6)]
    pub fn get_max_value(&self) -> f64 {
        unsafe {
            ffi::gtk_level_bar_get_max_value(self.to_glib_none().0)
        }
    }

    #[cfg(gtk_3_6)]
    pub fn get_min_value(&self) -> f64 {
        unsafe {
            ffi::gtk_level_bar_get_min_value(self.to_glib_none().0)
        }
    }

    #[cfg(gtk_3_6)]
    pub fn get_mode(&self) -> LevelBarMode {
        unsafe {
            ffi::gtk_level_bar_get_mode(self.to_glib_none().0)
        }
    }

    #[cfg(gtk_3_6)]
    pub fn get_offset_value(&self, name: Option<&str>, value: &mut f64) -> bool {
        unsafe {
            from_glib(ffi::gtk_level_bar_get_offset_value(self.to_glib_none().0, name.to_glib_none().0, value))
        }
    }

    #[cfg(gtk_3_6)]
    pub fn get_value(&self) -> f64 {
        unsafe {
            ffi::gtk_level_bar_get_value(self.to_glib_none().0)
        }
    }

    #[cfg(gtk_3_6)]
    pub fn remove_offset_value(&self, name: Option<&str>) {
        unsafe {
            ffi::gtk_level_bar_remove_offset_value(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    #[cfg(gtk_3_8)]
    pub fn set_inverted(&self, inverted: bool) {
        unsafe {
            ffi::gtk_level_bar_set_inverted(self.to_glib_none().0, inverted.to_glib());
        }
    }

    #[cfg(gtk_3_6)]
    pub fn set_max_value(&self, value: f64) {
        unsafe {
            ffi::gtk_level_bar_set_max_value(self.to_glib_none().0, value);
        }
    }

    #[cfg(gtk_3_6)]
    pub fn set_min_value(&self, value: f64) {
        unsafe {
            ffi::gtk_level_bar_set_min_value(self.to_glib_none().0, value);
        }
    }

    #[cfg(gtk_3_6)]
    pub fn set_mode(&self, mode: LevelBarMode) {
        unsafe {
            ffi::gtk_level_bar_set_mode(self.to_glib_none().0, mode);
        }
    }

    #[cfg(gtk_3_6)]
    pub fn set_value(&self, value: f64) {
        unsafe {
            ffi::gtk_level_bar_set_value(self.to_glib_none().0, value);
        }
    }

}

impl types::StaticType for LevelBar {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_level_bar_get_type()) }
    }
}
