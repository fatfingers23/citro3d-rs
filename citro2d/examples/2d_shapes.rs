//! This example demonstrates the most basic usage of `citro2d`: rendering shapes
//! on the top screen of the 3DS.
//! This is an exact copy of 2d_shapes from the devkitPro examples, but in Rust.
//! https://github.com/devkitPro/3ds-examples/blob/master/graphics/gpu/2d_shapes/source/main.c
#![feature(allocator_api)]

use citro2d::render::{Color, Target};
use citro2d::shapes::{Circle, CircleSolid, Ellipse, Rectangle, Triangle};
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
    let clr_white = Color::new_with_alpha(0xFF, 0xFF, 0xFF, 0xFF);
    let clr_green = Color::new_with_alpha(0x00, 0xFF, 0x00, 0xFF);
    let clr_red = Color::new_with_alpha(0xFF, 0x00, 0x00, 0xFF);
    let clr_blue = Color::new_with_alpha(0x00, 0x00, 0xFF, 0xFF);
    let clr_circle1 = Color::new_with_alpha(0xFF, 0x00, 0xFF, 0xFF);
    let clr_circle2 = Color::new_with_alpha(0xFF, 0xFF, 0x00, 0xFF);
    let clr_circle3 = Color::new_with_alpha(0x00, 0xFF, 0xFF, 0xFF);
    let clr_solid_circle = Color::new_with_alpha(0x68, 0xB0, 0xD8, 0xFF);
    let clr_tri1 = Color::new_with_alpha(0xFF, 0x15, 0x00, 0xFF);
    let clr_tri2 = Color::new_with_alpha(0x27, 0x69, 0xE5, 0xFF);
    let clr_rec1 = Color::new_with_alpha(0x9A, 0x6C, 0xB9, 0xFF);
    let clr_rec2 = Color::new_with_alpha(0xFF, 0xFF, 0x2C, 0xFF);
    let clr_rec3 = Color::new_with_alpha(0xD8, 0xF6, 0x0F, 0xFF);
    let clr_rec4 = Color::new_with_alpha(0x40, 0xEA, 0x87, 0xFF);
    let clr_clear = Color::new_with_alpha(0xFF, 0xD8, 0xB0, 0x68);

    while apt.main_loop() {
        hid.scan_input();

        citro2d_instance.render_target(&mut top_target, |_instance, render_target| {
            render_target.clear(clr_clear);

            render_target.render_2d_shape(&Triangle {
                x0: 25.0,
                y0: 190.0,
                color0: clr_white,
                x1: 0.0,
                y1: SCREEN_HEIGHT as f32,
                color1: clr_tri1,
                x2: 50.0,
                y2: SCREEN_HEIGHT as f32,
                color2: clr_tri2,
                depth: 0.0,
            });

            render_target.render_2d_shape(&Rectangle {
                x: 350.0,
                y: 0.0,
                z: 0.0,
                width: 50.0,
                height: 50.0,
                color_top_left: clr_rec1,
                color_top_right: clr_rec2,
                color_bottom_left: clr_rec3,
                color_bottom_right: clr_rec4,
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
                top_left_color: clr_circle1,
                top_right_color: clr_circle2,
                bottom_left_color: clr_circle3,
                bottom_right_color: clr_white,
            });

            render_target.render_2d_shape(&Circle {
                x: (SCREEN_WIDTH / 2) as f32,
                y: (SCREEN_HEIGHT / 2) as f32,
                z: 0.0,
                radius: 50.0,
                top_left_color: clr_circle3,
                top_right_color: clr_white,
                bottom_left_color: clr_circle1,
                bottom_right_color: clr_circle2,
            });

            render_target.render_2d_shape(&Circle {
                x: 25.0,
                y: 25.0,
                z: 0.0,
                radius: 25.0,
                top_left_color: clr_red,
                top_right_color: clr_blue,
                bottom_left_color: clr_green,
                bottom_right_color: clr_white,
            });

            render_target.render_2d_shape(&CircleSolid {
                x: (SCREEN_WIDTH - 25) as f32,
                y: (SCREEN_HEIGHT - 25) as f32,
                z: 0.0,
                radius: 25.0,
                color: clr_solid_circle,
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
