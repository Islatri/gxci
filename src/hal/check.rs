//! Check module for COMMON error handling.

use crate::error::{Error, ErrorKind, Result};

pub fn check_status<F>(status: i32, error_fn: F) -> Result<()>
where
    F: Fn(i32) -> Error,
{
    if status == 0 {
        Ok(())
    } else {
        Err(error_fn(status))
    }
}

pub fn check_status_with_ok_fn<S, F>(status: i32, ok_fn: S, error_fn: F) -> Result<()>
where
    S: FnOnce() -> Result<()>,
    F: Fn(i32) -> Error,
{
    if status == 0 {
        ok_fn()?;
        Ok(())
    } else {
        Err(error_fn(status))
    }
}

pub fn check_gx_status(status: i32) -> Result<()> {
    let gx_error_fn = |status| Error::new(ErrorKind::GxStatusError(status));
    check_status(status, gx_error_fn)
}

pub fn check_gx_status_with_ok_fn<S>(status: i32, ok_fn: S) -> Result<()>
where
    S: FnOnce() -> Result<()>,
{
    let gx_error_fn = |status| Error::new(ErrorKind::GxStatusError(status));
    check_status_with_ok_fn(status, ok_fn, gx_error_fn)
}
