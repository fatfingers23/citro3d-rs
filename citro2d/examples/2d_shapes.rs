//! This example demonstrates the most basic usage of `citro2d`: rendering shapes
//! on the top screen of the 3DS.

#![feature(allocator_api)]

use citro2d::{
    Instance,
    render::{Color, Target},
};
use citro3d_sys::C3D_FRAME_SYNCDRAW;
use ctru::{
    prelude::{Apt, Gfx, Hid, KeyPad},
    services::gfx::{RawFrameBuffer, Screen, TopScreen3D},
};

fn main() {
    let gfx = Gfx::new().expect("Couldn't obtain GFX controller");
    let mut hid = Hid::new().expect("Couldn't obtain HID controller");
    let apt = Apt::new().expect("Couldn't obtain APT controller");

    let citro2d_instance = citro2d::Instance::new().expect("Couldn't obtain citro2d instance");
    let top_screen = TopScreen3D::from(&gfx.top_screen);
    let (mut top_left, _) = top_screen.split_mut();
    let target = Target::new(top_left).expect("failed to create render target");

    let red = Color::new(255, 0, 0);
    while apt.main_loop() {
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }
        unsafe {
            citro3d_sys::C3D_FrameBegin(citro3d_sys::C3D_FRAME_SYNCDRAW);
            citro2d_sys::C2D_SceneBegin(target.raw);

            citro2d_sys::C2D_TargetClear(target.raw, red.inner);

            citro3d_sys::C3D_FrameEnd(0);
        }

        gfx.wait_for_vblank();
    }
}
