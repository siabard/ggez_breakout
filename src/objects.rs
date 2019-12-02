use crate::game;
use crate::quad::Quad;
use crate::reg::Reg;
use crate::states::{play_sound, play_sound_once};

use ggez::{self, graphics, Context};
use std::path::Path;

pub const PADDLE_FLAG: i32 = 0b0000_0000_0000_0000_0001_0000_0000_0000;
pub const BALL_FLAG: i32 = 0b0000_0000_0000_0000_0010_0000_0000_0000;
pub const BLOCK_FLAG: i32 = 0b0000_0000_0000_0000_0100_0000_0000_0000;
pub const HEARTS_FLAG: i32 = 0b0000_0000_0000_0000_1000_0000_0000_0000;
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

#[derive(PartialEq, Debug)]
pub enum CollideFlag {
    TOP,
    BOTTOM,
    LEFT,
    RIGHT,
    NONE,
}

pub trait Object {
    fn update(&mut self, ctx: &mut Context, reg: &mut Reg, dt: f32);
    fn draw(&mut self, ctx: &mut Context, reg: &mut Reg);
    fn set_sprite(&mut self, idx: i32);
    fn get_xywh(&self) -> (f32, f32, f32, f32);
}

/// AABB Collide
pub fn collide_aabb(a: &dyn Object, b: &dyn Object) -> Vec<CollideFlag> {
    let a_xywh = a.get_xywh();
    let b_xywh = b.get_xywh();

    if (a_xywh.0 < b_xywh.0 + b_xywh.2)
        && (a_xywh.0 + a_xywh.2 > b_xywh.0)
        && (a_xywh.1 < b_xywh.1 + b_xywh.3)
        && (a_xywh.1 + a_xywh.3 > b_xywh.1)
    {
        // 충돌시에 어느 면에 부딪히는지를 알려준다.
        // a.x < b.x -> LEFT
        // a.x > b.x -> RIGHT
        // a.y < b.y -> BOTTOM
        // a.y > b.y -> TOP
        let mut vec = Vec::<CollideFlag>::new();
        if a_xywh.0 < b_xywh.0 {
            vec.push(CollideFlag::LEFT);
        }

        if a_xywh.0 > b_xywh.0 {
            vec.push(CollideFlag::RIGHT);
        }

        if a_xywh.1 < b_xywh.1 {
            vec.push(CollideFlag::BOTTOM);
        }

        if a_xywh.1 > b_xywh.1 {
            vec.push(CollideFlag::TOP);
        }

        vec
    } else {
        vec![]
    }
}

// 기본적인 스프라이트 데이터를 모두 초기화한다.
pub fn init_global_sprite(reg: &mut Reg) {
    // Paddle
    reg.register_sprite(PADDLE_FLAG + BLUE + SMALL, 0., 64., 32., 16.);
    reg.register_sprite(PADDLE_FLAG + BLUE + MEDIUM, 32., 64., 64., 16.);
    reg.register_sprite(PADDLE_FLAG + BLUE + LARGE, 96., 64., 96., 16.);
    reg.register_sprite(PADDLE_FLAG + BLUE + HUGE, 0., 80., 128., 16.);

    reg.register_sprite(PADDLE_FLAG + GREEN + SMALL, 0., 96., 32., 16.);
    reg.register_sprite(PADDLE_FLAG + GREEN + MEDIUM, 32., 96., 64., 16.);
    reg.register_sprite(PADDLE_FLAG + GREEN + LARGE, 96., 96., 96., 16.);
    reg.register_sprite(PADDLE_FLAG + GREEN + HUGE, 0., 112., 128., 16.);

    reg.register_sprite(PADDLE_FLAG + RED + SMALL, 0., 128., 32., 16.);
    reg.register_sprite(PADDLE_FLAG + RED + MEDIUM, 32., 128., 64., 16.);
    reg.register_sprite(PADDLE_FLAG + RED + LARGE, 96., 128., 96., 16.);
    reg.register_sprite(PADDLE_FLAG + RED + HUGE, 0., 144., 128., 16.);

    reg.register_sprite(PADDLE_FLAG + MAGENTA + SMALL, 0., 160., 32., 16.);
    reg.register_sprite(PADDLE_FLAG + MAGENTA + MEDIUM, 32., 160., 64., 16.);
    reg.register_sprite(PADDLE_FLAG + MAGENTA + LARGE, 96., 160., 96., 16.);
    reg.register_sprite(PADDLE_FLAG + MAGENTA + HUGE, 0., 176., 128., 16.);

    // Ball

    reg.register_sprite(BALL_FLAG + BLUE, 96., 48., 8., 8.);
    reg.register_sprite(BALL_FLAG + GREEN, 104., 48., 8., 8.);
    reg.register_sprite(BALL_FLAG + RED, 112., 48., 8., 8.);
    reg.register_sprite(BALL_FLAG + MAGENTA, 120., 48., 8., 8.);

    reg.register_sprite(BALL_FLAG + STAT_1, 96., 56., 8., 8.);
    reg.register_sprite(BALL_FLAG + STAT_2, 104., 56., 8., 8.);
    reg.register_sprite(BALL_FLAG + STAT_3, 112., 56., 8., 8.);

    // block
    // 블럭 종류는 총 21개임
    let mut x: f32 = 0.;
    let mut y: f32 = 0.;

    for i in 1..22 {
        reg.register_sprite(BLOCK_FLAG + i, x, y, 32., 16.);

        if i % 6 == 0 {
            x = 0.;
            y = y + 16.;
        } else {
            x = x + 32.;
        }
    }

    // hearts
    reg.register_heart(HEARTS_FLAG, 0., 0., 10., 9.);
    reg.register_heart(HEARTS_FLAG + 1, 10., 0., 10., 9.);
}

