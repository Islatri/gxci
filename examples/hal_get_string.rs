use gxci::hal::base::*;
use gxci::hal::control::device::*;
use gxci::hal::control::image_format::*;
use gxci::hal::device::*;
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

    let name = gxi_get_device_vendor_name()?;
    let model = gxi_get_device_model_name()?;
    let version = gxi_get_device_version()?;
    let firmware_version = gxi_get_device_firmware_version()?;
    let serial_number = gxi_get_device_serial_number()?;
    println!("Vendor name: {}", name);
    println!("Model name: {}", model);
    println!("Version: {}", version);
    println!("Firmware version: {}", firmware_version);
    println!("Serial number: {}", serial_number);

    let width = gxi_get_width()?;
    let height = gxi_get_height()?;
    println!("Width: {}", width);
    println!("Height: {}", height);

    // gxi_set_width(640)?;
    // gxi_set_height(480)?;

    // let width = gxi_get_width()?;
    // let height = gxi_get_height()?;
    // println!("Width: {}", width);
    // println!("Height: {}", height);

    gxi_close_device()?;

    gxci_close()?;

    Ok(())
}
