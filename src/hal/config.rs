//! Set of Get some values

use crate::error::{Error, ErrorKind, Result};
use crate::hal::base::gxi_check;
use crate::hal::check::check_gx_status;
use crate::hal::device::gxi_get_device_handle;
use crate::raw::gx_enum::{GX_FEATURE_ID, GX_FEATURE_TYPE};
use crate::raw::gx_interface::GXInterface;
use crate::raw::gx_struct::{GX_ENUM_DESCRIPTION, GX_FLOAT_RANGE, GX_INT_RANGE};
use crate::utils::extract::{extract_n_value, extract_sz_symbolic};
use crate::utils::matching::match_feature_type;
use std::ffi::CString;

//----------------------------------------------------------
//---------------HAL Functions---------------------------
//----------------------------------------------------------

// 用这种方法非常简洁的解决，得益于Any和Box了
#[cfg(feature = "solo")]
pub fn gxi_get_feature_value<T>(feature_id: GX_FEATURE_ID) -> Result<T>
where
    T: std::any::Any + std::fmt::Debug + Clone,
{
    let feature_type = match_feature_type(feature_id);
    let value: Box<dyn std::any::Any> = match feature_type {
        GX_FEATURE_TYPE::GX_FEATURE_INT => Box::new(gxi_get_int(feature_id)?),
        GX_FEATURE_TYPE::GX_FEATURE_FLOAT => Box::new(gxi_get_float(feature_id)?),
        GX_FEATURE_TYPE::GX_FEATURE_ENUM => Box::new(gxi_get_enum(feature_id)?),
        GX_FEATURE_TYPE::GX_FEATURE_BOOL => Box::new(gxi_get_bool(feature_id)?),
        GX_FEATURE_TYPE::GX_FEATURE_STRING => Box::new(gxi_get_string(feature_id)?),
        GX_FEATURE_TYPE::GX_FEATURE_BUFFER => Box::new(gxi_get_buffer(feature_id)?),
        _ => {
            return Err(Error::new(ErrorKind::InvalidFeatureType(
                "Invalid feature type.".to_string(),
            )));
        }
    };

    if let Some(result) = value.downcast_ref::<T>() {
        Ok(result.clone())
    } else {
        Err(Error::new(ErrorKind::InvalidFeatureType(format!(
            "Expected {}.",
            std::any::type_name::<T>()
        ))))
    }
}

#[cfg(feature = "solo")]
pub fn gxi_set_feature_value(feature_id: GX_FEATURE_ID, value: &dyn std::any::Any) -> Result<()> {
    let feature_type = match_feature_type(feature_id);
    match feature_type {
        GX_FEATURE_TYPE::GX_FEATURE_INT => {
            if let Some(int_value) = value.downcast_ref::<i64>() {
                gxi_set_int(feature_id, *int_value)
            } else {
                Err(Error::new(ErrorKind::InvalidFeatureType(
                    "Expected i64.".to_string(),
                )))
            }
        }
        GX_FEATURE_TYPE::GX_FEATURE_FLOAT => {
            if let Some(float_value) = value.downcast_ref::<f64>() {
                gxi_set_float(feature_id, *float_value)
            } else {
                Err(Error::new(ErrorKind::InvalidFeatureType(
                    "Expected f64.".to_string(),
                )))
            }
        }
        GX_FEATURE_TYPE::GX_FEATURE_ENUM => {
            if let Some(enum_value) = value.downcast_ref::<i64>() {
                gxi_set_enum(feature_id, *enum_value)
            } else {
                Err(Error::new(ErrorKind::InvalidFeatureType(
                    "Expected i64 enum.".to_string(),
                )))
            }
        }
        GX_FEATURE_TYPE::GX_FEATURE_BOOL => {
            if let Some(bool_value) = value.downcast_ref::<bool>() {
                gxi_set_bool(feature_id, *bool_value)
            } else {
                Err(Error::new(ErrorKind::InvalidFeatureType(
                    "Expected bool.".to_string(),
                )))
            }
        }
        GX_FEATURE_TYPE::GX_FEATURE_STRING => {
            if let Some(string_value) = value.downcast_ref::<String>() {
                gxi_set_string(feature_id, string_value)
            } else {
                Err(Error::new(ErrorKind::InvalidFeatureType(
                    "Expected String.".to_string(),
                )))
            }
        }
        GX_FEATURE_TYPE::GX_FEATURE_BUFFER => {
            if let Some(buffer_value) = value.downcast_ref::<Vec<u8>>() {
                gxi_set_buffer(feature_id, buffer_value)
            } else {
                Err(Error::new(ErrorKind::InvalidFeatureType(
                    "Expected Vec<u8>.".to_string(),
                )))
            }
        }
        _ => Err(Error::new(ErrorKind::InvalidFeatureType(
            "Invalid feature type.".to_string(),
        ))),
    }
}

