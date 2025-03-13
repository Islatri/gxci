// use crate::raw::gx_struct::*;

// use opencv::{
//     core,
//     prelude::*,
// };

use crate::raw::gx_struct::*;
pub fn extract_image_data(frame_data: &GX_FRAME_DATA) -> Option<Vec<u8>> {
    unsafe {
        if frame_data.pImgBuf.is_null() {
            return None;
        }

        let data_len = (frame_data.nWidth * frame_data.nHeight) as usize;
        let data = std::slice::from_raw_parts(frame_data.pImgBuf as *const u8, data_len);

        Some(data.to_vec())
    }
}

// // 有待修改
// pub unsafe fn mono8_to_mat(frame_data: &GX_FRAME_DATA) -> Mat {
//     let mut mat = core::Mat::default();
//     if frame_data.nStatus == 0 {
//         let data = slice::from_raw_parts(frame_data.pImgBuf as *const u8, (frame_data.nWidth * frame_data.nHeight) as usize);

//         let mat = core::Mat::new_rows_cols_with_data(
//             frame_data.nHeight,
//             frame_data.nWidth,
//             // core::CV_8UC1,
//             data
//         ).unwrap();
//     }
//     mat
// }
