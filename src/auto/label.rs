// This file was generated by gir (e0b4c3b) from gir-files (11e0e6d)
// DO NOT EDIT

use glib::translate::*;
use glib::types;
use ffi;

use object::*;
use Buildable;
use Justification;
use Misc;
use Widget;

pub type Label = Object<ffi::GtkLabel>;

unsafe impl Upcast<Widget> for Label { }
unsafe impl Upcast<Misc> for Label { }
unsafe impl Upcast<Buildable> for Label { }

impl Label {
    pub fn new(str: Option<&str>) -> Label {
        unsafe {
            Widget::from_glib_none(ffi::gtk_label_new(str.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_mnemonic(str: Option<&str>) -> Label {
        unsafe {
            Widget::from_glib_none(ffi::gtk_label_new_with_mnemonic(str.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn get_angle(&self) -> f64 {
        unsafe {
            ffi::gtk_label_get_angle(self.to_glib_none().0)
        }
    }

    //pub fn get_attributes(&self) -> /*Unknown kind*/Unknown rust type: "AttrList" {
    //    unsafe { TODO: call ffi:gtk_label_get_attributes() }
    //}

    pub fn get_current_uri(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_label_get_current_uri(self.to_glib_none().0))
        }
    }

    //pub fn get_ellipsize(&self) -> pango::EllipsizeMode {
    //    unsafe { TODO: call ffi:gtk_label_get_ellipsize() }
    //}

    pub fn get_justify(&self) -> Justification {
        unsafe {
            ffi::gtk_label_get_justify(self.to_glib_none().0)
        }
    }

    pub fn get_label(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_label_get_label(self.to_glib_none().0))
        }
    }

    //pub fn get_layout(&self) -> Option<pango::Layout> {
    //    unsafe { TODO: call ffi:gtk_label_get_layout() }
    //}

    pub fn get_layout_offsets(&self) -> (i32, i32) {
        let mut x = Default::default();
        let mut y = Default::default();
        unsafe {
            ffi::gtk_label_get_layout_offsets(self.to_glib_none().0, &mut x, &mut y);
        }
        (x, y)
    }

    pub fn get_line_wrap(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_label_get_line_wrap(self.to_glib_none().0))
        }
    }

    //pub fn get_line_wrap_mode(&self) -> pango::WrapMode {
    //    unsafe { TODO: call ffi:gtk_label_get_line_wrap_mode() }
    //}

    #[cfg(gtk_3_10)]
    pub fn get_lines(&self) -> i32 {
        unsafe {
            ffi::gtk_label_get_lines(self.to_glib_none().0)
        }
    }

    pub fn get_max_width_chars(&self) -> i32 {
        unsafe {
            ffi::gtk_label_get_max_width_chars(self.to_glib_none().0)
        }
    }

    pub fn get_mnemonic_keyval(&self) -> u32 {
        unsafe {
            ffi::gtk_label_get_mnemonic_keyval(self.to_glib_none().0)
        }
    }

    pub fn get_mnemonic_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_label_get_mnemonic_widget(self.to_glib_none().0))
        }
    }

    pub fn get_selectable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_label_get_selectable(self.to_glib_none().0))
        }
    }

    pub fn get_selection_bounds(&self, start: &mut i32, end: &mut i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_label_get_selection_bounds(self.to_glib_none().0, start, end))
        }
    }

    pub fn get_single_line_mode(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_label_get_single_line_mode(self.to_glib_none().0))
        }
    }

    pub fn get_text(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_label_get_text(self.to_glib_none().0))
        }
    }

    pub fn get_track_visited_links(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_label_get_track_visited_links(self.to_glib_none().0))
        }
    }

    pub fn get_use_markup(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_label_get_use_markup(self.to_glib_none().0))
        }
    }

    pub fn get_use_underline(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_label_get_use_underline(self.to_glib_none().0))
        }
    }

    pub fn get_width_chars(&self) -> i32 {
        unsafe {
            ffi::gtk_label_get_width_chars(self.to_glib_none().0)
        }
    }

    #[cfg(gtk_3_16)]
    pub fn get_xalign(&self) -> f32 {
        unsafe {
            ffi::gtk_label_get_xalign(self.to_glib_none().0)
        }
    }

    #[cfg(gtk_3_16)]
    pub fn get_yalign(&self) -> f32 {
        unsafe {
            ffi::gtk_label_get_yalign(self.to_glib_none().0)
        }
    }

    pub fn select_region(&self, start_offset: i32, end_offset: i32) {
        unsafe {
            ffi::gtk_label_select_region(self.to_glib_none().0, start_offset, end_offset);
        }
    }

    pub fn set_angle(&self, angle: f64) {
        unsafe {
            ffi::gtk_label_set_angle(self.to_glib_none().0, angle);
        }
    }

    //pub fn set_attributes(&self, attrs: /*Unknown kind*/Unknown rust type: "AttrList") {
    //    unsafe { TODO: call ffi:gtk_label_set_attributes() }
    //}

    //pub fn set_ellipsize(&self, mode: pango::EllipsizeMode) {
    //    unsafe { TODO: call ffi:gtk_label_set_ellipsize() }
    //}

    pub fn set_justify(&self, jtype: Justification) {
        unsafe {
            ffi::gtk_label_set_justify(self.to_glib_none().0, jtype);
        }
    }

    pub fn set_label(&self, str: &str) {
        unsafe {
            ffi::gtk_label_set_label(self.to_glib_none().0, str.to_glib_none().0);
        }
    }

    pub fn set_line_wrap(&self, wrap: bool) {
        unsafe {
            ffi::gtk_label_set_line_wrap(self.to_glib_none().0, wrap.to_glib());
        }
    }

    //pub fn set_line_wrap_mode(&self, wrap_mode: pango::WrapMode) {
    //    unsafe { TODO: call ffi:gtk_label_set_line_wrap_mode() }
    //}

    #[cfg(gtk_3_10)]
    pub fn set_lines(&self, lines: i32) {
        unsafe {
            ffi::gtk_label_set_lines(self.to_glib_none().0, lines);
        }
    }

    pub fn set_markup(&self, str: &str) {
        unsafe {
            ffi::gtk_label_set_markup(self.to_glib_none().0, str.to_glib_none().0);
        }
    }

    pub fn set_markup_with_mnemonic(&self, str: &str) {
        unsafe {
            ffi::gtk_label_set_markup_with_mnemonic(self.to_glib_none().0, str.to_glib_none().0);
        }
    }

    pub fn set_max_width_chars(&self, n_chars: i32) {
        unsafe {
            ffi::gtk_label_set_max_width_chars(self.to_glib_none().0, n_chars);
        }
    }

    pub fn set_mnemonic_widget<T: Upcast<Widget>>(&self, widget: Option<&T>) {
        unsafe {
            ffi::gtk_label_set_mnemonic_widget(self.to_glib_none().0, widget.map(Upcast::upcast).to_glib_none().0);
        }
    }

    pub fn set_pattern(&self, pattern: &str) {
        unsafe {
            ffi::gtk_label_set_pattern(self.to_glib_none().0, pattern.to_glib_none().0);
        }
    }

    pub fn set_selectable(&self, setting: bool) {
        unsafe {
            ffi::gtk_label_set_selectable(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn set_single_line_mode(&self, single_line_mode: bool) {
        unsafe {
            ffi::gtk_label_set_single_line_mode(self.to_glib_none().0, single_line_mode.to_glib());
        }
    }

    pub fn set_text(&self, str: &str) {
        unsafe {
            ffi::gtk_label_set_text(self.to_glib_none().0, str.to_glib_none().0);
        }
    }

    pub fn set_text_with_mnemonic(&self, str: &str) {
        unsafe {
            ffi::gtk_label_set_text_with_mnemonic(self.to_glib_none().0, str.to_glib_none().0);
        }
    }

    pub fn set_track_visited_links(&self, track_links: bool) {
        unsafe {
            ffi::gtk_label_set_track_visited_links(self.to_glib_none().0, track_links.to_glib());
        }
    }

    pub fn set_use_markup(&self, setting: bool) {
        unsafe {
            ffi::gtk_label_set_use_markup(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn set_use_underline(&self, setting: bool) {
        unsafe {
            ffi::gtk_label_set_use_underline(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn set_width_chars(&self, n_chars: i32) {
        unsafe {
            ffi::gtk_label_set_width_chars(self.to_glib_none().0, n_chars);
        }
    }

    #[cfg(gtk_3_16)]
    pub fn set_xalign(&self, xalign: f32) {
        unsafe {
            ffi::gtk_label_set_xalign(self.to_glib_none().0, xalign);
        }
    }

    #[cfg(gtk_3_16)]
    pub fn set_yalign(&self, yalign: f32) {
        unsafe {
            ffi::gtk_label_set_yalign(self.to_glib_none().0, yalign);
        }
    }

}

impl types::StaticType for Label {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_label_get_type()) }
    }
}
