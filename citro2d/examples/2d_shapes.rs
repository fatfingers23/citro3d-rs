//! This example demonstrates the most basic usage of `citro2d`: rendering shapes
//! on the top screen of the 3DS.

#![feature(allocator_api)]

use citro2d::render::{Color, Target};
use citro2d::shapes::{self, Rectangle, Shape};
use ctru::{
    prelude::*,
    services::gfx::{BottomScreen, TopScreen3D},
};

fn main() {
    let gfx = Gfx::new().expect("Couldn't obtain GFX controller");
    let mut hid = Hid::new().expect("Couldn't obtain HID controller");
    let apt = Apt::new().expect("Couldn't obtain APT controller");

    let mut citro2d_instance = citro2d::Instance::new().expect("Couldn't obtain citro2d instance");
    let top_screen = TopScreen3D::from(&gfx.top_screen);
    let (mut top_left, _) = top_screen.split_mut();
    let mut top_target = Target::new(top_left).expect("failed to create render target");

    let bottom_screen = Console::new(gfx.bottom_screen.borrow_mut());
    let clrWhite = Color::new_with_alpha(0xFF, 0xFF, 0xFF, 0xFF);
    let clrGreen = Color::new_with_alpha(0x00, 0xFF, 0x00, 0xFF);
    let clrRed = Color::new_with_alpha(0xFF, 0x00, 0x00, 0xFF);
    let clrBlue = Color::new_with_alpha(0x00, 0x00, 0xFF, 0xFF);
    let clrCircle1 = Color::new_with_alpha(0xFF, 0x00, 0xFF, 0xFF);
    let clrCircle2 = Color::new_with_alpha(0xFF, 0xFF, 0x00, 0xFF);
    let clrCircle3 = Color::new_with_alpha(0x00, 0xFF, 0xFF, 0xFF);
    let clrSolidCircle = Color::new_with_alpha(0x68, 0xB0, 0xD8, 0xFF);
    let clrTri1 = Color::new_with_alpha(0xFF, 0x15, 0x00, 0xFF);
    let clrTri2 = Color::new_with_alpha(0x27, 0x69, 0xE5, 0xFF);
    let clrRec1 = Color::new_with_alpha(0x9A, 0x6C, 0xB9, 0xFF);
    let clrRec2 = Color::new_with_alpha(0xFF, 0xFF, 0x2C, 0xFF);
    let clrRec3 = Color::new_with_alpha(0xD8, 0xF6, 0x0F, 0xFF);
    let clrRec4 = Color::new_with_alpha(0x40, 0xEA, 0x87, 0xFF);
    let clrClear = Color::new_with_alpha(0xFF, 0xD8, 0xB0, 0x68);

    while apt.main_loop() {
        hid.scan_input();

        citro2d_instance.render_target(&mut top_target, |_instance, render_target| unsafe {
            render_target.clear(clrClear);

            render_target.render_2d_shape(&Rectangle {
                x: 350.0,
                y: 0.0,
                z: 0.0,
                width: 50.0,
                height: 50.0,
                color_top_left: clrRec1,
                color_top_right: clrRec2,
                color_bottom_left: clrRec3,
                color_bottom_right: clrRec4,
            });

            // green_rectangle.render();
        });

        let stats = citro2d_instance.get_3d_stats();
        bottom_screen.select();
        println!("\x1b[1;1HSimple citro2d shapes example");
        println!("\x1b[2;1HCPU: {:6.2}%", stats.processing_time * 6.0);
        println!("\x1b[3;1HGPU: {:6.2}%", stats.drawing_time * 6.0);
        println!("\x1b[4;1HCmdBuf: {:6.2}%", stats.cmd_buf_usage * 100.0);

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        //Uncomment to cap fps
        // gfx.wait_for_vblank();
    }
}
