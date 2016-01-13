// This file was generated by gir (34ea1b9) from gir-files (11e0e6d)
// DO NOT EDIT

use TreeIter;
use TreeModelFlags;
use TreePath;
use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct TreeModel(Object<ffi::GtkTreeModel>);

    match fn {
        get_type => || ffi::gtk_tree_model_get_type(),
    }
}

pub trait TreeModelExt {
    fn filter_new(&self, root: Option<&TreePath>) -> Option<TreeModel>;

    //fn foreach(&self, func: /*Unknown conversion*/Unknown rust type: "TreeModelForeachFunc", user_data: Fundamental: Pointer);

    //fn get(&self, iter: &mut TreeIter, : /*Unknown conversion*/Fundamental: VarArgs);

    fn get_column_type(&self, index_: i32) -> glib::types::Type;

    fn get_flags(&self) -> TreeModelFlags;

    fn get_iter(&self, path: &TreePath) -> Option<TreeIter>;

    fn get_iter_first(&self) -> Option<TreeIter>;

    fn get_iter_from_string(&self, path_string: &str) -> Option<TreeIter>;

    fn get_n_columns(&self) -> i32;

    fn get_path(&self, iter: &TreeIter) -> TreePath;

    fn get_string_from_iter(&self, iter: &mut TreeIter) -> Option<String>;

    //fn get_valist(&self, iter: &mut TreeIter, var_args: /*Unknown conversion*/Unsupported);

    //fn get_value(&self, iter: &mut TreeIter, column: i32, value: /*Ignored*/gobject::Value);

    fn iter_children(&self, parent: Option<&TreeIter>) -> Option<TreeIter>;

    fn iter_has_child(&self, iter: &TreeIter) -> bool;

    fn iter_n_children(&self, iter: Option<&TreeIter>) -> i32;

    fn iter_next(&self, iter: &mut TreeIter) -> bool;

    fn iter_nth_child(&self, parent: Option<&TreeIter>, n: i32) -> Option<TreeIter>;

    fn iter_parent(&self, child: &TreeIter) -> Option<TreeIter>;

    fn iter_previous(&self, iter: &mut TreeIter) -> bool;

    fn row_changed(&self, path: &TreePath, iter: &TreeIter);

    fn row_deleted(&self, path: &TreePath);

    fn row_has_child_toggled(&self, path: &TreePath, iter: &TreeIter);

    fn row_inserted(&self, path: &TreePath, iter: &TreeIter);

    //#[cfg(gtk_3_10)]
    //fn rows_reordered_with_length(&self, path: &mut TreePath, iter: Option<&mut TreeIter>, new_order: &Unknown rust type: "CArray TypeId { ns_id: 0, id: 14 }", length: i32);

    fn sort_new_with_model(&self) -> Option<TreeModel>;
}

impl<O: IsA<TreeModel>> TreeModelExt for O {
    fn filter_new(&self, root: Option<&TreePath>) -> Option<TreeModel> {
        unsafe {
            from_glib_full(ffi::gtk_tree_model_filter_new(self.to_glib_none().0, mut_override(root.to_glib_none().0)))
        }
    }

    //fn foreach(&self, func: /*Unknown conversion*/Unknown rust type: "TreeModelForeachFunc", user_data: Fundamental: Pointer) {
    //    unsafe { TODO: call ffi::gtk_tree_model_foreach() }
    //}

    //fn get(&self, iter: &mut TreeIter, : /*Unknown conversion*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_tree_model_get() }
    //}

    fn get_column_type(&self, index_: i32) -> glib::types::Type {
        unsafe {
            from_glib(ffi::gtk_tree_model_get_column_type(self.to_glib_none().0, index_))
        }
    }

    fn get_flags(&self) -> TreeModelFlags {
        unsafe {
            ffi::gtk_tree_model_get_flags(self.to_glib_none().0)
        }
    }

