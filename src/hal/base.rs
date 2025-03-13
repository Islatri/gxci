//! Base functions for the HAL. Such as lib checking, initialization and closing.
pub use crate::error::Result;
use crate::error::{Error, ErrorKind, MutexExt, MutexType};
use crate::raw::gx_interface::*;
use std::sync::{Arc, LazyLock, Mutex};

pub static GXI: LazyLock<Arc<Mutex<Option<GXInstance>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(None)));

// 添加通用的 Mutex 检查函数
pub fn gxi_check<T, F>(func: F) -> Result<T>
where
    F: FnOnce(&GXInstance) -> Result<T>,
{
    let gxi = GXI.lock_safe(MutexType::Gxi)?;
    if let Some(gxi) = gxi.as_ref() {
        func(gxi)
    } else {
        Err(Error::new(ErrorKind::GxiError(format!(
            "GXI is None while gxi_check(). Please check your gxci_init situation."
        ))))
    }
}

pub fn gxci_init(dll_path: &str) -> Result<()> {
    println!("Initializing GXI with DLL path: {}", dll_path);

    let mut gxi = GXI.lock_safe(MutexType::Gxi)?;
    if gxi.is_some() {
        println!("Warning: GXI is already initialized. Reinitializing.");
    }

    *gxi = Some(GXInstance::new(dll_path)?);
    gxi.as_ref().unwrap().gx_init_lib()?;
    Ok(())
}

pub fn gxci_init_default() -> Result<()> {
    let dll_path_default =
        "C:\\Program Files\\Daheng Imaging\\GalaxySDK\\APIDll\\Win64\\GxIAPI.dll";

    let mut gxi = GXI.lock_safe(MutexType::Gxi)?;
    if gxi.is_some() {
        println!("Warning: GXI is already initialized. Reinitializing.");
    }

    *gxi = Some(GXInstance::new(dll_path_default)?);
    gxi.as_ref().unwrap().gx_init_lib()?;
    Ok(())
}

pub fn gxci_close() -> Result<()> {
    gxi_check(|gxi| gxi.gx_close_lib())?;

    let mut gxi = GXI.lock_safe(MutexType::Gxi)?;
    *gxi = None;
    Ok(())
}
