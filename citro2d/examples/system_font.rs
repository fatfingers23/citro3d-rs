//! https://github.com/devkitPro/3ds-examples/blob/master/graphics/printing/system-font/source/main.c
#![feature(allocator_api)]

use citro2d::Point;
use citro2d::render::{Color, Target};
use citro2d::shapes::{Circle, CircleSolid, Ellipse, MultiColor, Rectangle, Triangle};
use citro2d::text::{Text, system_font};
use ctru::{prelude::*, services::gfx::TopScreen3D};

const SCREEN_WIDTH: u16 = 400;
const SCREEN_HEIGHT: u16 = 240;

fn main() {
    let gfx = Gfx::new().expect("Couldn't obtain GFX controller");
    let mut hid = Hid::new().expect("Couldn't obtain HID controller");
    let apt = Apt::new().expect("Couldn't obtain APT controller");

    let mut citro2d_instance = citro2d::Instance::new().expect("Couldn't obtain citro2d instance");
    let top_screen = TopScreen3D::from(&gfx.top_screen);
    let (top_left, _) = top_screen.split_mut();
    let mut top_target = Target::new(top_left).expect("failed to create render target");
    let bottom_screen = Console::new(gfx.bottom_screen.borrow_mut());

    let black = Color::new(0, 0, 0);

    let white = Color::new(255, 255, 255);

    // if let Err(e) = system_font() {
    //     panic!("Failed to load system font: {:?}", e);
    // }
    // let c2d_text_buffer = unsafe { citro2d_sys::C2D_TextBufNew(4096) };
    // let mut c2d_text = unsafe {
    //     citro2d_sys::C2D_Text {
    //         buf: c2d_text_buffer,
    //         begin: 0,
    //         end: 0,
    //         width: 0.0,
    //         lines: 0,
    //         words: 0,
    //         font: citro2d_sys::C2D_FontLoadSystem(0),
    //     }
    // };

    // unsafe {
    //     citro2d_sys::C2D_TextParse(&mut c2d_text, c2d_text_buffer, "DVD".as_ptr() as *const u8)
    // };
    // unsafe {
    //     citro2d_sys::C2D_TextOptimize(&c2d_text as *const _);
    // }

    // let system_font = system_font().expect("Failed to load system font");
    let mut dvd_text = Text::new().expect("Failed to create text buffer");
    let _ = dvd_text.parse_and_optimize("DVD");

    let mut x = 8.0;
    let mut y = 8.0;
    let mut dx = 1.0;
    let mut dy = 1.0;

    while apt.main_loop() {
        hid.scan_input();

        // Update position
        x += dx;
        y += dy;

        // // Check for collision with screen edges and reverse direction if needed
        // if x <= 0.0 || x + dvd_text.c2d_text.width > SCREEN_WIDTH as f32 {
        //     dx = -dx;
        // }
        // if y <= 0.0 || y + 16.0 > SCREEN_HEIGHT as f32 {
        //     dy = -dy;
        // }

        citro2d_instance.render_target(&mut top_target, |_instance, render_target| {
            render_target.clear(white);
            // dvd_text.clear_buffer();
            // dvd_text.draw(Point::new(x, y, 0.5), 1.0, 1.0);

            unsafe {
                // citro2d_sys::C2D_TextBufClear(dvd_text.buffer);
                citro2d_sys::C2D_DrawText(&dvd_text.c2d_text as *const _, 0, x, y, 0.5, 1.0, 1.0);
            }
        });

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }
    }
}
