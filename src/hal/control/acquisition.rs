//! Placeholder

use crate::error::Result;
use crate::hal::config::*;
use crate::hal::device::gxi_send_command;
use crate::raw::gx_enum::GX_FEATURE_ID;

#[cfg(feature = "solo")]
pub fn gxi_get_acquisition_mode() -> Result<i64> {
    let acquisition_mode: i64 = gxi_get_feature_value(GX_FEATURE_ID::GX_ENUM_ACQUISITION_MODE)?;
    println!("Now, acquisition mode is {}", acquisition_mode);
    Ok(acquisition_mode)
}

#[cfg(feature = "solo")]
pub fn gxi_set_acquisition_mode(acquisition_mode: i64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_ACQUISITION_MODE, &acquisition_mode)?;
    println!("Now, acquisition mode is {}", acquisition_mode);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_set_acquisition_mode_continuous() -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_ACQUISITION_MODE, &1i64)?;
    println!("Now, acquisition mode is continuous");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_acquisition_start() -> Result<()> {
    gxi_send_command(GX_FEATURE_ID::GX_COMMAND_ACQUISITION_START)?;
    println!("Acquisition started");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_acquisition_stop() -> Result<()> {
    gxi_send_command(GX_FEATURE_ID::GX_COMMAND_ACQUISITION_STOP)?;
    println!("Acquisition stopped");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_trigger_selector() -> Result<i64> {
    let trigger_selector: i64 = gxi_get_feature_value(GX_FEATURE_ID::GX_ENUM_TRIGGER_SELECTOR)?;
    println!("Now, trigger selector is {}", trigger_selector);
    Ok(trigger_selector)
}

#[cfg(feature = "solo")]
pub fn gxi_set_trigger_selector(trigger_selector: i64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_TRIGGER_SELECTOR, &trigger_selector)?;
    println!("Now, trigger selector is {}", trigger_selector);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_set_trigger_selector_frame_start() -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_TRIGGER_SELECTOR, &0i64)?;
    println!("Now, trigger selector is 0");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_trigger_mode() -> Result<i64> {
    let trigger_mode: i64 = gxi_get_feature_value(GX_FEATURE_ID::GX_ENUM_TRIGGER_MODE)?;
    println!("Now, trigger mode is {}", trigger_mode);
    Ok(trigger_mode)
}

#[cfg(feature = "solo")]
pub fn gxi_set_trigger_mode(trigger_mode: i64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_TRIGGER_MODE, &trigger_mode)?;
    println!("Now, trigger mode is {}", trigger_mode);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_set_trigger_mode_off() -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_TRIGGER_MODE, &0i64)?;
    println!("Now, trigger mode is 0");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_trigger_source() -> Result<i64> {
    let trigger_source: i64 = gxi_get_feature_value(GX_FEATURE_ID::GX_ENUM_TRIGGER_SOURCE)?;
    println!("Now, trigger source is {}", trigger_source);
    Ok(trigger_source)
}

#[cfg(feature = "solo")]
pub fn gxi_set_trigger_source(trigger_source: i64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_TRIGGER_SOURCE, &trigger_source)?;
    println!("Now, trigger source is {}", trigger_source);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_set_trigger_source_software() -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_TRIGGER_SOURCE, &0i64)?;
    println!("Now, trigger source is 0");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_trigger_activation() -> Result<i64> {
    let trigger_activation: i64 = gxi_get_feature_value(GX_FEATURE_ID::GX_ENUM_TRIGGER_ACTIVATION)?;
    println!("Now, trigger activation is {}", trigger_activation);
    Ok(trigger_activation)
}

#[cfg(feature = "solo")]
pub fn gxi_set_trigger_activation(trigger_activation: i64) -> Result<()> {
    gxi_set_feature_value(
        GX_FEATURE_ID::GX_ENUM_TRIGGER_ACTIVATION,
        &trigger_activation,
    )?;
    println!("Now, trigger activation is {}", trigger_activation);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_set_trigger_activation_falling_edge() -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_TRIGGER_ACTIVATION, &0i64)?;
    println!("Now, trigger activation is 0");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_set_trigger_activation_rising_edge() -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_TRIGGER_ACTIVATION, &1i64)?;
    println!("Now, trigger activation is 1");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_trigger_delay() -> Result<f64> {
    let trigger_delay: f64 = gxi_get_feature_value(GX_FEATURE_ID::GX_FLOAT_TRIGGER_DELAY)?;
    println!("Now, trigger delay is {}", trigger_delay);
    Ok(trigger_delay)
}