#[cfg(feature = "solo")]
#[derive(Debug, Clone)]
pub struct GxiEnumDescription {
    pub n_value: i64,
    pub sz_symbolic: String,
    pub feature_id: GX_FEATURE_ID,
}

//----------------------------------------------------------
//---------------Raw-Warpper Functions---------------------------
//----------------------------------------------------------

#[cfg(feature = "solo")]
pub fn gxi_get_feature_name(feature_id: GX_FEATURE_ID) -> Result<String> {
    let gxi_device = gxi_get_device_handle()?;
    let mut buffer_size: usize = 8;
    let mut feature_name = vec![0u8; buffer_size];
    let status = gxi_check(|gxi| {
        gxi.gx_get_feature_name(
            gxi_device,
            feature_id,
            feature_name.as_mut_ptr() as *mut i8,
            &mut buffer_size,
        )
    })?;
    // 这里的feature_name.as_mut_ptr() as *mut i8是将feature_name的地址转换成i8类型的指钋

    check_gx_status(status)?;
    println!("Successfully get feature name.");
    Ok(String::from_utf8(feature_name).unwrap())
}

#[cfg(feature = "solo")]
pub fn gxi_get_int_range(feature_id: GX_FEATURE_ID) -> Result<GX_INT_RANGE> {
    let gxi_device = gxi_get_device_handle()?;
    let mut int_range = GX_INT_RANGE::new();
    let status = gxi_check(|gxi| gxi.gx_get_int_range(gxi_device, feature_id, &mut int_range))?;

    check_gx_status(status)?;
    println!("Successfully get int range.");
    Ok(int_range)
}

#[cfg(feature = "solo")]
pub fn gxi_get_int(feature_id: GX_FEATURE_ID) -> Result<i64> {
    let gxi_device = gxi_get_device_handle()?;
    let mut int_value = 0;
    let status = gxi_check(|gxi| gxi.gx_get_int(gxi_device, feature_id, &mut int_value))?;

    check_gx_status(status)?;
    println!("Successfully get int value.");
    Ok(int_value)
}

