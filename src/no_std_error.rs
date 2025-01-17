#![allow(missing_docs)]

use core::fmt::{Debug, Display};

pub trait Error: Debug + Display {
    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }
    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

macro_rules! impl_error {
    ($($e:path),*) => {
        $(
            impl Error for $e {}
        )*
    }
}

// All errors supported by our minimum suported Rust version can be supported by
// default.
impl_error![
    core::str::ParseBoolError,    // 1.0
    core::str::Utf8Error,         // 1.0
    core::num::ParseIntError,     // 1.0
    core::num::ParseFloatError,   // 1.0
    core::char::DecodeUtf16Error, // 1.9
    core::fmt::Error,             // 1.11
    core::cell::BorrowMutError,   // 1.13
    core::cell::BorrowError,      // 1.13
    core::char::ParseCharError    // 1.20
];

// We can gate these together with std futures.
#[cfg(feature = "unstable-futures")]
impl_error![
    core::num::TryFromIntError,     // 1.34
    core::array::TryFromSliceError, // 1.34
    core::char::CharTryFromError    // 1.34
];
