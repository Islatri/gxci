//! Placeholder, but I found that no feature_id here

// use crate::hal::config::*;
// use crate::raw::gx_enum::GX_FEATURE_ID;
// use crate::error::Result;

// - [ ] gxi_get_user_output_value()
// - [ ] gxi_set_user_output_value()

// #[cfg(feature = "solo")]
// pub fn gxi_get_user_output_value() -> Result<bool> {
//     let user_output_value:bool = gxi_get_feature_value(GX_FEATURE_ID::GX_BOOL_USER_OUTPUT_VALUE)?;
//     println!("Now, user output value is {}",user_output_value);
//     Ok(user_output_value)
// }

// #[cfg(feature = "solo")]
// pub fn gxi_set_user_output_value(user_output_value:bool) -> Result<()> {
//     gxi_set_feature_value(GX_FEATURE_ID::GX_BOOL_USER_OUTPUT_VALUE, &user_output_value)?;
//     println!("Successfully set user output value to {}",user_output_value);
//     Ok(())
// }
