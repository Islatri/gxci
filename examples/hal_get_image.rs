use gxci::hal::device::*;
use gxci::hal::base::*;
use gxci::utils::debug::print_device_info;

fn main()->Result<()> {
    let dll_path = "C:\\Program Files\\Daheng Imaging\\GalaxySDK\\APIDll\\Win64\\GxIAPI.dll"; // 假设这是一个测试用的 DLL 路径
    gxci_init(dll_path)?;

    let device_num = gxi_count_devices( 1000)?;
    println!("Device number: {}", device_num);

    let base_info = gxi_list_devices()?;
    for device in &base_info {
        print_device_info(&device);
    }
    
    gxi_open_device()?;

    gxi_get_image()?;
    
    gxi_save_image_as_png("test.png")?;

    gxi_close_device()?;

    gxci_close()?;
    Ok(())
}