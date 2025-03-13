//! Some convert function about status
use crate::raw::gx_enum::*;

pub fn convert_to_gx_status(status_code: i32) -> GX_STATUS_LIST {
    match status_code {
        0 => GX_STATUS_LIST::GX_STATUS_SUCCESS,
        -1 => GX_STATUS_LIST::GX_STATUS_ERROR,
        -2 => GX_STATUS_LIST::GX_STATUS_NOT_FOUND_TL,
        -3 => GX_STATUS_LIST::GX_STATUS_NOT_FOUND_DEVICE,
        -4 => GX_STATUS_LIST::GX_STATUS_OFFLINE,
        -5 => GX_STATUS_LIST::GX_STATUS_INVALID_PARAMETER,
        -6 => GX_STATUS_LIST::GX_STATUS_INVALID_HANDLE,
        -7 => GX_STATUS_LIST::GX_STATUS_INVALID_CALL,
        -8 => GX_STATUS_LIST::GX_STATUS_INVALID_ACCESS,
        -9 => GX_STATUS_LIST::GX_STATUS_NEED_MORE_BUFFER,
        -10 => GX_STATUS_LIST::GX_STATUS_ERROR_TYPE,
        -11 => GX_STATUS_LIST::GX_STATUS_OUT_OF_RANGE,
        -12 => GX_STATUS_LIST::GX_STATUS_NOT_IMPLEMENTED,
        -13 => GX_STATUS_LIST::GX_STATUS_NOT_INIT_API,
        -14 => GX_STATUS_LIST::GX_STATUS_TIMEOUT,
        _ => GX_STATUS_LIST::GX_STATUS_ERROR, // Default error if unknown status code
    }
}

pub fn gx_status_describe(status: i32) {
    let gx_status = format!("{:?}", convert_to_gx_status(status));
    println!("GX Error: {:?}", gx_status);
}
