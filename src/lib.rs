#![doc = include_str!("../Docsrs.md")]
pub mod error;
pub mod hal;
pub mod raw;
pub mod utils;

#[cfg(feature = "use-opencv")]
pub use opencv;

#[cfg(feature = "use-imageproc")]
pub use imageproc;
