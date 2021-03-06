// This file was generated by gir (e43d6c3) from gir-files (???)
// DO NOT EDIT

use Error;
use URIType;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct URIHandler(Object<ffi::GstURIHandler>);

    match fn {
        get_type => || ffi::gst_uri_handler_get_type(),
    }
}

unsafe impl Send for URIHandler {}
unsafe impl Sync for URIHandler {}

pub trait URIHandlerExt {
    fn get_protocols(&self) -> Vec<String>;

    fn get_uri(&self) -> Option<String>;

    fn get_uri_type(&self) -> URIType;

    fn set_uri(&self, uri: &str) -> Result<(), Error>;
}

impl<O: IsA<URIHandler>> URIHandlerExt for O {
    fn get_protocols(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gst_uri_handler_get_protocols(self.to_glib_none().0))
        }
    }

    fn get_uri(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gst_uri_handler_get_uri(self.to_glib_none().0))
        }
    }

    fn get_uri_type(&self) -> URIType {
        unsafe {
            from_glib(ffi::gst_uri_handler_get_uri_type(self.to_glib_none().0))
        }
    }

    fn set_uri(&self, uri: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gst_uri_handler_set_uri(self.to_glib_none().0, uri.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }
}
