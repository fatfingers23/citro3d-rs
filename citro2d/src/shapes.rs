use crate::render::Color;

pub trait Shape {
    fn render(&self) -> bool;
}

pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub width: f32,
    pub height: f32,
    pub color_top_left: Color,
    pub color_top_right: Color,
    pub color_bottom_left: Color,
    pub color_bottom_right: Color,
}

impl Shape for Rectangle {
    fn render(&self) -> bool {
        unsafe {
            citro2d_sys::C2D_DrawRectangle(
                self.x,
                self.y,
                self.z,
                self.width,
                self.height,
                self.color_top_left.into(),
                self.color_top_right.into(),
                self.color_bottom_left.into(),
                self.color_bottom_right.into(),
            )
        }
    }
}

pub struct RectangleSolid {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub width: f32,
    pub height: f32,
    pub color: Color,
}

impl Shape for RectangleSolid {
    fn render(&self) -> bool {
        unsafe {
            citro2d_sys::C2D_DrawRectSolid(
                self.x,
                self.y,
                self.z,
                self.width,
                self.height,
                self.color.into(),
            )
        }
    }
}
