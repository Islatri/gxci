use gxci::hal::base::*;
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

    let raw = gxi_get_image_as_raw()?;

    println!("{:?}", raw);

    gxi_save_image_as_png("test.png")?;

    gxi_close_device()?;

    gxci_close()?;

    Ok(())
}
