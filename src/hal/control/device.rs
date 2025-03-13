//! Placeholder

use crate::error::Result;
use crate::hal::config::*;
use crate::raw::gx_enum::GX_FEATURE_ID;

#[cfg(feature = "solo")]
pub fn gxi_get_device_vendor_name() -> Result<String> {
    let vendor_name: String = gxi_get_feature_value(GX_FEATURE_ID::GX_STRING_DEVICE_VENDOR_NAME)?;
    println!("Now, vendor name is {}", vendor_name);
    Ok(vendor_name)
}

#[cfg(feature = "solo")]
pub fn gxi_get_device_model_name() -> Result<String> {
    let model_name: String = gxi_get_feature_value(GX_FEATURE_ID::GX_STRING_DEVICE_MODEL_NAME)?;
    println!("Now, model name is {}", model_name);
    Ok(model_name)
}

#[cfg(feature = "solo")]
pub fn gxi_get_device_version() -> Result<String> {
    let version: String = gxi_get_feature_value(GX_FEATURE_ID::GX_STRING_DEVICE_VERSION)?;
    println!("Now, version is {}", version);
    Ok(version)
}

#[cfg(feature = "solo")]
pub fn gxi_get_device_firmware_version() -> Result<String> {
    let firmware_version: String =
        gxi_get_feature_value(GX_FEATURE_ID::GX_STRING_DEVICE_FIRMWARE_VERSION)?;
    println!("Now, firmware version is {}", firmware_version);
    Ok(firmware_version)
}

#[cfg(feature = "solo")]
pub fn gxi_get_device_serial_number() -> Result<String> {
    let serial_number: String =
        gxi_get_feature_value(GX_FEATURE_ID::GX_STRING_DEVICE_SERIAL_NUMBER)?;
    println!("Now, serial number is {}", serial_number);
    Ok(serial_number)
}

#[cfg(feature = "solo")]
pub fn gxi_get_factory_setting_version() -> Result<String> {
    let factory_setting_version: String =
        gxi_get_feature_value(GX_FEATURE_ID::GX_STRING_FACTORY_SETTING_VERSION)?;
    println!(
        "Now, factory setting version is {}",
        factory_setting_version
    );
    Ok(factory_setting_version)
}

#[cfg(feature = "solo")]
pub fn gxi_get_device_user_id() -> Result<String> {
    let user_id: String = gxi_get_feature_value(GX_FEATURE_ID::GX_STRING_DEVICE_USERID)?;
    println!("Now, user id is {}", user_id);
    Ok(user_id)
}

#[cfg(feature = "solo")]
pub fn gxi_set_device_user_id(user_id: String) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_STRING_DEVICE_USERID, &user_id)?;
    println!("Now, user id is {}", user_id);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_device_link_selector() -> Result<i64> {
    let link_selector: i64 = gxi_get_feature_value(GX_FEATURE_ID::GX_INT_DEVICE_LINK_SELECTOR)?;
    println!("Now, link selector is {}", link_selector);
    Ok(link_selector)
}

#[cfg(feature = "solo")]
pub fn gxi_set_device_link_selector(link_selector: i64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_INT_DEVICE_LINK_SELECTOR, &link_selector)?;
    println!("Now, link selector is {}", link_selector);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_device_link_throughput_limit_mode() -> Result<i64> {
    let link_selector: i64 =
        gxi_get_feature_value(GX_FEATURE_ID::GX_ENUM_DEVICE_LINK_THROUGHPUT_LIMIT_MODE)?;
    println!("Now, link selector is {}", link_selector);
    Ok(link_selector)
}

#[cfg(feature = "solo")]
pub fn gxi_set_device_link_throughput_limit_mode(link_selector: i64) -> Result<()> {
    gxi_set_feature_value(
        GX_FEATURE_ID::GX_ENUM_DEVICE_LINK_THROUGHPUT_LIMIT_MODE,
        &link_selector,
    )?;
    println!("Now, link selector is {}", link_selector);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_set_device_link_throughput_limit_mode_off() -> Result<()> {
    gxi_set_feature_value(
        GX_FEATURE_ID::GX_ENUM_DEVICE_LINK_THROUGHPUT_LIMIT_MODE,
        &0i64,
    )?;
    println!("Now, link selector is off");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_set_device_link_throughput_limit_mode_on() -> Result<()> {
    gxi_set_feature_value(
        GX_FEATURE_ID::GX_ENUM_DEVICE_LINK_THROUGHPUT_LIMIT_MODE,
        &1i64,
    )?;
    println!("Now, link selector is on");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_device_link_throughput_limit() -> Result<i64> {
    let link_selector: i64 =
        gxi_get_feature_value(GX_FEATURE_ID::GX_INT_DEVICE_LINK_THROUGHPUT_LIMIT)?;
    println!("Now, link selector is {}", link_selector);
    Ok(link_selector)
}

#[cfg(feature = "solo")]
pub fn gxi_set_device_link_throughput_limit(link_selector: i64) -> Result<()> {
    gxi_set_feature_value(
        GX_FEATURE_ID::GX_INT_DEVICE_LINK_THROUGHPUT_LIMIT,
        &link_selector,
    )?;
    println!("Now, link selector is {}", link_selector);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_device_link_current_throughput() -> Result<i64> {
    let link_selector: i64 =
        gxi_get_feature_value(GX_FEATURE_ID::GX_INT_DEVICE_LINK_CURRENT_THROUGHPUT)?;
    println!("Now, link selector is {}", link_selector);
    Ok(link_selector)
}
