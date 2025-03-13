//! Placeholder

use crate::error::Result;
use crate::hal::config::*;
use crate::raw::gx_enum::GX_FEATURE_ID;

#[cfg(feature = "solo")]
pub fn gxi_get_payload_size() -> Result<u32> {
    let payload_size: u32 = gxi_get_feature_value(GX_FEATURE_ID::GX_INT_PAYLOAD_SIZE)?;
    println!("Now, payload size is {}", payload_size);
    Ok(payload_size)
}
