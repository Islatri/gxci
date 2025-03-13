use gxci::hal::base::*;
use gxci::hal::device::*;
use gxci::utils::debug::print_device_info;

fn main() -> Result<()> {
    let dll_path = "C:\\Program Files\\Daheng Imaging\\GalaxySDK\\APIDll\\Win64\\GxIAPI.dll";
    gxci_init(dll_path)?;

    let device_num = gxi_count_devices(1000)?;
    println!("Device number: {}", device_num);

    let base_info = gxi_list_devices()?;
    for device in &base_info {
        print_device_info(&device);
    }

    gxci_close()?;

    Ok(())
}
