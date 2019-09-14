// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use glib_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use AccelGroup;
use Align;
use Buildable;
use Container;
use Menu;
use MenuShell;
use RecentChooser;
use RecentFilter;
use RecentManager;
use RecentSortType;
use ResizeMode;
use Widget;

glib_wrapper! {
    pub struct RecentChooserMenu(Object<gtk_sys::GtkRecentChooserMenu, gtk_sys::GtkRecentChooserMenuClass, RecentChooserMenuClass>) @extends Menu, MenuShell, Container, Widget, @implements Buildable, RecentChooser;

    match fn {
        get_type => || gtk_sys::gtk_recent_chooser_menu_get_type(),
    }
}

impl RecentChooserMenu {
    pub fn new() -> RecentChooserMenu {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(gtk_sys::gtk_recent_chooser_menu_new()).unsafe_cast() }
    }

    pub fn new_for_manager<P: IsA<RecentManager>>(manager: &P) -> RecentChooserMenu {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_recent_chooser_menu_new_for_manager(
                manager.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }
}

impl Default for RecentChooserMenu {
    fn default() -> Self {
        Self::new()
    }
}

pub struct RecentChooserMenuBuilder {
    show_numbers: Option<bool>,
    accel_group: Option<AccelGroup>,
    accel_path: Option<String>,
    active: Option<i32>,
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    anchor_hints: Option<gdk::AnchorHints>,
    attach_widget: Option<Widget>,
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    menu_type_hint: Option<gdk::WindowTypeHint>,
    monitor: Option<i32>,
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    rect_anchor_dx: Option<i32>,
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    rect_anchor_dy: Option<i32>,
    reserve_toggle_size: Option<bool>,
    take_focus: Option<bool>,
    border_width: Option<u32>,
    child: Option<Widget>,
    resize_mode: Option<ResizeMode>,
    app_paintable: Option<bool>,
    can_default: Option<bool>,
    can_focus: Option<bool>,
    events: Option<gdk::EventMask>,
    expand: Option<bool>,
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    focus_on_click: Option<bool>,
    halign: Option<Align>,
    has_default: Option<bool>,
    has_focus: Option<bool>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    is_focus: Option<bool>,
    margin: Option<i32>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    no_show_all: Option<bool>,
    opacity: Option<f64>,
    parent: Option<Container>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
    filter: Option<RecentFilter>,
    limit: Option<i32>,
    local_only: Option<bool>,
    recent_manager: Option<RecentManager>,
    select_multiple: Option<bool>,
    show_icons: Option<bool>,
    show_not_found: Option<bool>,
    show_private: Option<bool>,
    show_tips: Option<bool>,
    sort_type: Option<RecentSortType>,
}

impl RecentChooserMenuBuilder {
    pub fn new() -> Self {
        Self {
            show_numbers: None,
            accel_group: None,
            accel_path: None,
            active: None,
            #[cfg(any(feature = "v3_22", feature = "dox"))]
            anchor_hints: None,
            attach_widget: None,
            #[cfg(any(feature = "v3_22", feature = "dox"))]
            menu_type_hint: None,
            monitor: None,
            #[cfg(any(feature = "v3_22", feature = "dox"))]
            rect_anchor_dx: None,
            #[cfg(any(feature = "v3_22", feature = "dox"))]
            rect_anchor_dy: None,
            reserve_toggle_size: None,
            take_focus: None,
            border_width: None,
            child: None,
            resize_mode: None,
            app_paintable: None,
            can_default: None,
            can_focus: None,
            events: None,
            expand: None,
            #[cfg(any(feature = "v3_20", feature = "dox"))]
            focus_on_click: None,
            halign: None,
            has_default: None,
            has_focus: None,
            has_tooltip: None,
            height_request: None,
            hexpand: None,
            hexpand_set: None,
            is_focus: None,
            margin: None,
            margin_bottom: None,
            margin_end: None,
            margin_start: None,
            margin_top: None,
            name: None,
            no_show_all: None,
            opacity: None,
            parent: None,
            receives_default: None,
            sensitive: None,
            tooltip_markup: None,
            tooltip_text: None,
            valign: None,
            vexpand: None,
            vexpand_set: None,
            visible: None,
            width_request: None,
            filter: None,
            limit: None,
            local_only: None,
            recent_manager: None,
            select_multiple: None,
            show_icons: None,
            show_not_found: None,
            show_private: None,
            show_tips: None,
            sort_type: None,
        }
    }

