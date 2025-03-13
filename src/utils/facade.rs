#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::error::Result;
use crate::raw::{gx_enum::*, gx_handle::GX_DEV_HANDLE, gx_interface::*, gx_struct::*};

use std::ffi::{c_int, c_void};

pub struct GXFrameDataFacade {
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

pub fn convert_to_frame_data(facade: &GXFrameDataFacade) -> GX_FRAME_DATA {
    GX_FRAME_DATA {
        nStatus: facade.nStatus,
        pImgBuf: facade.pImgBuf,
        nWidth: facade.nWidth,
        nHeight: facade.nHeight,
        nPixelFormat: facade.nPixelFormat,
        nImgSize: facade.nImgSize,
        nFrameID: facade.nFrameID,
        nTimestamp: facade.nTimestamp,
        reserved: facade.reserved,
    }
}

pub fn fetch_frame_data(
    gx: &GXInstance,
    device_handle: GX_DEV_HANDLE,
) -> Result<(GXFrameDataFacade, Vec<u8>)> {
    let mut width_value: i64 = 0;
    let mut height_value: i64 = 0;
    let mut pixel_format: i64 = 1;
    let mut payload_size: i64 = 1;
    let mut pixel_size: i64 = 1;

    gx.gx_get_int(device_handle, GX_FEATURE_ID::GX_INT_WIDTH, &mut width_value)?;
    gx.gx_get_int(
        device_handle,
        GX_FEATURE_ID::GX_INT_HEIGHT,
        &mut height_value,
    )?;
    gx.gx_get_enum(
        device_handle,
        GX_FEATURE_ID::GX_ENUM_PIXEL_SIZE,
        &mut pixel_size,
    )?;
    gx.gx_get_enum(
        device_handle,
        GX_FEATURE_ID::GX_ENUM_PIXEL_FORMAT,
        &mut pixel_format,
    )?;
    gx.gx_get_int(
        device_handle,
        GX_FEATURE_ID::GX_INT_PAYLOAD_SIZE,
        &mut payload_size,
    )?;

    let image_size = (width_value * height_value * pixel_size) as usize;
    let mut image_buffer = vec![1u8; image_size]; // 分配图像缓冲区
    let p_img_buf = image_buffer.as_mut_ptr();

    Ok((
        GXFrameDataFacade {
            nStatus: 0,
            pImgBuf: p_img_buf as *mut c_void,
            nWidth: width_value as i32,
            nHeight: height_value as i32,
            nPixelFormat: pixel_format as i32,
            nImgSize: image_size as i32,
            nFrameID: 0,
            nTimestamp: 0,
            reserved: [pixel_format as i32],
        },
        image_buffer,
    ))
}
