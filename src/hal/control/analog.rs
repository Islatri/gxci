//! Placeholder
use crate::error::Result;
use crate::hal::config::*;
use crate::raw::gx_enum::GX_FEATURE_ID;

#[cfg(feature = "solo")]
pub fn gxi_get_gain_selector() -> Result<i64> {
    let gain_selector: i64 = gxi_get_feature_value(GX_FEATURE_ID::GX_ENUM_GAIN_SELECTOR)?;
    println!("Now, gain selector is {}", gain_selector);
    Ok(gain_selector)
}

#[cfg(feature = "solo")]
pub fn gxi_set_gain_selector(gain_selector: i64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_GAIN_SELECTOR, &gain_selector)?;
    println!("Now, gain selector is {}", gain_selector);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_set_gain_selector_analog_all() -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_GAIN_SELECTOR, &0i64)?;
    println!("Now, gain selector is analog all");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_gain() -> Result<f64> {
    let gain: f64 = gxi_get_feature_value(GX_FEATURE_ID::GX_FLOAT_GAIN)?;
    println!("Now, gain is {}", gain);
    Ok(gain)
}

#[cfg(feature = "solo")]
pub fn gxi_set_gain(gain: f64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_FLOAT_GAIN, &gain)?;
    println!("Successfully set gain to {}", gain);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_gain_auto() -> Result<i64> {
    let gain_auto: i64 = gxi_get_feature_value(GX_FEATURE_ID::GX_ENUM_GAIN_AUTO)?;
    println!("Now, gain auto is {}", gain_auto);
    Ok(gain_auto)
}

#[cfg(feature = "solo")]
pub fn gxi_set_gain_auto(gain_auto: i64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_GAIN_AUTO, &gain_auto)?;
    println!("Now, gain auto is {}", gain_auto);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_set_gain_auto_off() -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_GAIN_AUTO, &0i64)?;
    println!("Now, gain auto is off");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_set_gain_auto_continuous() -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_GAIN_AUTO, &1i64)?;
    println!("Now, gain auto is continuous");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_set_gain_auto_once() -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_GAIN_AUTO, &2i64)?;
    println!("Now, gain auto is once");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_auto_gain_min() -> Result<f64> {
    let min: f64 = gxi_get_feature_value(GX_FEATURE_ID::GX_FLOAT_AUTO_GAIN_MIN)?;
    println!("Now, auto gain min is {}", min);
    Ok(min)
}

#[cfg(feature = "solo")]
pub fn gxi_set_auto_gain_min(min: f64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_FLOAT_AUTO_GAIN_MIN, &min)?;
    println!("Successfully set auto gain min to {}", min);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_auto_gain_max() -> Result<f64> {
    let max: f64 = gxi_get_feature_value(GX_FEATURE_ID::GX_FLOAT_AUTO_GAIN_MAX)?;
    println!("Now, auto gain max is {}", max);
    Ok(max)
}

#[cfg(feature = "solo")]
pub fn gxi_set_auto_gain_max(max: f64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_FLOAT_AUTO_GAIN_MAX, &max)?;
    println!("Successfully set auto gain max to {}", max);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_balance_ratio_selector() -> Result<i64> {
    let balance_ratio_selector: i64 =
        gxi_get_feature_value(GX_FEATURE_ID::GX_ENUM_BALANCE_RATIO_SELECTOR)?;
    println!("Now, balance ratio selector is {}", balance_ratio_selector);
    Ok(balance_ratio_selector)
}

#[cfg(feature = "solo")]
pub fn gxi_set_balance_ratio_selector(balance_ratio_selector: i64) -> Result<()> {
    gxi_set_feature_value(
        GX_FEATURE_ID::GX_ENUM_BALANCE_RATIO_SELECTOR,
        &balance_ratio_selector,
    )?;
    println!("Now, balance ratio selector is {}", balance_ratio_selector);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_set_balance_ratio_selector_red() -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_BALANCE_RATIO_SELECTOR, &0i64)?;
    println!("Now, balance ratio selector is red");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_set_balance_ratio_selector_green() -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_BALANCE_RATIO_SELECTOR, &1i64)?;
    println!("Now, balance ratio selector is green");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_set_balance_ratio_selector_blue() -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_BALANCE_RATIO_SELECTOR, &2i64)?;
    println!("Now, balance ratio selector is blue");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_balance_ratio() -> Result<f64> {
    let balance_ratio: f64 = gxi_get_feature_value(GX_FEATURE_ID::GX_FLOAT_BALANCE_RATIO)?;
    println!("Now, balance ratio is {}", balance_ratio);
    Ok(balance_ratio)
}

