use crate::game;
use crate::quad::Quad;

use ggez::{self, graphics, Context};
use std::path::Path;

pub const PADDLE_FLAG: i32 = 0b0000_0000_0000_0000_0001_0000_0000_0000;
pub const BALL_FLAG: i32 = 0b0000_0000_0000_0000_0010_0000_0000_0000;
pub const BLUE: i32 = 1;
pub const GREEN: i32 = 2;
pub const RED: i32 = 4;
pub const MAGENTA: i32 = 8;
pub const STAT_1: i32 = 0b0001_0000;
pub const STAT_2: i32 = 0b0010_0000;
pub const STAT_3: i32 = 0b0100_0000;
pub const COLOR_MASK: i32 = 0b1111;
pub const BALL_MASK: i32 = 0b1111_1111;

pub const SMALL: i32 = 0b0001_0000;
pub const MEDIUM: i32 = 0b0010_0000;
pub const LARGE: i32 = 0b0100_0000;
pub const HUGE: i32 = 0b1000_0000;
pub const SIZE_MASK: i32 = 0b1111_0000;

pub const PADDLE_SPEED: f32 = 200.;

pub struct Paddle {
    sprite: Quad,
    size: i32,
    color: i32,
    x: f32,
    y: f32,
    dx: f32,
}

pub struct Ball {
    sprite: Quad,
    color: i32,
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
}

pub trait Object {
    fn update(&mut self, ctx: &mut Context, dt: f32);
    fn draw(&mut self, ctx: &mut Context);
    fn set_sprite(&mut self, idx: i32);
}

impl Paddle {
    pub fn new(ctx: &mut Context) -> Paddle {
        let mut sprite = Quad::new(ctx, Path::new("/breakout.png"));
        sprite.add_sprite(PADDLE_FLAG + BLUE + SMALL, 0., 64., 32., 16.);
        sprite.add_sprite(PADDLE_FLAG + BLUE + MEDIUM, 32., 64., 64., 16.);
        sprite.add_sprite(PADDLE_FLAG + BLUE + LARGE, 96., 64., 96., 16.);
        sprite.add_sprite(PADDLE_FLAG + BLUE + HUGE, 0., 80., 128., 16.);

        sprite.add_sprite(PADDLE_FLAG + GREEN + SMALL, 0., 96., 32., 16.);
        sprite.add_sprite(PADDLE_FLAG + GREEN + MEDIUM, 32., 96., 64., 16.);
        sprite.add_sprite(PADDLE_FLAG + GREEN + LARGE, 96., 96., 96., 16.);
        sprite.add_sprite(PADDLE_FLAG + GREEN + HUGE, 0., 112., 128., 16.);

        sprite.add_sprite(PADDLE_FLAG + RED + SMALL, 0., 128., 32., 16.);
        sprite.add_sprite(PADDLE_FLAG + RED + MEDIUM, 32., 128., 64., 16.);
        sprite.add_sprite(PADDLE_FLAG + RED + LARGE, 96., 128., 96., 16.);
        sprite.add_sprite(PADDLE_FLAG + RED + HUGE, 0., 144., 128., 16.);

        sprite.add_sprite(PADDLE_FLAG + MAGENTA + SMALL, 0., 160., 32., 16.);
        sprite.add_sprite(PADDLE_FLAG + MAGENTA + MEDIUM, 32., 160., 64., 16.);
        sprite.add_sprite(PADDLE_FLAG + MAGENTA + LARGE, 96., 160., 96., 16.);
        sprite.add_sprite(PADDLE_FLAG + MAGENTA + HUGE, 0., 176., 128., 16.);

        // 화면 가운데에 위치시킨다.
        Paddle {
            sprite,
            size: MEDIUM,
            color: MAGENTA,
            x: game::VIRTUAL_WIDTH / 2.,
            y: game::VIRTUAL_HEIGHT - 32.,
            dx: 0.,
        }
    }
}

impl Ball {
    pub fn new(ctx: &mut Context) -> Ball {
        let mut sprite = Quad::new(ctx, Path::new("/breakout.png"));
        sprite.add_sprite(BALL_FLAG + BLUE, 96., 48., 8., 8.);
        sprite.add_sprite(BALL_FLAG + GREEN, 104., 48., 8., 8.);
        sprite.add_sprite(BALL_FLAG + RED, 112., 48., 8., 8.);
        sprite.add_sprite(BALL_FLAG + MAGENTA, 120., 48., 8., 8.);

        sprite.add_sprite(BALL_FLAG + STAT_1, 96., 56., 8., 8.);
        sprite.add_sprite(BALL_FLAG + STAT_2, 104., 56., 8., 8.);
        sprite.add_sprite(BALL_FLAG + STAT_3, 112., 56., 8., 8.);

        // 화면 가운데에 위치시킨다.
        Ball {
            sprite,
            color: MAGENTA,
            x: game::VIRTUAL_WIDTH / 2.,
            y: game::VIRTUAL_HEIGHT - 32.,
            dx: 0.,
            dy: 0.,
        }
    }
}

impl Object for Paddle {
    fn update(&mut self, ctx: &mut Context, dt: f32) {
        if ggez::input::keyboard::is_key_pressed(ctx, ggez::input::keyboard::KeyCode::Left) {
            self.dx = -1. * PADDLE_SPEED;
        } else if ggez::input::keyboard::is_key_pressed(ctx, ggez::input::keyboard::KeyCode::Right)
        {
            self.dx = PADDLE_SPEED;
        } else {
            self.dx = 0.;
        }

        if self.dx < 0. {
            self.x = (self.x + self.dx * dt).max(0.);
        } else if self.dx > 0. {
            self.x = game::VIRTUAL_WIDTH.min(self.x + self.dx * dt);
        }
    }

    fn draw(&mut self, ctx: &mut Context) {
        self.sprite
            .draw_sprite(ctx, PADDLE_FLAG + self.color + self.size, self.x, self.y);
    }

    fn set_sprite(&mut self, idx: i32) {
        let color = idx & COLOR_MASK;
        if color > 0 {
            self.color = color;
        }
        let size = idx & SIZE_MASK;
        if size > 0 {
            self.size = size;
        }
    }
}

impl Object for Ball {
    fn update(&mut self, _ctx: &mut Context, _dt: f32) {
        ()
    }

    fn draw(&mut self, ctx: &mut Context) {
        self.sprite
            .draw_sprite(ctx, BALL_FLAG + self.color, self.x, self.y);
    }

    fn set_sprite(&mut self, idx: i32) {
        let color = idx & BALL_MASK;
        if color > 0 {
            self.color = color;
        }
    }
}
