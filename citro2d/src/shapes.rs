use crate::{Point, Size, render::Color};

pub struct MultiColor {
    pub top_left: Color,
    pub top_right: Color,
    pub bottom_left: Color,
    pub bottom_right: Color,
}

pub trait Shape {
    fn render(&self) -> bool;
}

pub struct Rectangle {
    pub point: Point,
    pub size: Size,
    pub multi_color: MultiColor,
}

impl Shape for Rectangle {
    #[doc(alias = "C2D_DrawRectangle")]
    fn render(&self) -> bool {
        unsafe {
            citro2d_sys::C2D_DrawRectangle(
                self.point.x,
                self.point.y,
                self.point.z,
                self.size.width,
                self.size.height,
                self.multi_color.top_left.into(),
                self.multi_color.top_right.into(),
                self.multi_color.bottom_left.into(),
                self.multi_color.bottom_right.into(),
            )
        }
    }
}

pub struct RectangleSolid {
    pub point: Point,
    pub size: Size,
    pub color: Color,
}

impl Shape for RectangleSolid {
    #[doc(alias = "C2D_DrawRectSolid")]
    fn render(&self) -> bool {
        unsafe {
            citro2d_sys::C2D_DrawRectSolid(
                self.point.x,
                self.point.y,
                self.point.z,
                self.size.width,
                self.size.height,
                self.color.into(),
            )
        }
    }
}

pub struct Triangle {
    pub point0: Point,
    pub color0: Color,
    pub point1: Point,
    pub color1: Color,
    pub point2: Point,
    pub color2: Color,
    pub depth: f32,
}

impl Shape for Triangle {
    #[doc(alias = "C2D_DrawTriangle")]
    fn render(&self) -> bool {
        unsafe {
            citro2d_sys::C2D_DrawTriangle(
                self.point0.x,
                self.point0.y,
                self.color0.into(),
                self.point1.x,
                self.point1.y,
                self.color1.into(),
                self.point2.x,
                self.point2.y,
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
