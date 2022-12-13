#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::{c_char, c_void};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl GoString{
    pub fn new(ptr: *const u8, len: usize) -> Self{
        Self{p: ptr as *const c_char, n: len as isize}
    }
}

impl GoSlice{
    pub fn new(data: *mut GoString, len: usize, cap: usize) -> Self{
        Self{data: data as *mut c_void, len: len as GoInt, cap: cap as GoInt}
    }


}