    pub fn build(self) -> RecentChooserMenu {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref show_numbers) = self.show_numbers {
            properties.push(("show-numbers", show_numbers));
        }
        if let Some(ref accel_group) = self.accel_group {
            properties.push(("accel-group", accel_group));
        }
        if let Some(ref accel_path) = self.accel_path {
            properties.push(("accel-path", accel_path));
        }
        if let Some(ref active) = self.active {
            properties.push(("active", active));
        }
        #[cfg(any(feature = "v3_22", feature = "dox"))]
        {
            if let Some(ref anchor_hints) = self.anchor_hints {
                properties.push(("anchor-hints", anchor_hints));
            }
        }
        if let Some(ref attach_widget) = self.attach_widget {
            properties.push(("attach-widget", attach_widget));
        }
        #[cfg(any(feature = "v3_22", feature = "dox"))]
        {
            if let Some(ref menu_type_hint) = self.menu_type_hint {
                properties.push(("menu-type-hint", menu_type_hint));
            }
        }
        if let Some(ref monitor) = self.monitor {
            properties.push(("monitor", monitor));
        }
        #[cfg(any(feature = "v3_22", feature = "dox"))]
        {
            if let Some(ref rect_anchor_dx) = self.rect_anchor_dx {
                properties.push(("rect-anchor-dx", rect_anchor_dx));
            }
        }
        #[cfg(any(feature = "v3_22", feature = "dox"))]
        {
            if let Some(ref rect_anchor_dy) = self.rect_anchor_dy {
                properties.push(("rect-anchor-dy", rect_anchor_dy));
            }
        }
        if let Some(ref reserve_toggle_size) = self.reserve_toggle_size {
            properties.push(("reserve-toggle-size", reserve_toggle_size));
        }
        if let Some(ref take_focus) = self.take_focus {
            properties.push(("take-focus", take_focus));
        }
        if let Some(ref border_width) = self.border_width {
            properties.push(("border-width", border_width));
        }
        if let Some(ref child) = self.child {
            properties.push(("child", child));
        }
        if let Some(ref resize_mode) = self.resize_mode {
            properties.push(("resize-mode", resize_mode));
        }
        if let Some(ref app_paintable) = self.app_paintable {
            properties.push(("app-paintable", app_paintable));
        }
        if let Some(ref can_default) = self.can_default {
            properties.push(("can-default", can_default));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref events) = self.events {
            properties.push(("events", events));
        }
        if let Some(ref expand) = self.expand {
            properties.push(("expand", expand));
        }
        #[cfg(any(feature = "v3_20", feature = "dox"))]
        {
            if let Some(ref focus_on_click) = self.focus_on_click {
                properties.push(("focus-on-click", focus_on_click));
            }
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_default) = self.has_default {
            properties.push(("has-default", has_default));
        }
        if let Some(ref has_focus) = self.has_focus {
            properties.push(("has-focus", has_focus));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref is_focus) = self.is_focus {
            properties.push(("is-focus", is_focus));
        }
        if let Some(ref margin) = self.margin {
            properties.push(("margin", margin));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref no_show_all) = self.no_show_all {
            properties.push(("no-show-all", no_show_all));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref parent) = self.parent {
            properties.push(("parent", parent));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        if let Some(ref filter) = self.filter {
            properties.push(("filter", filter));
        }
        if let Some(ref limit) = self.limit {
            properties.push(("limit", limit));
        }
        if let Some(ref local_only) = self.local_only {
            properties.push(("local-only", local_only));
        }
        if let Some(ref recent_manager) = self.recent_manager {
            properties.push(("recent-manager", recent_manager));
        }
        if let Some(ref select_multiple) = self.select_multiple {
            properties.push(("select-multiple", select_multiple));
        }
        if let Some(ref show_icons) = self.show_icons {
            properties.push(("show-icons", show_icons));
        }
        if let Some(ref show_not_found) = self.show_not_found {
            properties.push(("show-not-found", show_not_found));
        }
        if let Some(ref show_private) = self.show_private {
            properties.push(("show-private", show_private));
        }
        if let Some(ref show_tips) = self.show_tips {
            properties.push(("show-tips", show_tips));
        }
        if let Some(ref sort_type) = self.sort_type {
            properties.push(("sort-type", sort_type));
        }
        glib::Object::new(RecentChooserMenu::static_type(), &properties)
            .expect("object new")
            .downcast()
            .expect("downcast")
    }

    pub fn show_numbers(mut self, show_numbers: bool) -> Self {
        self.show_numbers = Some(show_numbers);
        self
    }

    pub fn accel_group(mut self, accel_group: &AccelGroup) -> Self {
        self.accel_group = Some(accel_group.clone());
        self
    }

    pub fn accel_path(mut self, accel_path: &str) -> Self {
        self.accel_path = Some(accel_path.to_string());
        self
    }

