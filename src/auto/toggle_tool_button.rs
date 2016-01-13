// This file was generated by gir (34ea1b9) from gir-files (11e0e6d)
// DO NOT EDIT

use Actionable;
use Bin;
use Buildable;
use Container;
use ToolButton;
use ToolItem;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct ToggleToolButton(Object<ffi::GtkToggleToolButton>): Widget, Container, Bin, ToolItem, ToolButton, Actionable, Buildable;

    match fn {
        get_type => || ffi::gtk_toggle_tool_button_get_type(),
    }
}

impl ToggleToolButton {
    pub fn new() -> ToggleToolButton {
        assert_initialized_main_thread!();
        unsafe {
            ToolItem::from_glib_none(ffi::gtk_toggle_tool_button_new()).downcast_unchecked()
        }
    }

    pub fn new_from_stock(stock_id: &str) -> ToggleToolButton {
        assert_initialized_main_thread!();
        unsafe {
            ToolItem::from_glib_none(ffi::gtk_toggle_tool_button_new_from_stock(stock_id.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn get_active(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_toggle_tool_button_get_active(self.to_glib_none().0))
        }
    }

    pub fn set_active(&self, is_active: bool) {
        unsafe {
            ffi::gtk_toggle_tool_button_set_active(self.to_glib_none().0, is_active.to_glib());
        }
    }
}
