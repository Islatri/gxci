// Placeholder
use std::sync::{Arc, LazyLock, Mutex};
use std::sync::{MutexGuard, PoisonError};

pub type Result<T> = core::result::Result<T, Error>;

// Error

pub struct Error {
    pub inner: Box<ErrorKind>,
}

impl Error {
    pub fn new(inner: ErrorKind) -> Self {
        Error {
            inner: Box::new(inner),
        }
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl std::error::Error for Error {}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self::new(kind)
    }
}

impl From<libloading::Error> for Error {
    fn from(error: libloading::Error) -> Self {
        Error::new(ErrorKind::LibLoadingError(error))
    }
}

impl From<std::ffi::NulError> for Error {
    fn from(err: std::ffi::NulError) -> Self {
        Error::new(ErrorKind::NulError(err))
    }
}

#[derive(Debug)]
pub enum MutexType {
    Gxi,
    FrameData,
    FrameCallback,
    Device,
}

pub trait MutexExt<T> {
    fn lock_safe(&self, mutex_type: MutexType) -> Result<MutexGuard<T>>;
}

impl<T> MutexExt<T> for LazyLock<Arc<Mutex<T>>> {
    fn lock_safe(&self, mutex_type: MutexType) -> Result<MutexGuard<T>> {
        self.lock().map_err(|e: PoisonError<MutexGuard<T>>| {
            Error::new(ErrorKind::MutexPoisonError {
                mutex_type,
                message: format!("{:?}", e),
            })
        })
    }
}

// ErrorKind

pub enum ErrorKind {
    InvalidFeatureType(String),
    NulError(std::ffi::NulError),
    LibLoadingError(libloading::Error),
    GxiError(String),
    GxStatusError(i32),
    DeviceHandleError(String),
    FrameDataError(String),
    MutexPoisonError {
        mutex_type: MutexType,
        message: String,
    },
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorKind::InvalidFeatureType(e) => write!(f, "InvalidFeatureType: {:?}", e),
            ErrorKind::NulError(e) => write!(f, "NulError: {:?}", e),
            ErrorKind::DeviceHandleError(e) => write!(f, "DeviceHandleError: {:?}", e),
            ErrorKind::LibLoadingError(e) => write!(f, "LibLoadingError: {:?}", e),
            ErrorKind::GxStatusError(e) => write!(f, "GxStatusError: {:?}", e),
            ErrorKind::GxiError(e) => write!(f, "GxiError: {:?}", e),
            ErrorKind::FrameDataError(e) => write!(f, "FrameDataError: {:?}", e),
            ErrorKind::MutexPoisonError {
                mutex_type,
                message,
            } => {
                write!(f, "MutexPoisonError: {:?}, {:?}", mutex_type, message)
            }
        }
    }
}

impl std::fmt::Debug for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorKind::InvalidFeatureType(e) => write!(f, "InvalidFeatureType: {:?}", e),
            ErrorKind::NulError(e) => write!(f, "NulError: {:?}", e),
            ErrorKind::DeviceHandleError(e) => write!(f, "DeviceHandleError: {:?}", e),
            ErrorKind::LibLoadingError(e) => write!(f, "LibLoadingError: {:?}", e),
            ErrorKind::GxStatusError(e) => write!(f, "GxStatusError: {:?}", e),
            ErrorKind::GxiError(e) => write!(f, "GxiError: {:?}", e),
            ErrorKind::FrameDataError(e) => write!(f, "FrameDataError: {:?}", e),
            ErrorKind::MutexPoisonError {
                mutex_type,
                message,
            } => {
                write!(f, "MutexPoisonError: {:?}, {:?}", mutex_type, message)
            }
        }
    }
}
