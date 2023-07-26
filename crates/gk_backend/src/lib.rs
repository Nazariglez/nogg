#[cfg(not(feature = "winit"))]
mod empty;

#[cfg(not(feature = "winit"))]
pub use crate::empty::*;

#[cfg(feature = "winit")]
mod winit;

#[cfg(feature = "winit")]
pub use crate::winit::*;

mod platform;
mod config;

pub use platform::*;
pub use config::*;