#[cfg(feature = "solo")]
pub fn gxi_set_trigger_delay(trigger_delay: f64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_FLOAT_TRIGGER_DELAY, &trigger_delay)?;
    println!("Now, trigger delay is {}", trigger_delay);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_trigger_filter_rasing_edge() -> Result<f64> {
    let trigger_filter_rasing_edge: f64 =
        gxi_get_feature_value(GX_FEATURE_ID::GX_FLOAT_TRIGGER_FILTER_RAISING)?;
    println!(
        "Now, trigger filter rasing edge is {}",
        trigger_filter_rasing_edge
    );
    Ok(trigger_filter_rasing_edge)
}

#[cfg(feature = "solo")]
pub fn gxi_set_trigger_filter_rasing_edge(trigger_filter_rasing_edge: f64) -> Result<()> {
    gxi_set_feature_value(
        GX_FEATURE_ID::GX_FLOAT_TRIGGER_FILTER_RAISING,
        &trigger_filter_rasing_edge,
    )?;
    println!(
        "Now, trigger filter rasing edge is {}",
        trigger_filter_rasing_edge
    );
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_trigger_filter_falling_edge() -> Result<f64> {
    let trigger_filter_falling_edge: f64 =
        gxi_get_feature_value(GX_FEATURE_ID::GX_FLOAT_TRIGGER_FILTER_FALLING)?;
    println!(
        "Now, trigger filter falling edge is {}",
        trigger_filter_falling_edge
    );
    Ok(trigger_filter_falling_edge)
}

#[cfg(feature = "solo")]
pub fn gxi_set_trigger_filter_falling_edge(trigger_filter_falling_edge: f64) -> Result<()> {
    gxi_set_feature_value(
        GX_FEATURE_ID::GX_FLOAT_TRIGGER_FILTER_FALLING,
        &trigger_filter_falling_edge,
    )?;
    println!(
        "Now, trigger filter falling edge is {}",
        trigger_filter_falling_edge
    );
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_exposure_mode() -> Result<i64> {
    let exposure_mode: i64 = gxi_get_feature_value(GX_FEATURE_ID::GX_ENUM_EXPOSURE_MODE)?;
    println!("Now, exposure mode is {}", exposure_mode);
    Ok(exposure_mode)
}

#[cfg(feature = "solo")]
pub fn gxi_set_exposure_mode(exposure_mode: i64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_EXPOSURE_MODE, &exposure_mode)?;
    println!("Now, exposure mode is {}", exposure_mode);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_set_exposure_mode_timed() -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_EXPOSURE_MODE, &1i64)?;
    println!("Now, exposure mode is 1");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_exposure_time() -> Result<f64> {
    let exp_time: f64 = gxi_get_feature_value(GX_FEATURE_ID::GX_FLOAT_EXPOSURE_TIME)?;
    println!("Now, exposure time is {}", exp_time);
    Ok(exp_time)
}

#[cfg(feature = "solo")]
pub fn gxi_set_exposure_time(exp_time: f64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_FLOAT_EXPOSURE_TIME, &exp_time)?;
    println!("Successfully set exposure time to {}", exp_time);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_exposure_auto() -> Result<i64> {
    let exposure_time_auto: i64 = gxi_get_feature_value(GX_FEATURE_ID::GX_ENUM_EXPOSURE_AUTO)?;
    println!("Now, exposure time auto is {}", exposure_time_auto);
    Ok(exposure_time_auto)
}

#[cfg(feature = "solo")]
pub fn gxi_set_exposure_auto(exposure_time_auto: i64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_EXPOSURE_AUTO, &exposure_time_auto)?;
    println!("Now, exposure time auto is {}", exposure_time_auto);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_set_exposure_auto_off() -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_EXPOSURE_AUTO, &0i64)?;
    println!("Now, exposure time auto is 0");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_set_exposure_auto_continuous() -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_EXPOSURE_AUTO, &1i64)?;
    println!("Now, exposure time auto is 1");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_set_exposure_auto_once() -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_EXPOSURE_AUTO, &2i64)?;
    println!("Now, exposure time auto is 2");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_auto_exposure_time_min() -> Result<f64> {
    let min: f64 = gxi_get_feature_value(GX_FEATURE_ID::GX_FLOAT_AUTO_EXPOSURE_TIME_MIN)?;
    println!("Now, auto exposure time min is {}", min);
    Ok(min)
}

