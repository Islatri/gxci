//! Callback function for frame data acquisition, type declaration.
#![allow(dead_code)]

use crate::raw::{gx_enum::*, gx_struct::*};
use std::ffi::c_void;

pub type GXCaptureCallBack = extern "C" fn(pFrameData: *mut GX_FRAME_CALLBACK_PARAM);
pub type GXDeviceOfflineCallBack = extern "C" fn(pUserParam: *mut c_void);
pub type GXFeatureCallBack = extern "C" fn(nFeatureID: GX_FEATURE_ID, pUserParam: *mut c_void);
