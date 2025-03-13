use gxci::hal::base::*;
use gxci::hal::config::{gxi_get_enum_description, gxi_get_float_range};
use gxci::hal::control::analog::*;
use gxci::hal::device::*;
use gxci::raw::gx_enum::GX_FEATURE_ID;
use gxci::utils::debug::print_device_info;

fn main() -> Result<()> {
    gxci_init_default()?;

    let device_num = gxi_count_devices(1000)?;
    println!("Device number: {}", device_num);

    let base_info = gxi_list_devices()?;
    for device in &base_info {
        print_device_info(&device);
    }

    gxi_open_device()?;

    let gain_range = gxi_get_float_range(GX_FEATURE_ID::GX_FLOAT_GAIN)?;
    println!("{:?}", gain_range);
    // 0 - 16

    let gain = 10.0;
    gxi_set_gain(gain)?;
    gxi_get_image()?;
    gxi_save_image_as_png("gain10.png")?;

    gxi_get_gain()?;
    let gain = 16.0;
    gxi_set_gain(gain)?;
    gxi_get_image()?;
    gxi_save_image_as_png("gain16.png")?;

    let gain_autos = gxi_get_enum_description(GX_FEATURE_ID::GX_ENUM_GAIN_AUTO)?;
    for gain_auto in gain_autos {
        println!("{:?}", gain_auto);
    }

    gxi_set_gain_auto_continuous()?;
    gxi_get_image()?;
    gxi_save_image_as_png("gain_auto.png")?;

    gxi_close_device()?;

    gxci_close()?;
    Ok(())
}