pub struct Paddle {
    size: i32,
    color: i32,
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
    pub dx: f32,
}

impl Paddle {
    fn get_width(&self) -> f32 {
        match self.size {
            SMALL => 32.,
            MEDIUM => 64.,
            LARGE => 96.,
            HUGE => 128.,
            _ => 32.,
        }
    }

    pub fn new() -> Paddle {
        // 화면 가운데에 위치시킨다.
        Paddle {
            size: MEDIUM,
            color: MAGENTA,
            x: game::VIRTUAL_WIDTH / 2.,
            y: game::VIRTUAL_HEIGHT - 32.,
            width: 64.,
            height: 16.,
            dx: 0.,
        }
    }
}

impl Object for Paddle {
    fn update(&mut self, ctx: &mut Context, _reg: &mut Reg, dt: f32) {
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

    fn draw(&mut self, ctx: &mut Context, reg: &mut Reg) {
        reg.draw_sprite(ctx, PADDLE_FLAG + self.color + self.size, self.x, self.y);
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

    fn get_xywh(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.width, self.height)
    }
}

pub struct Ball {
    color: i32,
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
}

impl Ball {
    pub fn new() -> Ball {
        // 화면 가운데에 위치시킨다.
        Ball {
            color: MAGENTA,
            x: game::VIRTUAL_WIDTH / 2.,
            y: game::VIRTUAL_HEIGHT - 40.,
            width: 8.,
            height: 8.,
            dx: 0.,
            dy: 0.,
        }
    }

    pub fn reset(&mut self) {
        self.y = VIRTUAL_HEIGHT - 40.,;
        self.dx = 0.;
        self.dy = 0.;
    }
}

impl Object for Ball {
    fn update(&mut self, _ctx: &mut Context, reg: &mut Reg, _dt: f32) {
        if (self.x < 0. || self.x > game::VIRTUAL_WIDTH) {
            self.dx = -self.dx;
            //let sound = reg.get_sound_mut("wall-hit".to_owned()).unwrap();
            play_sound_once(&"wall-hit".to_owned(), reg);
        }
        if self.y < 0. {
            self.dy = -self.dy;
            //let sound = reg.get_sound_mut("wall-hit".to_owned()).unwrap();
            play_sound_once(&"wall-hit".to_owned(), reg);
        }

        self.x += self.dx;
        self.y += self.dy;
    }

    fn draw(&mut self, ctx: &mut Context, reg: &mut Reg) {
        reg.draw_sprite(ctx, BALL_FLAG + self.color, self.x, self.y);
    }

    fn set_sprite(&mut self, idx: i32) {
        let color = idx & BALL_MASK;
        if color > 0 {
            self.color = color;
        }
    }

    fn get_xywh(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.width, self.height)
    }
}

pub struct Block {
    pub color: i32,
    pub tier: i32,
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub inplay: bool,
}

impl Block {
    pub fn new(ox: f32, oy: f32) -> Block {
        // Block 설치하기
        Block {
            color: 1,
            tier: 0,
            x: ox,
            y: oy,
            width: 32.,
            height: 16.,
            dx: 0.,
            dy: 0.,
            inplay: true,
        }
    }

    pub fn hit(&mut self, reg: &mut Reg) {
        self.inplay = false;
        play_sound(&"brick-hit-2".to_owned(), reg);
    }
}

impl Object for Block {
    fn update(&mut self, _ctx: &mut Context, _reg: &mut Reg, _dt: f32) {
        ()
    }

    fn draw(&mut self, ctx: &mut Context, reg: &mut Reg) {
        if self.inplay {
            reg.draw_sprite(
                ctx,
                BLOCK_FLAG + 1 + (self.color - 1) * 4 + self.tier,
                self.x,
                self.y,
            );
        }
    }

    fn set_sprite(&mut self, idx: i32) {
        let color = idx & BALL_MASK;
        if color > 0 {
            self.color = color;
        }
    }

    fn get_xywh(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.width, self.height)
    }
}