#[cfg(feature = "solo")]
pub fn gxi_set_auto_exposure_time_min(min: f64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_FLOAT_AUTO_EXPOSURE_TIME_MIN, &min)?;
    println!("Successfully set auto exposure time min to {}", min);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_auto_exposure_time_max() -> Result<f64> {
    let max: f64 = gxi_get_feature_value(GX_FEATURE_ID::GX_FLOAT_AUTO_EXPOSURE_TIME_MAX)?;
    println!("Now, auto exposure time max is {}", max);
    Ok(max)
}

#[cfg(feature = "solo")]
pub fn gxi_set_auto_exposure_time_max(max: f64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_FLOAT_AUTO_EXPOSURE_TIME_MAX, &max)?;
    println!("Successfully set auto exposure time max to {}", max);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_aaroi_width() -> Result<i64> {
    let aaroi_width: i64 = gxi_get_feature_value(GX_FEATURE_ID::GX_INT_AAROI_WIDTH)?;
    println!("Now, aaroi width is {}", aaroi_width);
    Ok(aaroi_width)
}

#[cfg(feature = "solo")]
pub fn gxi_set_aaroi_width(aaroi_width: i64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_INT_AAROI_WIDTH, &aaroi_width)?;
    println!("Now, aaroi width is {}", aaroi_width);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_aaroi_height() -> Result<i64> {
    let aaroi_height: i64 = gxi_get_feature_value(GX_FEATURE_ID::GX_INT_AAROI_HEIGHT)?;
    println!("Now, aaroi height is {}", aaroi_height);
    Ok(aaroi_height)
}

#[cfg(feature = "solo")]
pub fn gxi_set_aaroi_height(aaroi_height: i64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_INT_AAROI_HEIGHT, &aaroi_height)?;
    println!("Now, aaroi height is {}", aaroi_height);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_aaroi_offset_x() -> Result<i64> {
    let aaroi_offset_x: i64 = gxi_get_feature_value(GX_FEATURE_ID::GX_INT_AAROI_OFFSETX)?;
    println!("Now, aaroi offset x is {}", aaroi_offset_x);
    Ok(aaroi_offset_x)
}

#[cfg(feature = "solo")]
pub fn gxi_set_aaroi_offset_x(aaroi_offset_x: i64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_INT_AAROI_OFFSETX, &aaroi_offset_x)?;
    println!("Now, aaroi offset x is {}", aaroi_offset_x);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_aaroi_offset_y() -> Result<i64> {
    let aaroi_offset_y: i64 = gxi_get_feature_value(GX_FEATURE_ID::GX_INT_AAROI_OFFSETY)?;
    println!("Now, aaroi offset y is {}", aaroi_offset_y);
    Ok(aaroi_offset_y)
}

#[cfg(feature = "solo")]
pub fn gxi_set_aaroi_offset_y(aaroi_offset_y: i64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_INT_AAROI_OFFSETY, &aaroi_offset_y)?;
    println!("Now, aaroi offset y is {}", aaroi_offset_y);
    Ok(())
}

// I only found GX_INT_GRAY_VALUE but not GX_INT_EXPECTED_GRAY_VALUE

// #[cfg(feature = "solo")]
// pub fn gxi_get_expected_gray_value() -> Result<i64> {
//     let expected_gray_value:i64 = gxi_get_feature_value(GX_FEATURE_ID::GX_INT_EXPECTED_GRAY_VALUE)?;
//     println!("Now, expected gray value is {}",expected_gray_value);
//     Ok(expected_gray_value)
// }

// #[cfg(feature = "solo")]
// pub fn gxi_set_expected_gray_value(expected_gray_value:i64) -> Result<()> {
//     gxi_set_feature_value(GX_FEATURE_ID::GX_INT_EXPECTED_GRAY_VALUE,&expected_gray_value)?;
//     println!("Now, expected gray value is {}",expected_gray_value);
//     Ok(())
// }