    pub fn active(mut self, active: i32) -> Self {
        self.active = Some(active);
        self
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    pub fn anchor_hints(mut self, anchor_hints: gdk::AnchorHints) -> Self {
        self.anchor_hints = Some(anchor_hints);
        self
    }

    pub fn attach_widget(mut self, attach_widget: &Widget) -> Self {
        self.attach_widget = Some(attach_widget.clone());
        self
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    pub fn menu_type_hint(mut self, menu_type_hint: gdk::WindowTypeHint) -> Self {
        self.menu_type_hint = Some(menu_type_hint);
        self
    }

    pub fn monitor(mut self, monitor: i32) -> Self {
        self.monitor = Some(monitor);
        self
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    pub fn rect_anchor_dx(mut self, rect_anchor_dx: i32) -> Self {
        self.rect_anchor_dx = Some(rect_anchor_dx);
        self
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    pub fn rect_anchor_dy(mut self, rect_anchor_dy: i32) -> Self {
        self.rect_anchor_dy = Some(rect_anchor_dy);
        self
    }

    pub fn reserve_toggle_size(mut self, reserve_toggle_size: bool) -> Self {
        self.reserve_toggle_size = Some(reserve_toggle_size);
        self
    }

    pub fn take_focus(mut self, take_focus: bool) -> Self {
        self.take_focus = Some(take_focus);
        self
    }

    pub fn border_width(mut self, border_width: u32) -> Self {
        self.border_width = Some(border_width);
        self
    }

    pub fn child(mut self, child: &Widget) -> Self {
        self.child = Some(child.clone());
        self
    }

    pub fn resize_mode(mut self, resize_mode: ResizeMode) -> Self {
        self.resize_mode = Some(resize_mode);
        self
    }

    pub fn app_paintable(mut self, app_paintable: bool) -> Self {
        self.app_paintable = Some(app_paintable);
        self
    }

    pub fn can_default(mut self, can_default: bool) -> Self {
        self.can_default = Some(can_default);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn events(mut self, events: gdk::EventMask) -> Self {
        self.events = Some(events);
        self
    }

    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_default(mut self, has_default: bool) -> Self {
        self.has_default = Some(has_default);
        self
    }

    pub fn has_focus(mut self, has_focus: bool) -> Self {
        self.has_focus = Some(has_focus);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn is_focus(mut self, is_focus: bool) -> Self {
        self.is_focus = Some(is_focus);
        self
    }

    pub fn margin(mut self, margin: i32) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn no_show_all(mut self, no_show_all: bool) -> Self {
        self.no_show_all = Some(no_show_all);
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn parent(mut self, parent: &Container) -> Self {
        self.parent = Some(parent.clone());
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }

    pub fn filter(mut self, filter: &RecentFilter) -> Self {
        self.filter = Some(filter.clone());
        self
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn local_only(mut self, local_only: bool) -> Self {
        self.local_only = Some(local_only);
        self
    }

    pub fn recent_manager(mut self, recent_manager: &RecentManager) -> Self {
        self.recent_manager = Some(recent_manager.clone());
        self
    }

    pub fn select_multiple(mut self, select_multiple: bool) -> Self {
        self.select_multiple = Some(select_multiple);
        self
    }

    pub fn show_icons(mut self, show_icons: bool) -> Self {
        self.show_icons = Some(show_icons);
        self
    }

    pub fn show_not_found(mut self, show_not_found: bool) -> Self {
        self.show_not_found = Some(show_not_found);
        self
    }

    pub fn show_private(mut self, show_private: bool) -> Self {
        self.show_private = Some(show_private);
        self
    }

    pub fn show_tips(mut self, show_tips: bool) -> Self {
        self.show_tips = Some(show_tips);
        self
    }

    pub fn sort_type(mut self, sort_type: RecentSortType) -> Self {
        self.sort_type = Some(sort_type);
        self
    }
}

pub const NONE_RECENT_CHOOSER_MENU: Option<&RecentChooserMenu> = None;

pub trait RecentChooserMenuExt: 'static {
    fn get_show_numbers(&self) -> bool;

    fn set_show_numbers(&self, show_numbers: bool);

    fn connect_property_show_numbers_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;
}

impl<O: IsA<RecentChooserMenu>> RecentChooserMenuExt for O {
    fn get_show_numbers(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_recent_chooser_menu_get_show_numbers(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_show_numbers(&self, show_numbers: bool) {
        unsafe {
            gtk_sys::gtk_recent_chooser_menu_set_show_numbers(
                self.as_ref().to_glib_none().0,
                show_numbers.to_glib(),
            );
        }
    }

    fn connect_property_show_numbers_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_numbers_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkRecentChooserMenu,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<RecentChooserMenu>,
        {
            let f: &F = &*(f as *const F);
            f(&RecentChooserMenu::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-numbers\0".as_ptr() as *const _,
                Some(transmute(
                    notify_show_numbers_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for RecentChooserMenu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RecentChooserMenu")
    }
}
