//! Raw GX Struct
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use super::gx_const::*;
use super::gx_enum::*;
use std::ffi::{c_char, c_int, c_void};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct GX_DEVICE_BASE_INFO {
    pub szVendorName: [u8; GX_INFO_LENGTH_32_BYTE],
    pub szModelName: [u8; GX_INFO_LENGTH_32_BYTE],
    pub szSN: [u8; GX_INFO_LENGTH_32_BYTE],
    pub szDisplayName: [u8; GX_INFO_LENGTH_128_BYTE],
    pub szDeviceID: [u8; GX_INFO_LENGTH_64_BYTE],
    pub szUserID: [u8; GX_INFO_LENGTH_64_BYTE],
    pub accessStatus: GX_ACCESS_STATUS,
    pub deviceClass: GX_DEVICE_CLASS,
    pub reserved: [u8; 300],
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct GX_DEVICE_IP_INFO {
    pub szDeviceID: [u8; GX_INFO_LENGTH_64_BYTE + 4],
    pub szMAC: [u8; GX_INFO_LENGTH_32_BYTE],
    pub szIP: [u8; GX_INFO_LENGTH_32_BYTE],
    pub szSubNetMask: [u8; GX_INFO_LENGTH_32_BYTE],
    pub szGateWay: [u8; GX_INFO_LENGTH_32_BYTE],
    pub szNICMAC: [u8; GX_INFO_LENGTH_32_BYTE],
    pub szNICIP: [u8; GX_INFO_LENGTH_32_BYTE],
    pub szNICSubNetMask: [u8; GX_INFO_LENGTH_32_BYTE],
    pub szNICGateWay: [u8; GX_INFO_LENGTH_32_BYTE],
    pub szNICDescription: [u8; GX_INFO_LENGTH_128_BYTE + 4],
    pub reserved: [u8; 512],
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct GX_OPEN_PARAM {
    pub pszContent: *const c_char,
    pub openMode: GX_OPEN_MODE,
    pub accessMode: GX_ACCESS_MODE,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GX_FRAME_CALLBACK_PARAM {
    pub pUserParam: *mut c_void,
    pub status: c_int,
    pub pImgBuf: *const c_void,
    pub nImgSize: i32,
    pub nWidth: i32,
    pub nHeight: i32,
    pub nPixelFormat: i32,
    pub nFrameID: u64,
    pub nTimestamp: u64,
    pub reserved: [i32; 1],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GX_FRAME_DATA {
    pub nStatus: c_int,
    pub pImgBuf: *mut c_void, // Pointer to the image buffer
    pub nWidth: i32,          // Image width, adjusted to i32 to match C definition
    pub nHeight: i32,         // Image height, adjusted to i32 to match C definition
    pub nPixelFormat: i32,    // Pixel format, adjusted to i32 to match C definition
    pub nImgSize: i32,        // Size of the image buffer, adjusted to i32 to match C definition
    pub nFrameID: u64,        // Frame ID
    pub nTimestamp: u64,      // Timestamp of the frame
    pub reserved: [i32; 1],   // Reserved, array of 1 i32 to match C definition
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct GX_FRAME_BUFFER {
    pub nStatus: GX_FRAME_STATUS,
    pub pImgBuf: *mut c_void,
    pub nWidth: i32,
    pub nHeight: i32,
    pub nPixelFormat: i32,
    pub nImgSize: i32,
    pub nFrameID: u64,
    pub nTimestamp: u64,
    pub nBufID: u64,
    pub nOffsetX: i32,
    pub nOffsetY: i32,
    pub reserved: [i32; 16],
}

pub type PGX_FRAME_BUFFER = *mut GX_FRAME_BUFFER;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct GX_INT_RANGE {
    pub nMin: i64,
    pub nMax: i64,
    pub nInc: i64,
    pub reserved: [i32; 8],
}

impl GX_INT_RANGE {
    pub fn new() -> Self {
        Self {
            nMin: 0,
            nMax: 0,
            nInc: 0,
            reserved: [0; 8],
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct GX_FLOAT_RANGE {
    pub dMin: f64,
    pub dMax: f64,
    pub dInc: f64,
    pub szUnit: [c_char; GX_INFO_LENGTH_8_BYTE],
    pub bIncIsValid: bool,
    pub reserved: [i8; 31],
}

impl GX_FLOAT_RANGE {
    pub fn new() -> Self {
        Self {
            dMin: 0.0,
            dMax: 0.0,
            dInc: 0.0,
            szUnit: [0; GX_INFO_LENGTH_8_BYTE],
            bIncIsValid: false,
            reserved: [0; 31],
        }
    }
}

// The packed attribute is used to tell the compiler to avoid padding between structure fields. Here just like - 4
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct GX_ENUM_DESCRIPTION {
    pub nValue: i64,
    // It's a AMAZING SOLUTION that - 4 to make the size of C's szSymbolic align to Rusts
    pub szSymbolic: [c_char; GX_INFO_LENGTH_64_BYTE - 4],
    pub reserved: [i32; 8],
}

impl GX_ENUM_DESCRIPTION {
    pub fn new() -> Self {
        Self {
            nValue: 0,
            szSymbolic: [0; GX_INFO_LENGTH_64_BYTE - 4],
            reserved: [0; 8],
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct GX_REGISTER_STACK_ENTRY {
    pub nAddress: u64,
    pub pBuffer: *mut c_void,
    pub nSize: usize,
}
