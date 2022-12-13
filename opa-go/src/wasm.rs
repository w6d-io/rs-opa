use std::{os::raw::{c_char, c_void}, path::Path, slice};

use opa_go_sys::{Free, GoInt, GoSlice, GoString, WasmBuild};

use crate::error::{Error, GoError};

struct GoWasm {
    ptr: *const u8,
    len: usize,
}

impl From<GoWasm> for Vec<u8> {
    fn from(input: GoWasm) -> Vec<u8> {
        let bytes = unsafe {
            if input.ptr.is_null() {
                vec![]
            } else {
                let b = slice::from_raw_parts(input.ptr, input.len);
                Vec::from(b)
            }
        };
        bytes
    }
}

impl Drop for GoWasm {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { Free(self.ptr as *mut c_void) }
        }
    }
}

fn compile(
        query: GoString,
        data: GoSlice,
        bundles: GoSlice,
        ignore: GoSlice,
        ) -> Result<GoWasm, Error> {
        let result = unsafe { WasmBuild(query, data, bundles, ignore) };
        if !result.r0.is_null() && !result.r2.is_null() {
            let r = GoWasm {
                ptr: result.r0 as *const u8,
                len: result.r1 as usize,
            };
            let err = GoError::new(result.r2);
            drop(err);
            Ok(r)
        } else if !result.r2.is_null() {
            let err = GoError::new(result.r2);
            Err(Error::from(err))
        } else if !result.r0.is_null() {
            let r = GoWasm {
                ptr: result.r0 as *const u8,
                len: result.r1 as usize,
            };
            Ok(r)
        } else {
            let message = "Result and error pointers are both null.".to_string();
            let e = Error::Compile(message);
            Err(e)
        }
    }

pub struct Wasm<P: AsRef<Path>, S: AsRef<str>>{
    query: S,
    data: P,
}

impl<P: AsRef<Path>, S: AsRef<str>> Wasm<P, S> {
    pub fn new(query: S, data: P) -> Self {
        Self{ query, data }
    }


    pub fn build(self) -> Result<Vec<u8>, Error> {
        let query = self.query.as_ref();
        let query = GoString {
            p: query.as_ptr() as *const c_char,
            n: query.len() as isize,
        };

        let data = self.data.as_ref().to_str().unwrap();
        let mut data = GoString {
            p: data.as_ptr() as *const c_char,
            n: data.len() as isize,
        };
        let data = slice::from_mut(&mut data);
        let data = GoSlice {
            data: data.as_mut_ptr() as *mut c_void,
            len: data.len() as GoInt,
            cap: data.len() as GoInt,
        };

        let bundles = GoSlice {
            data: std::ptr::null_mut() as *mut c_void,
            len: 0,
            cap: 0,
        };

        let ignore = GoSlice {
            data: std::ptr::null_mut() as *mut c_void,
            len: 0,
            cap: 0,
        };
        let build = compile(query, data, bundles, ignore)?;
        let bytes = build.into();
        Ok(bytes)
    }
}
