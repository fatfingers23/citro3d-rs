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
pub mod render;
pub mod shapes;
use citro2d_sys::C2D_DEFAULT_MAX_OBJECTS;
pub use error::{Error, Result};
use render::Target;

/// The single instance for using `citro2d`. This is the base type that an application
/// should instantiate to use this library.
#[non_exhaustive]
#[must_use]
pub struct Instance {
    pub citro3d_instance: citro3d::Instance,
}

impl Instance {
    /// Create a new instance of `citro2d`.
    /// This also initializes `citro3d` since it is required for `citro2d`.
    pub fn new() -> Result<Self> {
        //TODO prob need to save this instance somewhere in struct. Appears it's used in the 3d implementation
        let citro3d_instance = citro3d::Instance::new().expect("failed to initialize Citro3D");
        let citro2d = Self::with_max_objects(
            C2D_DEFAULT_MAX_OBJECTS.try_into().unwrap(),
            citro3d_instance,
        );

        citro2d
    }

    /// You have to initialize citro3d before using citro2d, but some cases you may
    /// Have initialized citro3d already, so you can use this function to initialize
    /// You pass in the citro3d instance you already initialized to ensure it's lifetime is the same as citro2d
    /// **Note** The above statement may not work, and may not be able to switch between the two without api changes
    pub fn new_without_c3d_init(citro3d_instance: citro3d::Instance) -> Result<Self> {
        Self::with_max_objects(
            C2D_DEFAULT_MAX_OBJECTS.try_into().unwrap(),
            citro3d_instance,
        )
    }

    #[doc(alias = "C2D_Init")]
    pub fn with_max_objects(
        max_objects: usize,
        citro3d_instance: citro3d::Instance,
    ) -> Result<Self> {
        let new_citro_2d = match unsafe { citro2d_sys::C2D_Init(max_objects) } {
            true => Ok(Self {
                citro3d_instance: citro3d_instance,
            }),
            false => Err(Error::FailedToInitialize),
        };
        unsafe { citro2d_sys::C2D_Prepare() };
        //TODO add this here but the docs read like it may need to be called again if it switches between 2d and 3d?
        new_citro_2d
    }

    pub fn render_target<F>(&mut self, target: &mut Target<'_>, f: F)
    where
        F: FnOnce(&Self, &mut Target<'_>),
    {
        unsafe {
            citro3d_sys::C3D_FrameBegin(citro3d_sys::C3D_FRAME_SYNCDRAW);
            citro2d_sys::C2D_SceneBegin(target.raw);
            f(self, target);
            citro3d_sys::C3D_FrameEnd(0);
        }
    }

    /// Returns some stats about the 3Ds's graphics
    pub fn get_3d_stats(&self) -> Citro3DStats {
        //TODO should i check for NaN? Seems like that's like null?
        let processing_time_f32 = unsafe { citro3d_sys::C3D_GetProcessingTime() };
        let drawing_time_f32 = unsafe { citro3d_sys::C3D_GetDrawingTime() };
        //Seems more info for this from C3D_Context?
        let cmd_buf_usage_f32 = unsafe { citro3d_sys::C3D_GetCmdBufUsage() };
        Citro3DStats {
            processing_time: processing_time_f32,
            drawing_time: drawing_time_f32,
            cmd_buf_usage: cmd_buf_usage_f32,
        }
    }
}

/// Stats about the 3Ds's graphics
pub struct Citro3DStats {
    pub processing_time: f32,
    pub drawing_time: f32,
    pub cmd_buf_usage: f32,
}
