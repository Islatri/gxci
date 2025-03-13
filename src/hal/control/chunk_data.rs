//! Placeholder

use crate::error::Result;
use crate::hal::config::*;
use crate::raw::gx_enum::GX_FEATURE_ID;

#[cfg(feature = "solo")]
pub fn gxi_get_chunk_mode_active() -> Result<bool> {
    let mode_active: bool = gxi_get_feature_value(GX_FEATURE_ID::GX_BOOL_CHUNKMODE_ACTIVE)?;
    println!("Now, chunk mode active is {}", mode_active);
    Ok(mode_active)
}

#[cfg(feature = "solo")]
pub fn gxi_set_chunk_mode_active(mode_active: bool) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_BOOL_CHUNKMODE_ACTIVE, &mode_active)?;
    println!("Successfully set chunk mode active to {}", mode_active);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_chunk_selector() -> Result<i64> {
    let chunk_selector: i64 = gxi_get_feature_value(GX_FEATURE_ID::GX_ENUM_CHUNK_SELECTOR)?;
    println!("Now, chunk selector is {}", chunk_selector);
    Ok(chunk_selector)
}

#[cfg(feature = "solo")]
pub fn gxi_set_chunk_selector(chunk_selector: i64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_CHUNK_SELECTOR, &chunk_selector)?;
    println!("Now, chunk selector is {}", chunk_selector);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_set_chunk_selector_frame_id() -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_CHUNK_SELECTOR, &0i64)?;
    println!("Now, chunk selector is frame id");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_set_chunk_selector_timestamp() -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_CHUNK_SELECTOR, &1i64)?;
    println!("Now, chunk selector is timestamp");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_chunk_enable() -> Result<bool> {
    let chunk_enable: bool = gxi_get_feature_value(GX_FEATURE_ID::GX_BOOL_CHUNK_ENABLE)?;
    println!("Now, chunk enable is {}", chunk_enable);
    Ok(chunk_enable)
}

#[cfg(feature = "solo")]
pub fn gxi_set_chunk_enable(chunk_enable: bool) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_BOOL_CHUNK_ENABLE, &chunk_enable)?;
    println!("Successfully set chunk enable to {}", chunk_enable);
    Ok(())
}

// I cant find the feature id for chunk timestamp
// #[cfg(feature = "solo")]
// pub fn gxi_get_chunk_timestamp() -> Result<i64> {
//     let chunk_timestamp:i64 = gxi_get_feature_value(GX_FEATURE_ID::GX_INT_CHUNK_TIMESTAMP)?;
//     println!("Now, chunk timestamp is {}",chunk_timestamp);
//     Ok(chunk_timestamp)
// }
