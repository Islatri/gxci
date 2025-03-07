#![doc = include_str!("../Docsrs.md")]
pub mod raw;
pub mod hal;
pub mod utils;
pub mod error;

#[cfg(feature = "use-opencv")]
pub use opencv;

#[cfg(feature = "use-imageproc")]
pub use imageproc;