//! Placeholder

use crate::error::Result;
use crate::hal::config::*;
use crate::raw::gx_enum::GX_FEATURE_ID;

#[cfg(feature = "solo")]
pub fn gxi_get_sensor_width() -> Result<i64> {
    let sensor_width: i64 = gxi_get_feature_value(GX_FEATURE_ID::GX_INT_SENSOR_WIDTH)?;
    println!("Now, sensor width is {}", sensor_width);
    Ok(sensor_width)
}

#[cfg(feature = "solo")]
pub fn gxi_get_sensor_height() -> Result<i64> {
    let sensor_height: i64 = gxi_get_feature_value(GX_FEATURE_ID::GX_INT_SENSOR_HEIGHT)?;
    println!("Now, sensor height is {}", sensor_height);
    Ok(sensor_height)
}

#[cfg(feature = "solo")]
pub fn gxi_get_max_width() -> Result<i64> {
    let max_width: i64 = gxi_get_feature_value(GX_FEATURE_ID::GX_INT_WIDTH_MAX)?;
    println!("Now, max width is {}", max_width);
    Ok(max_width)
}

#[cfg(feature = "solo")]
pub fn gxi_get_max_height() -> Result<i64> {
    let max_height: i64 = gxi_get_feature_value(GX_FEATURE_ID::GX_INT_HEIGHT_MAX)?;
    println!("Now, max height is {}", max_height);
    Ok(max_height)
}

#[cfg(feature = "solo")]
pub fn gxi_get_width() -> Result<i64> {
    let width: i64 = gxi_get_feature_value(GX_FEATURE_ID::GX_INT_WIDTH)?;
    println!("Now, width is {}", width);
    Ok(width)
}

#[cfg(feature = "solo")]
pub fn gxi_set_width(width: i64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_INT_WIDTH, &width)?;
    println!("Now, width is {}", width);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_height() -> Result<i64> {
    let height: i64 = gxi_get_feature_value(GX_FEATURE_ID::GX_INT_HEIGHT)?;
    println!("Now, height is {}", height);
    Ok(height)
}

#[cfg(feature = "solo")]
pub fn gxi_set_height(height: i64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_INT_HEIGHT, &height)?;
    println!("Now, height is {}", height);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_offset_x() -> Result<i64> {
    let offset_x: i64 = gxi_get_feature_value(GX_FEATURE_ID::GX_INT_OFFSET_X)?;
    println!("Now, offset x is {}", offset_x);
    Ok(offset_x)
}

#[cfg(feature = "solo")]
pub fn gxi_set_offset_x(offset_x: i64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_INT_OFFSET_X, &offset_x)?;
    println!("Now, offset x is {}", offset_x);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_offset_y() -> Result<i64> {
    let offset_y: i64 = gxi_get_feature_value(GX_FEATURE_ID::GX_INT_OFFSET_Y)?;
    println!("Now, offset y is {}", offset_y);
    Ok(offset_y)
}

#[cfg(feature = "solo")]
pub fn gxi_set_offset_y(offset_y: i64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_INT_OFFSET_Y, &offset_y)?;
    println!("Now, offset y is {}", offset_y);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_region_selector() -> Result<i64> {
    let region_selector: i64 = gxi_get_feature_value(GX_FEATURE_ID::GX_ENUM_REGION_SELECTOR)?;
    println!("Now, region selector is {}", region_selector);
    Ok(region_selector)
}

#[cfg(feature = "solo")]
pub fn gxi_set_region_selector(region_selector: i64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_REGION_SELECTOR, &region_selector)?;
    println!("Now, region selector is {}", region_selector);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_set_region_selector_region0() -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_REGION_SELECTOR, &0i64)?;
    println!("Now, region selector is 0");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_pixel_format() -> Result<i64> {
    let pixel_format: i64 = gxi_get_feature_value(GX_FEATURE_ID::GX_ENUM_PIXEL_FORMAT)?;
    println!("Now, pixel format is {}", pixel_format);
    Ok(pixel_format)
}

#[cfg(feature = "solo")]
pub fn gxi_set_pixel_format(pixel_format: i64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_PIXEL_FORMAT, &pixel_format)?;
    println!("Now, pixel format is {}", pixel_format);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_set_pixel_format_bayer_rgb() -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_PIXEL_FORMAT, &0i64)?;
    println!("Now, pixel format is 0");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_set_pixel_format_bayer_rg10() -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_PIXEL_FORMAT, &1i64)?;
    println!("Now, pixel format is 1");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_pixel_size() -> Result<i64> {
    let pixel_size: i64 = gxi_get_feature_value(GX_FEATURE_ID::GX_ENUM_PIXEL_SIZE)?;
    println!("Now, pixel size is {}", pixel_size);
    Ok(pixel_size)
}

#[cfg(feature = "solo")]
pub fn gxi_get_pixel_color_filter() -> Result<i64> {
    let pixel_color_filter: i64 = gxi_get_feature_value(GX_FEATURE_ID::GX_ENUM_PIXEL_COLOR_FILTER)?;
    println!("Now, pixel color filter is {}", pixel_color_filter);
    Ok(pixel_color_filter)
}

#[cfg(feature = "solo")]
pub fn gxi_get_test_pattern_generator_selector() -> Result<i64> {
    let test_pattern_generator_selector: i64 =
        gxi_get_feature_value(GX_FEATURE_ID::GX_ENUM_TEST_PATTERN_GENERATOR_SELECTOR)?;
    println!(
        "Now, test pattern generator selector is {}",
        test_pattern_generator_selector
    );
    Ok(test_pattern_generator_selector)
}

#[cfg(feature = "solo")]
pub fn gxi_set_test_pattern_generator_selector(test_pattern_generator_selector: i64) -> Result<()> {
    gxi_set_feature_value(
        GX_FEATURE_ID::GX_ENUM_TEST_PATTERN_GENERATOR_SELECTOR,
        &test_pattern_generator_selector,
    )?;
    println!(
        "Now, test pattern generator selector is {}",
        test_pattern_generator_selector
    );
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_set_test_pattern_generator_selector_region0() -> Result<()> {
    gxi_set_feature_value(
        GX_FEATURE_ID::GX_ENUM_TEST_PATTERN_GENERATOR_SELECTOR,
        &0i64,
    )?;
    println!("Now, test pattern generator selector is 0");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_test_pattern() -> Result<i64> {
    let test_pattern: i64 = gxi_get_feature_value(GX_FEATURE_ID::GX_ENUM_TEST_PATTERN)?;
    println!("Now, test pattern is {}", test_pattern);
    Ok(test_pattern)
}

#[cfg(feature = "solo")]
pub fn gxi_set_test_pattern(test_pattern: i64) -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_TEST_PATTERN, &test_pattern)?;
    println!("Now, test pattern is {}", test_pattern);
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_set_test_pattern_off() -> Result<()> {
    gxi_set_feature_value(GX_FEATURE_ID::GX_ENUM_TEST_PATTERN, &0i64)?;
    println!("Now, test pattern is 0");
    Ok(())
}
