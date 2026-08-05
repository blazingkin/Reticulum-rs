#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(feature = "std", feature = "no_std"))]
compile_error!("features 'std' and 'no_std' cannot be enabled at the same time");

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod buffer;
pub mod crypt;
pub mod destination;
pub mod error;
pub mod hash;
pub mod identity;
pub mod packet;
pub mod serde;
pub mod time;

#[cfg(not(feature = "std"))]
pub use self::time::init;