    fn get_iter(&self, path: &TreePath) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ret = from_glib(ffi::gtk_tree_model_get_iter(self.to_glib_none().0, iter.to_glib_none_mut().0, mut_override(path.to_glib_none().0)));
            if ret { Some(iter) } else { None }
        }
    }

    fn get_iter_first(&self) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ret = from_glib(ffi::gtk_tree_model_get_iter_first(self.to_glib_none().0, iter.to_glib_none_mut().0));
            if ret { Some(iter) } else { None }
        }
    }

    fn get_iter_from_string(&self, path_string: &str) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ret = from_glib(ffi::gtk_tree_model_get_iter_from_string(self.to_glib_none().0, iter.to_glib_none_mut().0, path_string.to_glib_none().0));
            if ret { Some(iter) } else { None }
        }
    }

    fn get_n_columns(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_model_get_n_columns(self.to_glib_none().0)
        }
    }

    fn get_path(&self, iter: &TreeIter) -> TreePath {
        unsafe {
            from_glib_full(ffi::gtk_tree_model_get_path(self.to_glib_none().0, mut_override(iter.to_glib_none().0)))
        }
    }

    fn get_string_from_iter(&self, iter: &mut TreeIter) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_tree_model_get_string_from_iter(self.to_glib_none().0, iter.to_glib_none_mut().0))
        }
    }

    //fn get_valist(&self, iter: &mut TreeIter, var_args: /*Unknown conversion*/Unsupported) {
    //    unsafe { TODO: call ffi::gtk_tree_model_get_valist() }
    //}

    //fn get_value(&self, iter: &mut TreeIter, column: i32, value: /*Ignored*/gobject::Value) {
    //    unsafe { TODO: call ffi::gtk_tree_model_get_value() }
    //}

    fn iter_children(&self, parent: Option<&TreeIter>) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ret = from_glib(ffi::gtk_tree_model_iter_children(self.to_glib_none().0, iter.to_glib_none_mut().0, mut_override(parent.to_glib_none().0)));
            if ret { Some(iter) } else { None }
        }
    }

    fn iter_has_child(&self, iter: &TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_model_iter_has_child(self.to_glib_none().0, mut_override(iter.to_glib_none().0)))
        }
    }

    fn iter_n_children(&self, iter: Option<&TreeIter>) -> i32 {
        unsafe {
            ffi::gtk_tree_model_iter_n_children(self.to_glib_none().0, mut_override(iter.to_glib_none().0))
        }
    }

    fn iter_next(&self, iter: &mut TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_model_iter_next(self.to_glib_none().0, iter.to_glib_none_mut().0))
        }
    }

    fn iter_nth_child(&self, parent: Option<&TreeIter>, n: i32) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ret = from_glib(ffi::gtk_tree_model_iter_nth_child(self.to_glib_none().0, iter.to_glib_none_mut().0, mut_override(parent.to_glib_none().0), n));
            if ret { Some(iter) } else { None }
        }
    }

    fn iter_parent(&self, child: &TreeIter) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ret = from_glib(ffi::gtk_tree_model_iter_parent(self.to_glib_none().0, iter.to_glib_none_mut().0, mut_override(child.to_glib_none().0)));
            if ret { Some(iter) } else { None }
        }
    }

    fn iter_previous(&self, iter: &mut TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_model_iter_previous(self.to_glib_none().0, iter.to_glib_none_mut().0))
        }
    }

    fn row_changed(&self, path: &TreePath, iter: &TreeIter) {
        unsafe {
            ffi::gtk_tree_model_row_changed(self.to_glib_none().0, mut_override(path.to_glib_none().0), mut_override(iter.to_glib_none().0));
        }
    }

    fn row_deleted(&self, path: &TreePath) {
        unsafe {
            ffi::gtk_tree_model_row_deleted(self.to_glib_none().0, mut_override(path.to_glib_none().0));
        }
    }

    fn row_has_child_toggled(&self, path: &TreePath, iter: &TreeIter) {
        unsafe {
            ffi::gtk_tree_model_row_has_child_toggled(self.to_glib_none().0, mut_override(path.to_glib_none().0), mut_override(iter.to_glib_none().0));
        }
    }

    fn row_inserted(&self, path: &TreePath, iter: &TreeIter) {
        unsafe {
            ffi::gtk_tree_model_row_inserted(self.to_glib_none().0, mut_override(path.to_glib_none().0), mut_override(iter.to_glib_none().0));
        }
    }

    //#[cfg(gtk_3_10)]
    //fn rows_reordered_with_length(&self, path: &mut TreePath, iter: Option<&mut TreeIter>, new_order: &Unknown rust type: "CArray TypeId { ns_id: 0, id: 14 }", length: i32) {
    //    unsafe { TODO: call ffi::gtk_tree_model_rows_reordered_with_length() }
    //}

    fn sort_new_with_model(&self) -> Option<TreeModel> {
        unsafe {
            from_glib_full(ffi::gtk_tree_model_sort_new_with_model(self.to_glib_none().0))
        }
    }
}
