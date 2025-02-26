#![feature(custom_test_frameworks)]
#![test_runner(test_runner::run_gdb)]
#![feature(doc_cfg)]
#![feature(doc_auto_cfg)]
#![doc(html_root_url = "https://rust3ds.github.io/citro3d-rs/crates")]
#![doc(
    html_favicon_url = "https://user-images.githubusercontent.com/11131775/225929072-2fa1741c-93ae-4b47-9bdf-af70f3d59910.png"
)]
#![doc(
    html_logo_url = "https://user-images.githubusercontent.com/11131775/225929072-2fa1741c-93ae-4b47-9bdf-af70f3d59910.png"
)]

//! Safe Rust bindings to `citro2d`. This crate wraps `citro2d-sys` to provide
//! safer APIs for graphics programs targeting the 3DS.
//!
//! ## Feature flags
#![doc = document_features::document_features!()]

pub mod error;

use citro2d_sys::C2D_DEFAULT_MAX_OBJECTS;
// use citro2d_sys::C2D_DEFAULT_MAX_OBJECTS;
pub use error::{Error, Result};

/// The single instance for using `citro2d`. This is the base type that an application
/// should instantiate to use this library.
#[non_exhaustive]
#[must_use]
pub struct Instance {}

impl Instance {
    /// Create a new instance of `citro2d`.
    /// This also initializes `citro3d` since it is required for `citro2d`.
    pub fn new() -> Result<Self> {
        //TODO prob need to save this instance somewhere in struct
        let _ = citro3d::Instance::new().expect("failed to initialize Citro3D");
        Self::with_max_objects(C2D_DEFAULT_MAX_OBJECTS.try_into().unwrap())
    }

    /// You have to initialize citro3d before using citro2d, but some cases you may
    /// Have initialized citro3d already, so you can use this function to initialize
    pub fn new_without_c3d_init() -> Result<Self> {
        Self::with_max_objects(C2D_DEFAULT_MAX_OBJECTS.try_into().unwrap())
    }

    #[doc(alias = "C2D_Init")]
    pub fn with_max_objects(max_objects: usize) -> Result<Self> {
        //TODO add this here but the docs read like it may need to be called again if it switches between 2d and 3d?
        // citro2d_sys::C2D_Prepare();
        if unsafe { citro2d_sys::C2D_Init(max_objects) } {
            Ok(Self {})
        } else {
            Err(Error::FailedToInitialize)
        }
    }
}
