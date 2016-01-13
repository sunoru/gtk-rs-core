// This file was generated by gir (34ea1b9) from gir-files (11e0e6d)
// DO NOT EDIT

use Buildable;
use CellArea;
use CellLayout;
use CellRenderer;
use Rectangle;
use SortType;
use TreeIter;
use TreeModel;
use TreeViewColumnSizing;
use Widget;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use std::mem;

glib_wrapper! {
    pub struct TreeViewColumn(Object<ffi::GtkTreeViewColumn>): Buildable, CellLayout;

    match fn {
        get_type => || ffi::gtk_tree_view_column_get_type(),
    }
}

impl TreeViewColumn {
    pub fn new() -> TreeViewColumn {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_tree_view_column_new())
        }
    }

    pub fn new_with_area<T: IsA<CellArea>>(area: &T) -> TreeViewColumn {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(ffi::gtk_tree_view_column_new_with_area(area.to_glib_none().0))
        }
    }

    //pub fn new_with_attributes<T: IsA<CellRenderer>>(title: &str, cell: &T, : /*Unknown conversion*/Fundamental: VarArgs) -> TreeViewColumn {
    //    unsafe { TODO: call ffi::gtk_tree_view_column_new_with_attributes() }
    //}

    pub fn add_attribute<T: IsA<CellRenderer>>(&self, cell_renderer: &T, attribute: &str, column: i32) {
        unsafe {
            ffi::gtk_tree_view_column_add_attribute(self.to_glib_none().0, cell_renderer.to_glib_none().0, attribute.to_glib_none().0, column);
        }
    }

    pub fn cell_get_position<T: IsA<CellRenderer>>(&self, cell_renderer: &T) -> Option<(i32, i32)> {
        unsafe {
            let mut x_offset = mem::uninitialized();
            let mut width = mem::uninitialized();
            let ret = from_glib(ffi::gtk_tree_view_column_cell_get_position(self.to_glib_none().0, cell_renderer.to_glib_none().0, &mut x_offset, &mut width));
            if ret { Some((x_offset, width)) } else { None }
        }
    }

    pub fn cell_get_size(&self, cell_area: Option<&Rectangle>) -> (i32, i32, i32, i32) {
        unsafe {
            let mut x_offset = mem::uninitialized();
            let mut y_offset = mem::uninitialized();
            let mut width = mem::uninitialized();
            let mut height = mem::uninitialized();
            ffi::gtk_tree_view_column_cell_get_size(self.to_glib_none().0, cell_area.to_glib_none().0, &mut x_offset, &mut y_offset, &mut width, &mut height);
            (x_offset, y_offset, width, height)
        }
    }

    pub fn cell_is_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_column_cell_is_visible(self.to_glib_none().0))
        }
    }

    pub fn cell_set_cell_data<T: IsA<TreeModel>>(&self, tree_model: &T, iter: &mut TreeIter, is_expander: bool, is_expanded: bool) {
        unsafe {
            ffi::gtk_tree_view_column_cell_set_cell_data(self.to_glib_none().0, tree_model.to_glib_none().0, iter.to_glib_none_mut().0, is_expander.to_glib(), is_expanded.to_glib());
        }
    }

    pub fn clear(&self) {
        unsafe {
            ffi::gtk_tree_view_column_clear(self.to_glib_none().0);
        }
    }

    pub fn clear_attributes<T: IsA<CellRenderer>>(&self, cell_renderer: &T) {
        unsafe {
            ffi::gtk_tree_view_column_clear_attributes(self.to_glib_none().0, cell_renderer.to_glib_none().0);
        }
    }

    pub fn clicked(&self) {
        unsafe {
            ffi::gtk_tree_view_column_clicked(self.to_glib_none().0);
        }
    }

    pub fn focus_cell<T: IsA<CellRenderer>>(&self, cell: &T) {
        unsafe {
            ffi::gtk_tree_view_column_focus_cell(self.to_glib_none().0, cell.to_glib_none().0);
        }
    }

    pub fn get_alignment(&self) -> f32 {
        unsafe {
            ffi::gtk_tree_view_column_get_alignment(self.to_glib_none().0)
        }
    }

    pub fn get_button(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_tree_view_column_get_button(self.to_glib_none().0))
        }
    }

    pub fn get_clickable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_column_get_clickable(self.to_glib_none().0))
        }
    }

    pub fn get_expand(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_column_get_expand(self.to_glib_none().0))
        }
    }

    pub fn get_fixed_width(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_column_get_fixed_width(self.to_glib_none().0)
        }
    }

    pub fn get_max_width(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_column_get_max_width(self.to_glib_none().0)
        }
    }

    pub fn get_min_width(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_column_get_min_width(self.to_glib_none().0)
        }
    }

    pub fn get_reorderable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_column_get_reorderable(self.to_glib_none().0))
        }
    }

    pub fn get_resizable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_column_get_resizable(self.to_glib_none().0))
        }
    }

    pub fn get_sizing(&self) -> TreeViewColumnSizing {
        unsafe {
            ffi::gtk_tree_view_column_get_sizing(self.to_glib_none().0)
        }
    }

    pub fn get_sort_column_id(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_column_get_sort_column_id(self.to_glib_none().0)
        }
    }

    pub fn get_sort_indicator(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_column_get_sort_indicator(self.to_glib_none().0))
        }
    }

    pub fn get_sort_order(&self) -> SortType {
        unsafe {
            ffi::gtk_tree_view_column_get_sort_order(self.to_glib_none().0)
        }
    }

    pub fn get_spacing(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_column_get_spacing(self.to_glib_none().0)
        }
    }

    pub fn get_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_tree_view_column_get_title(self.to_glib_none().0))
        }
    }

    pub fn get_tree_view(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_tree_view_column_get_tree_view(self.to_glib_none().0))
        }
    }

    pub fn get_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_view_column_get_visible(self.to_glib_none().0))
        }
    }

    pub fn get_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_tree_view_column_get_widget(self.to_glib_none().0))
        }
    }

    pub fn get_width(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_column_get_width(self.to_glib_none().0)
        }
    }

    pub fn get_x_offset(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_view_column_get_x_offset(self.to_glib_none().0)
        }
    }

    pub fn pack_end<T: IsA<CellRenderer>>(&self, cell: &T, expand: bool) {
        unsafe {
            ffi::gtk_tree_view_column_pack_end(self.to_glib_none().0, cell.to_glib_none().0, expand.to_glib());
        }
    }

    pub fn pack_start<T: IsA<CellRenderer>>(&self, cell: &T, expand: bool) {
        unsafe {
            ffi::gtk_tree_view_column_pack_start(self.to_glib_none().0, cell.to_glib_none().0, expand.to_glib());
        }
    }

    pub fn queue_resize(&self) {
        unsafe {
            ffi::gtk_tree_view_column_queue_resize(self.to_glib_none().0);
        }
    }

    pub fn set_alignment(&self, xalign: f32) {
        unsafe {
            ffi::gtk_tree_view_column_set_alignment(self.to_glib_none().0, xalign);
        }
    }

    //pub fn set_attributes<T: IsA<CellRenderer>>(&self, cell_renderer: &T, : /*Unknown conversion*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_tree_view_column_set_attributes() }
    //}

    //pub fn set_cell_data_func<T: IsA<CellRenderer>>(&self, cell_renderer: &T, func: /*Unknown conversion*/Unknown rust type: "TreeCellDataFunc", func_data: Fundamental: Pointer, destroy: /*Unknown conversion*/Unknown rust type: "DestroyNotify") {
    //    unsafe { TODO: call ffi::gtk_tree_view_column_set_cell_data_func() }
    //}

    pub fn set_clickable(&self, clickable: bool) {
        unsafe {
            ffi::gtk_tree_view_column_set_clickable(self.to_glib_none().0, clickable.to_glib());
        }
    }

    pub fn set_expand(&self, expand: bool) {
        unsafe {
            ffi::gtk_tree_view_column_set_expand(self.to_glib_none().0, expand.to_glib());
        }
    }

    pub fn set_fixed_width(&self, fixed_width: i32) {
        unsafe {
            ffi::gtk_tree_view_column_set_fixed_width(self.to_glib_none().0, fixed_width);
        }
    }

    pub fn set_max_width(&self, max_width: i32) {
        unsafe {
            ffi::gtk_tree_view_column_set_max_width(self.to_glib_none().0, max_width);
        }
    }

    pub fn set_min_width(&self, min_width: i32) {
        unsafe {
            ffi::gtk_tree_view_column_set_min_width(self.to_glib_none().0, min_width);
        }
    }

    pub fn set_reorderable(&self, reorderable: bool) {
        unsafe {
            ffi::gtk_tree_view_column_set_reorderable(self.to_glib_none().0, reorderable.to_glib());
        }
    }

    pub fn set_resizable(&self, resizable: bool) {
        unsafe {
            ffi::gtk_tree_view_column_set_resizable(self.to_glib_none().0, resizable.to_glib());
        }
    }

    pub fn set_sizing(&self, type_: TreeViewColumnSizing) {
        unsafe {
            ffi::gtk_tree_view_column_set_sizing(self.to_glib_none().0, type_);
        }
    }

    pub fn set_sort_column_id(&self, sort_column_id: i32) {
        unsafe {
            ffi::gtk_tree_view_column_set_sort_column_id(self.to_glib_none().0, sort_column_id);
        }
    }

    pub fn set_sort_indicator(&self, setting: bool) {
        unsafe {
            ffi::gtk_tree_view_column_set_sort_indicator(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn set_sort_order(&self, order: SortType) {
        unsafe {
            ffi::gtk_tree_view_column_set_sort_order(self.to_glib_none().0, order);
        }
    }

    pub fn set_spacing(&self, spacing: i32) {
        unsafe {
            ffi::gtk_tree_view_column_set_spacing(self.to_glib_none().0, spacing);
        }
    }

    pub fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_tree_view_column_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

    pub fn set_visible(&self, visible: bool) {
        unsafe {
            ffi::gtk_tree_view_column_set_visible(self.to_glib_none().0, visible.to_glib());
        }
    }

    pub fn set_widget<T: IsA<Widget>>(&self, widget: Option<&T>) {
        unsafe {
            ffi::gtk_tree_view_column_set_widget(self.to_glib_none().0, widget.to_glib_none().0);
        }
    }
}
