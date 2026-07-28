#![no_std]

#[cfg(feature = "alloc")]
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

/// Initialize the UNIX time
///
/// Reticulum needs to know the current UNIX timestamp in order to create
/// announce packets.
///
/// On the `std` build, the system clock is used and this function has no
/// effect.
///
/// On the `no_std` build, it must be called before any destinations are
/// created. Creating a destination without having called this function
/// first will panic.
///
/// `unix_now`: the current UNIX timestamp in seconds.
#[allow(unused)]
pub fn init(unix_now: u64) {
    #[cfg(not(feature = "std"))]
    crate::time::init(unix_now);
}
