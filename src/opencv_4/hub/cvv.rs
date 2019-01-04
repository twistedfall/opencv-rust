//! # GUI for Interactive Visual Debugging of Computer Vision Programs
//!
//! Namespace for all functions is **cvv**, i.e. *cvv::showImage()*.
//!
//! Compilation:
//!
//! *   For development, i.e. for cvv GUI to show up, compile your code using cvv with
//! *g++ -DCVVISUAL_DEBUGMODE*.
//! *   For release, i.e. cvv calls doing nothing, compile your code without above flag.
//!
//! See cvv tutorial for a commented example application using cvv.
use std::os::raw::{c_char, c_void};
use libc::{ptrdiff_t, size_t};
use crate::{Error, Result, core, sys, types};
use crate::core::{_InputArrayTrait, _OutputArrayTrait};


/// Returns whether debug-mode is active for this TU and thread.
pub fn debug_mode() -> Result<bool> {
    unsafe { sys::cvv_debugMode() }.into_result()
}

/// Passes the control to the debug-window for a last time.
///
/// This function **must** be called *once* *after* all cvv calls if any. As an alternative create an
/// instance of FinalShowCaller, which calls finalShow() in its destructor (RAII-style).
pub fn final_show() -> Result<()> {
    unsafe { sys::cvv_finalShow() }.into_result()
}

pub fn final_show_1() -> Result<()> {
    unsafe { sys::cvv_impl_finalShow() }.into_result()
}

/// The debug-flag-singleton
pub fn get_debug_flag() -> Result<bool> {
    unsafe { sys::cvv_impl_getDebugFlag() }.into_result()
}

/// Enable or disable cvv for current translation unit and thread
///
/// (disabled this way has higher - but still low - overhead compared to using the compile flags).
/// ## Parameters
/// * active:
pub fn set_debug_flag(active: bool) -> Result<()> {
    unsafe { sys::cvv_setDebugFlag_bool(active) }.into_result()
}

// boxed class cvv::FinalShowCaller
/// RAII-class to call finalShow() in it's dtor.
pub struct FinalShowCaller {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for FinalShowCaller {
    fn drop(&mut self) {
        unsafe { sys::cv_FinalShowCaller_delete(self.ptr) };
    }
}

impl FinalShowCaller {
    #[inline(always)] pub fn as_raw_FinalShowCaller(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for FinalShowCaller {}

// boxed class cvv::impl::CallMetaData
/// Optional information about a location in Code.
pub struct CallMetaData {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for CallMetaData {
    fn drop(&mut self) {
        unsafe { sys::cv_CallMetaData_delete(self.ptr) };
    }
}

impl CallMetaData {
    #[inline(always)] pub fn as_raw_CallMetaData(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for CallMetaData {}

impl CallMetaData {
    /// Creates an unknown location.
    pub fn default() -> Result<crate::cvv::CallMetaData> {
        unsafe { sys::cvv_impl_CallMetaData_CallMetaData() }.into_result().map(|ptr| crate::cvv::CallMetaData { ptr })
    }
    
    /// Creates the provided location.
    ///
    /// Argument should be self-explaining.
    pub fn new(file: &str, line: size_t, function: &str) -> Result<crate::cvv::CallMetaData> {
        string_arg!(file);
        string_arg!(function);
        unsafe { sys::cvv_impl_CallMetaData_CallMetaData_const_char_X_size_t_const_char_X(file.as_ptr(), line, function.as_ptr()) }.into_result().map(|ptr| crate::cvv::CallMetaData { ptr })
    }
    
    pub fn to_bool(&mut self) -> Result<bool> {
        unsafe { sys::cvv_impl_CallMetaData_operator_bool(self.as_raw_CallMetaData()) }.into_result()
    }
    
}

