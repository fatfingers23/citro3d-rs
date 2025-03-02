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

pub struct Triangle {
    pub x0: f32,
    pub y0: f32,
    pub color0: Color,
    pub x1: f32,
    pub y1: f32,
    pub color1: Color,
    pub x2: f32,
    pub y2: f32,
    pub color2: Color,
    pub depth: f32,
}

impl Shape for Triangle {
    fn render(&self) -> bool {
        unsafe {
            citro2d_sys::C2D_DrawTriangle(
                self.x0,
                self.y0,
                self.color0.into(),
                self.x1,
                self.y1,
                self.color1.into(),
                self.x2,
                self.y2,
                self.color2.into(),
                self.depth,
            )
        }
    }
}

pub struct Ellipse {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub width: f32,
    pub height: f32,
    pub top_left_color: Color,
    pub top_right_color: Color,
    pub bottom_left_color: Color,
    pub bottom_right_color: Color,
}

impl Shape for Ellipse {
    fn render(&self) -> bool {
        unsafe {
            citro2d_sys::C2D_DrawEllipse(
                self.x,
                self.y,
                self.z,
                self.width,
                self.height,
                self.top_left_color.into(),
                self.top_right_color.into(),
                self.bottom_left_color.into(),
                self.bottom_right_color.into(),
            )
        }
    }
}

pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub radius: f32,
    pub top_left_color: Color,
    pub top_right_color: Color,
    pub bottom_left_color: Color,
    pub bottom_right_color: Color,
}

impl Shape for Circle {
    fn render(&self) -> bool {
        unsafe {
            citro2d_sys::C2D_DrawCircle(
                self.x,
                self.y,
                self.z,
                self.radius,
                self.top_left_color.into(),
                self.top_right_color.into(),
                self.bottom_left_color.into(),
                self.bottom_right_color.into(),
            )
        }
    }
}

pub struct CircleSolid {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub radius: f32,
    pub color: Color,
}

impl Shape for CircleSolid {
    fn render(&self) -> bool {
        unsafe {
            citro2d_sys::C2D_DrawCircleSolid(self.x, self.y, self.z, self.radius, self.color.into())
        }
    }
}