#[cfg(feature = "solo")]
pub fn gxi_get_balance_white_auto() -> Result<i64> {
    let balance_white_auto: i64 = gxi_get_feature_value(GX_FEATURE_ID::GX_ENUM_BALANCE_WHITE_AUTO)?;
    println!("Now, balance white auto is {}", balance_white_auto);
    Ok(balance_white_auto)
}

#[cfg(feature = "solo")]
pub fn gxi_set_balance_white_auto(balance_white_auto: i64) -> Result<()> {
    gxi_set_feature_value(
        GX_FEATURE_ID::GX_ENUM_BALANCE_WHITE_AUTO,
        &balance_white_auto,
    )?;
    println!("Now, balance white auto is {}", balance_white_auto);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_set_balance_white_auto_off() -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_BALANCE_WHITE_AUTO, &0i64)?;
    println!("Now, balance white auto is off");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_set_balance_white_auto_continuous() -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_BALANCE_WHITE_AUTO, &1i64)?;
    println!("Now, balance white auto is continuous");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_set_balance_white_auto_once() -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_BALANCE_WHITE_AUTO, &2i64)?;
    println!("Now, balance white auto is once");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_awb_lamp_house() -> Result<i64> {
    let awb_lamp_house: i64 = gxi_get_feature_value(GX_FEATURE_ID::GX_ENUM_AWB_LAMP_HOUSE)?;
    println!("Now, awbroi lamp house is {}", awb_lamp_house);
    Ok(awb_lamp_house)
}

#[cfg(feature = "solo")]
pub fn gxi_set_awb_lamp_house(awb_lamp_house: i64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_AWB_LAMP_HOUSE, &awb_lamp_house)?;
    println!("Now, awbroi lamp house is {}", awb_lamp_house);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_set_awb_lamp_house_adaptive() -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_AWB_LAMP_HOUSE, &0i64)?;
    println!("Now, awbroi lamp house is adaptive");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_awbroi_width() -> Result<i64> {
    let awbroi_width: i64 = gxi_get_feature_value(GX_FEATURE_ID::GX_INT_AWBROI_WIDTH)?;
    println!("Now, awbroi width is {}", awbroi_width);
    Ok(awbroi_width)
}

#[cfg(feature = "solo")]
pub fn gxi_set_awbroi_width(awbroi_width: i64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_INT_AWBROI_WIDTH, &awbroi_width)?;
    println!("Now, awbroi width is {}", awbroi_width);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_awbroi_height() -> Result<i64> {
    let awbroi_height: i64 = gxi_get_feature_value(GX_FEATURE_ID::GX_INT_AWBROI_HEIGHT)?;
    println!("Now, awbroi height is {}", awbroi_height);
    Ok(awbroi_height)
}

#[cfg(feature = "solo")]
pub fn gxi_set_awbroi_height(awbroi_height: i64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_INT_AWBROI_HEIGHT, &awbroi_height)?;
    println!("Now, awbroi height is {}", awbroi_height);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_awbroi_offset_x() -> Result<i64> {
    let awbroi_offset_x: i64 = gxi_get_feature_value(GX_FEATURE_ID::GX_INT_AWBROI_OFFSETX)?;
    println!("Now, awbroi offset x is {}", awbroi_offset_x);
    Ok(awbroi_offset_x)
}

#[cfg(feature = "solo")]
pub fn gxi_set_awbroi_offset_x(awbroi_offset_x: i64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_INT_AWBROI_OFFSETX, &awbroi_offset_x)?;
    println!("Now, awbroi offset x is {}", awbroi_offset_x);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_awbroi_offset_y() -> Result<i64> {
    let awbroi_offset_y: i64 = gxi_get_feature_value(GX_FEATURE_ID::GX_INT_AWBROI_OFFSETY)?;
    println!("Now, awbroi offset y is {}", awbroi_offset_y);
    Ok(awbroi_offset_y)
}

#[cfg(feature = "solo")]
pub fn gxi_set_awbroi_offset_y(awbroi_offset_y: i64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_INT_AWBROI_OFFSETY, &awbroi_offset_y)?;
    println!("Now, awbroi offset y is {}", awbroi_offset_y);
    Ok(())
}