#[cfg(feature = "solo")]
pub fn gxi_set_int(feature_id: GX_FEATURE_ID, int_value: i64) -> Result<()> {
    let gxi_device = gxi_get_device_handle()?;
    let status = gxi_check(|gxi| gxi.gx_set_int(gxi_device, feature_id, int_value))?;

    check_gx_status(status)?;
    println!("Successfully set int value.");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_float_range(feature_id: GX_FEATURE_ID) -> Result<GX_FLOAT_RANGE> {
    let gxi_device = gxi_get_device_handle()?;
    let mut float_range = GX_FLOAT_RANGE::new();
    let status = gxi_check(|gxi| gxi.gx_get_float_range(gxi_device, feature_id, &mut float_range))?;

    check_gx_status(status)?;
    println!("Successfully get float range.");
    Ok(float_range)
}

#[cfg(feature = "solo")]
pub fn gxi_get_float(feature_id: GX_FEATURE_ID) -> Result<f64> {
    let gxi_device = gxi_get_device_handle()?;
    let mut float_value = 0.0;
    let status = gxi_check(|gxi| gxi.gx_get_float(gxi_device, feature_id, &mut float_value))?;

    check_gx_status(status)?;
    println!("Successfully get float value.");
    Ok(float_value)
}

#[cfg(feature = "solo")]
pub fn gxi_set_float(feature_id: GX_FEATURE_ID, float_value: f64) -> Result<()> {
    let gxi_device = gxi_get_device_handle()?;
    let status = gxi_check(|gxi| gxi.gx_set_float(gxi_device, feature_id, float_value))?;

    check_gx_status(status)?;
    println!("Successfully set float value.");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_enum_entry_nums(feature_id: GX_FEATURE_ID) -> Result<u32> {
    let gxi_device = gxi_get_device_handle()?;
    let mut enum_entry_nums = 0;
    let status =
        gxi_check(|gxi| gxi.gx_get_enum_entry_nums(gxi_device, feature_id, &mut enum_entry_nums))?;

    check_gx_status(status)?;
    println!("Successfully get enum entry number.");
    Ok(enum_entry_nums)
}

#[cfg(feature = "solo")]
pub fn gxi_get_enum_description(feature_id: GX_FEATURE_ID) -> Result<Vec<GxiEnumDescription>> {
    let gxi_device = gxi_get_device_handle()?;
    let enum_entry_nums = gxi_get_enum_entry_nums(feature_id)?;

    let mut enum_descriptions: Vec<GX_ENUM_DESCRIPTION> =
        vec![GX_ENUM_DESCRIPTION::new(); enum_entry_nums as usize];
    let enum_descriptions_ptr: *mut GX_ENUM_DESCRIPTION = enum_descriptions.as_mut_ptr();
    let mut buffer_size: usize =
        enum_entry_nums as usize * core::mem::size_of::<GX_ENUM_DESCRIPTION>() as usize + 1usize;

    let status = gxi_check(|gxi| {
        gxi.gx_get_enum_description(
            gxi_device,
            feature_id,
            enum_descriptions_ptr,
            &mut buffer_size,
        )
    })?;

    let mut gxi_enum_descriptions: Vec<GxiEnumDescription> = Vec::new();

    for i in 0..enum_entry_nums {
        let n_value = extract_n_value(enum_descriptions[i as usize]);
        let sz_symbolic = extract_sz_symbolic(enum_descriptions[i as usize]);
        gxi_enum_descriptions.push(GxiEnumDescription {
            n_value,
            sz_symbolic,
            feature_id,
        });
    }

    check_gx_status(status)?;
    println!("Successfully get enum description.");
    Ok(gxi_enum_descriptions)
}

#[cfg(feature = "solo")]
pub fn gxi_get_enum(feature_id: GX_FEATURE_ID) -> Result<i64> {
    let gxi_device = gxi_get_device_handle()?;
    let mut enum_value = 0;
    let status = gxi_check(|gxi| gxi.gx_get_enum(gxi_device, feature_id, &mut enum_value))?;

    check_gx_status(status)?;
    println!("Successfully get enum value.");
    Ok(enum_value)
}

#[cfg(feature = "solo")]
pub fn gxi_set_enum(feature_id: GX_FEATURE_ID, enum_value: i64) -> Result<()> {
    let gxi_device = gxi_get_device_handle()?;
    let status = gxi_check(|gxi| gxi.gx_set_enum(gxi_device, feature_id, enum_value))?;

    check_gx_status(status)?;
    println!("Successfully set enum value.");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_bool(feature_id: GX_FEATURE_ID) -> Result<bool> {
    let gxi_device = gxi_get_device_handle()?;
    let mut bool_value = false;
    let status = gxi_check(|gxi| gxi.gx_get_bool(gxi_device, feature_id, &mut bool_value))?;

    check_gx_status(status)?;
    println!("Successfully get bool value.");
    Ok(bool_value)
}

#[cfg(feature = "solo")]
pub fn gxi_set_bool(feature_id: GX_FEATURE_ID, bool_value: bool) -> Result<()> {
    let gxi_device = gxi_get_device_handle()?;
    let status = gxi_check(|gxi| gxi.gx_set_bool(gxi_device, feature_id, bool_value))?;

    check_gx_status(status)?;
    println!("Successfully set bool value.");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_string_length(feature_id: GX_FEATURE_ID) -> Result<usize> {
    let gxi_device = gxi_get_device_handle()?;
    let mut string_length = 0;
    let status =
        gxi_check(|gxi| gxi.gx_get_string_length(gxi_device, feature_id, &mut string_length))?;

    check_gx_status(status)?;
    println!("Successfully get string length.");
    Ok(string_length)
}

#[cfg(feature = "solo")]
pub fn gxi_get_string_max_length(feature_id: GX_FEATURE_ID) -> Result<usize> {
    let gxi_device = gxi_get_device_handle()?;
    let mut string_max_length = 0;
    let status = gxi_check(|gxi| {
        gxi.gx_get_string_max_length(gxi_device, feature_id, &mut string_max_length)
    })?;

    check_gx_status(status)?;
    println!("Successfully get string max length.");
    Ok(string_max_length)
}

#[cfg(feature = "solo")]
pub fn gxi_get_string(feature_id: GX_FEATURE_ID) -> Result<String> {
    let gxi_device = gxi_get_device_handle()?;
    let mut buffer_size: usize = gxi_get_string_length(feature_id)?;
    let mut string = vec![0u8; buffer_size];
    let status = gxi_check(|gxi| {
        gxi.gx_get_string(
            gxi_device,
            feature_id,
            string.as_mut_ptr() as *mut i8,
            &mut buffer_size,
        )
    })?;

    check_gx_status(status)?;
    println!("Successfully get string.");
    Ok(String::from_utf8(string).unwrap())
}

#[cfg(feature = "solo")]
pub fn gxi_set_string(feature_id: GX_FEATURE_ID, string_value: &str) -> Result<()> {
    let gxi_device = gxi_get_device_handle()?;

    // &str -> CString -> *mut i8
    let c_string = CString::new(string_value)?;
    let c_ptr: *mut i8 = c_string.as_ptr() as *mut i8;

    let status = gxi_check(|gxi| gxi.gx_set_string(gxi_device, feature_id, c_ptr))?;

    check_gx_status(status)?;
    println!("Successfully set string.");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_buffer_length(feature_id: GX_FEATURE_ID) -> Result<usize> {
    let gxi_device = gxi_get_device_handle()?;
    let mut buffer_length = 0;
    let status =
        gxi_check(|gxi| gxi.gx_get_buffer_length(gxi_device, feature_id, &mut buffer_length))?;

    check_gx_status(status)?;
    println!("Successfully get buffer length.");
    Ok(buffer_length)
}

#[cfg(feature = "solo")]
pub fn gxi_get_buffer(feature_id: GX_FEATURE_ID) -> Result<Vec<u8>> {
    let gxi_device = gxi_get_device_handle()?;
    let mut buffer_size = gxi_get_buffer_length(feature_id)?;
    let mut buffer = vec![0u8; buffer_size];
    let status = gxi_check(|gxi| {
        gxi.gx_get_buffer(
            gxi_device,
            feature_id,
            buffer.as_mut_ptr(),
            &mut buffer_size,
        )
    })?;

    check_gx_status(status)?;
    println!("Successfully get buffer.");
    Ok(buffer)
}

#[cfg(feature = "solo")]
pub fn gxi_set_buffer(feature_id: GX_FEATURE_ID, buffer: &[u8]) -> Result<()> {
    let gxi_device = gxi_get_device_handle()?;
    let buffer_size = buffer.len();
    let status =
        gxi_check(|gxi| gxi.gx_set_buffer(gxi_device, feature_id, buffer.as_ptr(), buffer_size))?;

    check_gx_status(status)?;
    println!("Successfully set buffer.");
    Ok(())
}
