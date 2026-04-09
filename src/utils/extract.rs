use crate::raw::gx_struct::{GX_ENUM_DESCRIPTION, GX_FRAME_CALLBACK_PARAM, GX_FRAME_DATA};

pub fn extract_sz_symbolic(data: GX_ENUM_DESCRIPTION) -> String {
    let symbolic_bytes: Vec<u8> = data
        .szSymbolic
        .iter()
        .take_while(|&&x| x != 0)
        .map(|&x| x as u8) // Convert i8 to u8
        .collect();

    String::from_utf8_lossy(&symbolic_bytes).to_string()
}

pub fn extract_n_value(data: GX_ENUM_DESCRIPTION) -> i64 {
    data.nValue
}

pub fn extract_callback_img_buf(frame_callback_data: &GX_FRAME_CALLBACK_PARAM) -> &'static [u8] {
    unsafe {
        if frame_callback_data.status == 0 {
            let data_len = (frame_callback_data.nWidth * frame_callback_data.nHeight) as usize;
            std::slice::from_raw_parts(frame_callback_data.pImgBuf as *const u8, data_len)
        } else {
            &[]
        }
    }
}

pub fn extract_img_buf(frame_data: &GX_FRAME_DATA) -> &'static [u8] {
    unsafe {
        if frame_data.nStatus == 0 {
            let data_len = (frame_data.nWidth * frame_data.nHeight) as usize;
            std::slice::from_raw_parts(frame_data.pImgBuf as *const u8, data_len)
        } else {
            &[]
        }
    }
}

/// Extracts a reference to a frame callback parameter from a raw pointer.
///
/// # Safety
///
/// The caller must ensure that:
/// - The pointer is valid and not null
/// - The pointer is properly aligned
/// - The pointer points to a valid `GX_FRAME_CALLBACK_PARAM` instance
/// - The lifetime of the returned reference is valid as long as the pointed data is accessible
pub unsafe fn extract_frame_callback_param(
    p_frame_callback_data: *mut GX_FRAME_CALLBACK_PARAM,
) -> &'static GX_FRAME_CALLBACK_PARAM {
    // SAFETY: The caller must ensure that the pointer is valid and properly aligned
    unsafe { &*p_frame_callback_data }
}
