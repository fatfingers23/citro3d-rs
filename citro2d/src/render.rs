use std::cell::RefMut;

use ctru::services::gfx::Screen;

use crate::{Error, Result, shapes::Shape};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    // inner_rgb: Rgba<u8>,
    pub inner: u32,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self::new_with_alpha(r, g, b, 255)
    }

    pub fn new_with_alpha(r: u8, g: u8, b: u8, a: u8) -> Self {
        let inner = r as u32 | (g as u32) << 8 | (b as u32) << 16 | (a as u32) << 24;
        Self { inner }
    }
}

impl Into<Color> for u32 {
    fn into(self) -> Color {
        Color { inner: self }
    }
}

impl From<Color> for u32 {
    fn from(color: Color) -> u32 {
        color.inner
    }
}

/// HACK Not able to use citro3d's target because citro2d had to import citro3d library in build.rs
/// While citro3d already imports it's version to so the two different bindings are seen as different
/// Possible solution one crate? Fix build.rs to use from citro3d-sys?
#[doc(alias = "C3D_RenderTarget")]
pub struct Target<'screen> {
    pub raw: *mut citro2d_sys::C3D_RenderTarget_tag,
    // This is unused after construction, but ensures unique access to the
    // screen this target writes to during rendering
    _phantom_screen: RefMut<'screen, dyn Screen>,
}

impl<'screen> Target<'screen> {
    /// Create a new render target with the given parameters. This takes a
    /// [`RenderQueue`] parameter to make sure this  [`Target`] doesn't outlive
    /// the render queue.
    pub fn new(screen: RefMut<'screen, dyn Screen>) -> Result<Self> {
        let raw =
            unsafe { citro2d_sys::C2D_CreateScreenTarget(screen.as_raw(), screen.side().into()) };

        if raw.is_null() {
            return Err(Error::FailedToInitialize);
        }

        Ok(Self {
            raw,
            _phantom_screen: screen,
        })
    }

    pub fn clear(&mut self, color: Color) {
        unsafe {
            citro2d_sys::C2D_TargetClear(self.raw, color.inner);
        }
    }

    pub fn render_2d_shape<S>(&self, shape: &S)
    where
        S: Shape,
    {
        shape.render();
    }
}
