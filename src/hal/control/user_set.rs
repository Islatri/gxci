//! Placeholder

use crate::error::Result;
use crate::hal::config::*;
use crate::hal::device::gxi_send_command;
use crate::raw::gx_enum::GX_FEATURE_ID;

#[cfg(feature = "solo")]
pub fn gxi_get_user_set_selector() -> Result<i64> {
    let user_set_selector: i64 = gxi_get_feature_value(GX_FEATURE_ID::GX_ENUM_USER_SET_SELECTOR)?;
    println!("Now, user set selector is {}", user_set_selector);
    Ok(user_set_selector)
}

#[cfg(feature = "solo")]
pub fn gxi_set_user_set_selector(user_set_selector: i64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_USER_SET_SELECTOR, &user_set_selector)?;
    println!("Now, user set selector is {}", user_set_selector);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_set_user_set_selector_default() -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_USER_SET_SELECTOR, &0i64)?;
    println!("Now, user set selector is default");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_set_user_set_selector_user_set0() -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_USER_SET_SELECTOR, &1i64)?;
    println!("Now, user set selector is user set 0");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_user_set_load() -> Result<()> {
    gxi_send_command(GX_FEATURE_ID::GX_COMMAND_USER_SET_LOAD)?;
    println!("Now, user set load");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_user_set_save() -> Result<()> {
    gxi_send_command(GX_FEATURE_ID::GX_COMMAND_USER_SET_SAVE)?;
    println!("Now, user set save");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_user_set_default() -> Result<i64> {
    let user_set_default: i64 = gxi_get_feature_value(GX_FEATURE_ID::GX_ENUM_USER_SET_DEFAULT)?;
    println!("Now, user set default is {}", user_set_default);
    Ok(user_set_default)
}

#[cfg(feature = "solo")]
pub fn gxi_set_user_set_default(user_set_default: i64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_USER_SET_DEFAULT, &user_set_default)?;
    println!("Now, user set default is {}", user_set_default);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_set_user_set_default_default() -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_USER_SET_DEFAULT, &0i64)?;
    println!("Now, user set default is default");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_set_user_set_default_user_set0() -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_USER_SET_DEFAULT, &1i64)?;
    println!("Now, user set default is user set 0");
    Ok(())
}
