#[cfg(feature = "std")]
mod inner {
    pub use std::fmt::{self, Display, Debug, Formatter};
    pub use std::mem::replace;
    pub use std::str::FromStr;
    pub use std::vec::Vec;
    pub use std::string::String;
}

#[cfg(not(feature = "std"))]
mod inner {
    pub extern crate alloc;

    pub use core::write;
    pub use core::fmt::{self, Display, Debug, Formatter};
    pub use core::mem::replace;
    pub use core::str::FromStr;
    pub use alloc::vec::Vec;
    pub use alloc::string::String;
}

pub use inner::*;
