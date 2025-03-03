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
    pub top: Point,
    pub top_color: Color,
    pub left: Point,
    pub left_color: Color,
    pub right: Point,
    pub right_color: Color,
    pub depth: f32,
}

impl Shape for Triangle {
    #[doc(alias = "C2D_DrawTriangle")]
    fn render(&self) -> bool {
        unsafe {
            citro2d_sys::C2D_DrawTriangle(
                self.top.x,
                self.top.y,
                self.top_color.into(),
                self.left.x,
                self.left.y,
                self.left_color.into(),
                self.right.x,
                self.right.y,
                self.right_color.into(),
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
