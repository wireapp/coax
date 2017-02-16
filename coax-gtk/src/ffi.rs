use std::ffi::{CStr, CString};
use std::ptr;
use glib::translate::ToGlibPtr;
use glib_sys::{gpointer, g_markup_escape_text};
use gtk::{self, Label};
use gtk_sys::{self, GtkListBoxRow, gtk_label_set_ellipsize};
use gobject_sys::{GObject, g_object_set_data_full, g_object_get_data};
use gobject_sys::{g_signal_handler_block, g_signal_handler_unblock};
use gobject_sys::g_object_add_weak_pointer;
use pango_sys::PangoEllipsizeMode;
use libc::{c_char, c_int, ssize_t};

lazy_static! {
    pub static ref KEY_ID: CString = CString::new("id").unwrap();
    pub static ref TSTAMP: CString = CString::new("tstamp").unwrap();
}

extern fn drop_box(p: gpointer) {
    unsafe {
        Box::from_raw(p);
    }
}

pub fn set_data<'a, T, A>(obj: &'a T, k: &CStr, a: A)
    where T: ToGlibPtr<'a, *mut GObject>
{
    let data  = Box::into_raw(Box::new(a)) as gpointer;
    let stash = obj.to_glib_none();
    unsafe {
        g_object_set_data_full(stash.0, k.as_ptr(), data, Some(drop_box))
    }
}

pub fn get_data<'a, T, A>(obj: &'a T, k: &CStr) -> Option<&'a A>
    where T: ToGlibPtr<'a, *mut GObject>
{
    let stash = obj.to_glib_none();
    unsafe {
        let ptr = g_object_get_data(stash.0, k.as_ptr()) as *mut A;
        ptr.as_ref()
    }
}

pub fn block_handler<'a, T>(obj: &'a T, id: u64, block: bool)
    where T: ToGlibPtr<'a, *mut GObject>
{
    let stash = obj.to_glib_none();
    unsafe {
        if block {
            g_signal_handler_block(stash.0, id);
        } else {
            g_signal_handler_unblock(stash.0, id);
        }
    }
}

pub fn escape<'a>(s: &str) -> &'a CStr {
    unsafe {
        let ptr = g_markup_escape_text(s.as_ptr() as *const c_char, s.len() as ssize_t);
        CStr::from_ptr(ptr)
    }
}

pub fn set_ellipsis(lbl: &Label) {
    unsafe {
        gtk_label_set_ellipsize(lbl.to_glib_none().0, PangoEllipsizeMode::End)
    }
}

unsafe extern fn cmp_rows_by_time(a: *mut GtkListBoxRow, b: *mut GtkListBoxRow, _: gpointer) -> c_int {
    let a_time_ptr = g_object_get_data(a as *mut GObject, TSTAMP.as_ptr()) as *mut i64;
    let b_time_ptr = g_object_get_data(b as *mut GObject, TSTAMP.as_ptr()) as *mut i64;
    match (a_time_ptr.as_ref(), b_time_ptr.as_ref()) {
        (Some(ta), Some(tb)) => (tb - ta) as c_int,
        _                    => 0
    }
}

pub fn set_sort_by_time(r: &gtk::ListBox) {
    let stash = r.to_glib_none();
    unsafe {
        gtk_sys::gtk_list_box_set_sort_func(stash.0, Some(cmp_rows_by_time), ptr::null_mut(), None)
    }
}

pub fn add_weak_ptr<'a, T>(obj: &'a T) -> gpointer
    where T: ToGlibPtr<'a, *mut GObject>
{
    let stash = obj.to_glib_none();
    let mut p = stash.0 as gpointer;
    unsafe {
        g_object_add_weak_pointer(stash.0, &mut p)
    }
    p
}

