use std::{
    ffi::CStr,
    os::raw::{c_char, c_void},
    fmt::{self, Display},
};
use thiserror::Error;

use opa_go_sys::*;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error compiling to wasm: {0}")]
    Compile(String),
    #[error("Go error: {0}")]
    Go(GoError),
}

#[derive(Debug)]
pub struct GoError {
    pub ptr: *const c_char,
}

impl Drop for GoError {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { Free(self.ptr as *mut c_void) }
        }
    }
}

impl Display for GoError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = unsafe { CStr::from_ptr(self.ptr).to_string_lossy().into_owned() };
        write!(fmt, "{}", message)
    }
}
