<p align="center" dir="auto">
    <img style="height:240px;width:240px"  src="https://s2.loli.net/2024/09/08/uDKESYW7ks9eRyf.png" alt="Logo逃走啦~"/>
</p>

<h1 align="center" tabindex="-1" class="heading-element" dir="auto">GXCI</h1>

<p align="center">
  <a href="https://crates.io/crates/gxci" target="_blank"><img src="https://img.shields.io/crates/v/gxci"/></a>
  <a href="https://docs.rs/gxci" target="_blank"><img src="https://img.shields.io/docsrs/gxci/0.3.7"/></a>
  <a href="https://github.com/islatri/gxci" target="_blank"><img src="https://img.shields.io/badge/License-MIT-green.svg"/></a>

</p>

<p align="center">
    Rust-based safe interface development for Daheng Industrial Camera GxIAPI
</p>

<hr />


# Now, the document site is available!

1. English: [https://hakochest.github.io/gxci-en/](https://hakochest.github.io/gxci-en/)
2. 中文: [https://hakochest.github.io/gxci-cn/](https://hakochest.github.io/gxci-cn/)


# Introduction
gxci(Galaxy Camera Interface)是一款用Rust基于大恒工业相机GxIAPI的库进行的接口开发;

目前已实现USB单相机的HAL库封装，raw内包含着C语言接口除去网络相机的全部内容(句柄、常量、结构、枚举、回调函数等)的直接rust实现；HAL内做了硬件抽象层的封装(目前包括连接、采图、推流)，适合实际开发使用；utils内则是一些工具类函数(常用的Builder模式和Facade模式函数封装);

Gxci (Galaxy Camera Interface) is an interface developed using Rust based on the Daxi API library of Daheng Industrial Camera; 

At present, HAL library encapsulation for USB single camera has been implemented, and raw contains a direct Rust implementation of all contents (handles, constants, structures, enumerations, callback functions, etc.) of the C language interface except for the network camera; HAL has encapsulated the hardware abstraction layer (currently including connections, image capture, and streaming), which is suitable for practical development and use; Inside the utilities are some utility class functions (encapsulated with commonly used Builder and Facade pattern functions);


# Overview
You can get the sdk-dev-doc from the SDK of Daheng Imaging you have installed.

# Quick Start
1. Ensure you have OpenCV Rust Bindings installed.
2. Ensure your camera version is supported by the GxIAPI SDK,and ensure you have installed the GxIAPI SDK.

the document of the environment config is follow the quick start part.

in your Cargo.toml, add the following dependencies:

```toml
[dependencies]
gxci = "0.3.7"
# It default with the feature "solo" and "use-opencv", and now there're only this two features can be used.
```

The solo feature can simplify some operation if you only have one camera, because it will default to the first camera in all functions.

then, you can use the following code to get a single image from the camera and save it as png.

```rust
use gxci::hal::device::*;
use gxci::hal::base::*;
use gxci::hal::control::analog::gxi_set_gain_auto_continuous;
use gxci::utils::debug::print_device_info;

fn main()->Result<()> {
    gxci_init_default()?;

    let device_num = gxi_count_devices(1000)?;
    println!("Device number: {}", device_num);

    let base_info = gxi_list_devices()?;
    for device in &base_info {
        print_device_info(&device);
    }
    
    gxi_open_device()?;

    gxi_set_gain_auto_continuous()?;

    gxi_get_image()?;

    gxi_save_image_as_png("test.png")?;

    gxi_close_device()?;

    gxci_close()?;

    Ok(())
}
```

The terminal output should be like this:

```powershell
PS C:\Users\Chest\Documents\GitHub\crate_zone\gxci> cargo run --example hal_get_image
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target\debug\examples\hal_get_image.exe`
Initializing GXI with DLL path: C:\Program Files\Daheng Imaging\GalaxySDK\APIDll\Win64\GxIAPI.dll
Device number: 1
p_device_info: 0x1d10264e0b0, p_buffer_size: 0x69120ff590
Vendor Name: Daheng Imaging
Model Name: MER-050-560U3C
Serial Number: KJ0180110048
Display Name: MER-050-560U3C(KJ0180110048)
Device ID: MER-050-560U3C(KJ0180110048)
User ID:
Access Status: GX_ACCESS_STATUS_UNKNOWN
Device Class: GX_DEVICE_CLASS_U3V
-----------------------
Successfully opened device index 1
Successfully sent command
int_value: 0x69120fefd8
int_value: 0x69120fefe0
enum_value: 0x69120feff8
enum_value: 0x69120fefe8
int_value: 0x69120feff0
p_frame_data: 0x69120ff448
frame_data: GX_FRAME_DATA { nStatus: 0, pImgBuf: 0x1d1042c6040, nWidth: 640, nHeight: 480, nPixelFormat: 17301513, nImgSize: 2457600, nFrameID: 0, nTimestamp: 0, reserved: [17301513] }
Successfully sent command
Successfully got image
Image saved successfully.
Successfully closed device
```

if your camera is as the following:

![图片逃走啦~](https://s2.loli.net/2024/09/08/QHiIKXd4c7MSegR.png)

then you will get a test.png as

![图片逃走啦~](https://s2.loli.net/2024/09/08/zxBCpr59XD74UIq.png)

more codes just see the examples and document.

# Platform Support
Now, is Windows only. 


# Licensing
Licensed under the MIT License.


# Contributing

If you have any questions, please issue it.

If you want to contribute, please fork it and pull request.

If you want to contact me, you can email me: zoneherobrine@gmail.com;

or by Tencent QQ: 2212540603 (with related information in the friend request)


# Acknowledgments
GXCI(GalaXy Camera Interface)的命名要感谢[MoonWX](https://github.com/MoonWX)同学的建议，这是一个简洁明确并且很帅的名字ヽ(・∀・)ﾉ；

同时也感谢同专业李同学的帮助，在gx_enum的冗长的类型转换中，他与我协力在十分钟之内完成了C枚举到Rust枚举的转换；

也感谢[西西](https://github.com/Ben-Phantom)帮忙找的免费图床网站，进一步压缩了包的大小( - ω - )

Also thanks to OpenAI's GPT model DELTA·E for drawing the cool LOGO :D


