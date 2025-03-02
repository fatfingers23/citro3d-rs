//! This example demonstrates the most basic usage of `citro2d`: rendering shapes
//! on the top screen of the 3DS.

#![feature(allocator_api)]

use citro2d::render::{Color, Target};
use ctru::{
    prelude::{Apt, Gfx, Hid, KeyPad},
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
    let mut bottom_target =
        Target::new(gfx.bottom_screen.borrow_mut()).expect("failed to create render target");
    let colors = [
        Color::new(255, 0, 0),   // Red
        Color::new(255, 127, 0), // Orange
        Color::new(255, 255, 0), // Yellow
        Color::new(0, 255, 0),   // Green
        Color::new(0, 0, 255),   // Blue
        Color::new(75, 0, 130),  // Indigo
        Color::new(148, 0, 211), // Violet
    ];
    let mut top_color_index = 0;
    let mut bottom_color_index = 0;
    let mut pick_new_color_counter = 0;
    let mut pick_new_color = false;
    while apt.main_loop() {
        pick_new_color_counter += 1;

        hid.scan_input();

        citro2d_instance.render_target(&mut top_target, |_instance, render_target| unsafe {
            if pick_new_color {
                render_target.clear(colors[top_color_index]);
                top_color_index = (top_color_index + 1) % colors.len();
            }
        });

        citro2d_instance.render_target(&mut bottom_target, |_instance, render_target| unsafe {
            if pick_new_color {
                render_target.clear(colors[bottom_color_index]);
                bottom_color_index = (bottom_color_index + 1) % colors.len();
            }
        });

        if pick_new_color {
            pick_new_color = false;
        }

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        if pick_new_color_counter % 10 == 0 {
            pick_new_color_counter = 0;
            pick_new_color = true;
        }

        gfx.wait_for_vblank();
    }
}
