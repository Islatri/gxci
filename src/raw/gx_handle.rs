//! Raw GX handle
#![allow(dead_code)]
#![allow(non_camel_case_types)]

use std::ffi::c_void;

pub type GX_DEV_HANDLE = *mut c_void;
pub type GX_EVENT_CALLBACK_HANDLE = *mut c_void;
pub type GX_FEATURE_CALLBACK_HANDLE = *mut c_void;
