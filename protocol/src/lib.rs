//! Simple packet-based protocol definitions in Rust.
//!
//! * The `Parcel` trait defines any type that can be serialized
//!   to a connection.
//! * The `wire` module deals with transmission of `Parcel`s.

pub use self::parcel::Parcel;
pub use self::errors::{Error, ErrorKind, ResultExt, CharTryFromError, TryFromIntError};
pub use self::settings::*;

// Must go first because it defines common macros.
#[macro_use]
mod packet;

mod settings;
#[macro_use]
pub mod types;
#[macro_use]
pub mod wire;

mod errors;
mod parcel;

extern crate byteorder;
extern crate flate2;
#[macro_use]
extern crate error_chain;

#[cfg(feature = "uuid")]
extern crate uuid;
extern crate num_traits;

#[cfg(feature = "tokio")]
extern crate tokio;

/// The default byte ordering.
pub type DefaultByteOrder = ::byteorder::BigEndian;

