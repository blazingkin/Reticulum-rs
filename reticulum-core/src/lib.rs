#![cfg_attr(not(feature = "std"), no_std)]

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
