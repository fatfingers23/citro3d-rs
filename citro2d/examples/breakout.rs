//! This example demonstrates a simple 2d game of the classic game Breakout
#![feature(allocator_api)]

use citro2d::render::{Color, Target};
use citro2d::shapes::{
    Circle, CircleSolid, Ellipse, MultiColor, Rectangle, RectangleSolid, Triangle,
};
use citro2d::{Point, Size};
use ctru::{prelude::*, services::gfx::TopScreen3D};

const TOP_SCREEN_WIDTH: u16 = 400;
const TOP_SCREEN_HEIGHT: u16 = 240;

const BOTTOM_SCREEN_WIDTH: u16 = 320;
const BOTTOM_SCREEN_HEIGHT: u16 = 240;

fn main() {
    let gfx = Gfx::new().expect("Couldn't obtain GFX controller");
    let mut hid = Hid::new().expect("Couldn't obtain HID controller");
    let apt = Apt::new().expect("Couldn't obtain APT controller");

    let mut citro2d_instance = citro2d::Instance::new().expect("Couldn't obtain citro2d instance");
    let top_screen = TopScreen3D::from(&gfx.top_screen);
    let (top_left, _) = top_screen.split_mut();
    let mut top_target = Target::new(top_left).expect("failed to create render target");

    let mut bottom_target =
        Target::new(gfx.bottom_screen.borrow_mut()).expect("failed to create render target");

    let white = Color::new(255, 255, 255);
    let black = Color::new(0, 0, 0);

    let mut paddle = Paddle {
        position: Point::new(
            BOTTOM_SCREEN_WIDTH as f32 / 2.0,
            BOTTOM_SCREEN_HEIGHT as f32 - 10.0,
            0.0,
        ),
        size: (75.0, 10.0).into(),
        color: white,
    };

    let mut ball = Ball {
        position: Point::new(
            BOTTOM_SCREEN_WIDTH as f32 / 2.0,
            BOTTOM_SCREEN_HEIGHT as f32 / 2.0,
            0.0,
        ),
        radius: 5.0,
        color: white,
        velocity: Point::new(2.0, -2.0, 0.0),
    };

    while apt.main_loop() {
        hid.scan_input();
        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        if hid.keys_held().contains(KeyPad::LEFT) || hid.keys_held().contains(KeyPad::DPAD_LEFT) {
            paddle.move_left();
        }

        if hid.keys_held().contains(KeyPad::RIGHT) || hid.keys_held().contains(KeyPad::DPAD_RIGHT) {
            paddle.move_right();
        }

        citro2d_instance.render_target(&mut top_target, |_instance, render_target| {
            render_target.clear(black);
        });

        citro2d_instance.render_target(&mut bottom_target, |_instance, render_target| {
            render_target.clear(black);
            ball.bounce(&paddle);
            paddle.render(render_target);
            //circles are better to render last for performance reasons
            ball.render(render_target);
        });

        //Uncomment to cap fps
        // gfx.wait_for_vblank();
    }
}

struct Paddle {
    pub position: Point,
    pub size: Size,
    pub color: Color,
}

impl Paddle {
    fn render(&self, render_target: &mut Target) {
        render_target.render_2d_shape(&RectangleSolid {
            point: self.position,
            size: self.size,
            color: self.color,
        });
    }

    fn move_left(&mut self) {
        if self.position.x > 0.0 {
            self.position.x -= 1.75;
        }
    }

    fn move_right(&mut self) {
        if self.position.x <= BOTTOM_SCREEN_WIDTH as f32 - self.size.width {
            self.position.x += 1.75;
        }
    }
}

struct Ball {
    pub position: Point,
    pub radius: f32,
    pub color: Color,
    pub velocity: Point,
}

impl Ball {
    fn render(&self, render_target: &mut Target) {
        render_target.render_2d_shape(&CircleSolid {
            x: self.position.x,
            y: self.position.y,
            z: self.position.z,
            radius: self.radius,
            color: self.color,
        });
    }

    fn bounce(&mut self, paddle: &Paddle) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;

        // Check for collision with the walls
        if self.position.x - self.radius <= 0.0
            || self.position.x + self.radius >= BOTTOM_SCREEN_WIDTH as f32
        {
            self.velocity.x = -self.velocity.x;
        }

        if self.position.y - self.radius <= 0.0 {
            self.velocity.y = -self.velocity.y;
        }

        // Check for collision with the paddle
        if self.position.y + self.radius >= paddle.position.y
            && self.position.x >= paddle.position.x
            && self.position.x <= paddle.position.x + paddle.size.width
        {
            self.velocity.y = -self.velocity.y;
        }

        // Check if the ball hits the bottom of the screen
        if self.position.y + self.radius >= BOTTOM_SCREEN_HEIGHT as f32 {
            // Reset ball position or handle game over
            self.position = Point::new(
                BOTTOM_SCREEN_WIDTH as f32 / 2.0,
                BOTTOM_SCREEN_HEIGHT as f32 / 2.0,
                0.0,
            );
            self.velocity = Point::new(2.0, -2.0, 0.0);
        }
    }
}
