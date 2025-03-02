//! This example demonstrates the most basic usage of `citro2d`: rendering shapes
//! on the top screen of the 3DS.

#![feature(allocator_api)]

use citro2d::render::{Color, Target};
use citro2d::shapes::{self, Circle, CircleSolid, Ellipse, Rectangle, Shape, Triangle};
use ctru::{
    prelude::*,
    services::gfx::{BottomScreen, TopScreen3D},
};

const SCREEN_WIDTH: u16 = 400;
const SCREEN_HEIGHT: u16 = 240;

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

            render_target.render_2d_shape(&Triangle {
                x0: 25.0,
                y0: 190.0,
                color0: clrWhite,
                x1: 0.0,
                y1: SCREEN_HEIGHT as f32,
                color1: clrTri1,
                x2: 50.0,
                y2: SCREEN_HEIGHT as f32,
                color2: clrTri2,
                depth: 0.0,
            });

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

            // Circles require a state change (an expensive operation) within citro2d's internals, so draw them last.
            // Although it is possible to draw them in the middle of drawing non-circular objects
            // (sprites, images, triangles, rectangles, etc.) this is not recommended. They should either
            // be drawn before all non-circular objects, or afterwards.

            render_target.render_2d_shape(&Ellipse {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                width: SCREEN_WIDTH as f32,
                height: SCREEN_HEIGHT as f32,
                top_left_color: clrCircle1,
                top_right_color: clrCircle2,
                bottom_left_color: clrCircle3,
                bottom_right_color: clrWhite,
            });

            render_target.render_2d_shape(&Circle {
                x: (SCREEN_WIDTH / 2) as f32,
                y: (SCREEN_HEIGHT / 2) as f32,
                z: 0.0,
                radius: 50.0,
                top_left_color: clrCircle3,
                top_right_color: clrWhite,
                bottom_left_color: clrCircle1,
                bottom_right_color: clrCircle2,
            });

            render_target.render_2d_shape(&Circle {
                x: 25.0,
                y: 25.0,
                z: 0.0,
                radius: 25.0,
                top_left_color: clrRed,
                top_right_color: clrBlue,
                bottom_left_color: clrGreen,
                bottom_right_color: clrWhite,
            });

            render_target.render_2d_shape(&CircleSolid {
                x: (SCREEN_WIDTH - 25) as f32,
                y: (SCREEN_HEIGHT - 25) as f32,
                z: 0.0,
                radius: 25.0,
                color: clrSolidCircle,
            });
        });

        let stats = citro2d_instance.get_3d_stats();
        bottom_screen.select();
        println!("\x1b[1;1HSimple Rusty citro2d shapes example");
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
