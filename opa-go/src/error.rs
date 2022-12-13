use std::ffi::CStr;
use std::os::raw::{c_char, c_void};
use thiserror::Error;

use opa_go_sys::*;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error compiling to wasm: {0}")]
    Compile(String),
}

pub(crate) struct GoError {
    ptr: *const c_char,
}

impl GoError {
    pub fn new(ptr: *const c_char) -> GoError {
        GoError{ptr}
    }
}

impl Drop for GoError {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { Free(self.ptr as *mut c_void) }
        }
    }
}

impl From<GoError> for Error {
    fn from(error: GoError) -> Self {
        let message = unsafe { CStr::from_ptr(error.ptr).to_string_lossy().into_owned() };
        Self::Compile(message)
    }
}
