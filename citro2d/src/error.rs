//! General-purpose error and result types returned by public APIs of this crate.

// use std::ffi::NulError;
// use std::num::TryFromIntError;
// use std::sync::TryLockError;

/// The common result type returned by `citro3d` functions.
pub type Result<T> = std::result::Result<T, Error>;

// TODO probably want a similar type to ctru::Result to make it easier to convert
// nonzero result codes to errors.

/// The common error type that may be returned by `citro3d` functions.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// A C2D object or context could not be initialized.
    FailedToInitialize,
}

// impl From<TryFromIntError> for Error {
//     fn from(_: TryFromIntError) -> Self {
//         Self::InvalidSize
//     }
// }

// impl<T> From<TryLockError<T>> for Error {
//     fn from(_: TryLockError<T>) -> Self {
//         Self::LockHeld
//     }
// }

// impl From<NulError> for Error {
//     fn from(_: NulError) -> Self {
//         Self::InvalidName
//     }
// }
