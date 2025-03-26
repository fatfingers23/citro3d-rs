use std::fmt::Error;

// pub fn text_parse() {
//     unsafe { citro2d_sys::C2D_TextParse(text, buf, str_) }
// }
use citro2d_sys::{C2D_Font, C2D_Text, C2D_TextBuf};

use crate::Point;

pub struct Text {
    pub buffer: C2D_TextBuf,
    font: C2D_Font,
    pub c2d_text: C2D_Text,
}

#[derive(Debug)]
pub enum TextError {
    FailedToInitializeBuffer,
    FailedToInitializeFont,
    TextParseError,
}

pub fn system_font() -> Result<C2D_Font, TextError> {
    let system_font = unsafe { citro2d_sys::C2D_FontLoadSystem(1) };
    if system_font.is_null() {
        return Err(TextError::FailedToInitializeFont);
    }
    Ok(system_font)
}

impl Text {
    pub fn new() -> Result<Self, TextError> {
        let system_font = unsafe { citro2d_sys::C2D_FontLoadSystem(1) };

        Self::new_with_size(4096, system_font)
    }

    pub fn new_with_size(size: usize, font: C2D_Font) -> Result<Self, TextError> {
        let buffer = unsafe { citro2d_sys::C2D_TextBufNew(size) };
        if buffer.is_null() {
            return Err(TextError::FailedToInitializeBuffer);
        }

        let c2d_text = C2D_Text {
            buf: buffer,
            //Just setting defaults I think citro2d takes over and sets these
            begin: 0,
            end: 0,
            width: 0.0,
            lines: 0,
            words: 0,
            font,
        };

        Ok(Self {
            buffer,
            font,
            c2d_text,
        })
    }

    pub fn parse_and_optimize(&mut self, text: &str) -> Result<(), TextError> {
        let parse = unsafe {
            citro2d_sys::C2D_TextParse(&mut self.c2d_text, self.buffer, text.as_ptr() as *const u8)
        };
        // if parse.is_null() {
        //     return Err(TextError::TextParseError);
        // }
        unsafe {
            citro2d_sys::C2D_TextOptimize(&self.c2d_text);
        }

        Ok(())
    }

    pub fn clear_buffer(&mut self) {
        unsafe {
            citro2d_sys::C2D_TextBufClear(self.buffer);
        }
    }

    pub fn draw(&self, point: Point, scale_x: f32, scale_y: f32) {
        unsafe {
            citro2d_sys::C2D_DrawText(
                &self.c2d_text,
                0,
                point.x,
                point.y,
                point.x,
                scale_x,
                scale_y,
            );
        }
    }

    // pub fn optimize(&mut self) {
    //     unsafe {
    //         citro2d_sys::C2D_TextOptimize(&self.buffer as *const _);
    //     }
    // }
}
