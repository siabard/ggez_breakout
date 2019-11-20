//! states는 다양한 게임내 State를 정의한다.
//! GameState : 게임 진행 상태
//! InitState : 초기 시작 상태
//! MenuState : 메뉴 상태

use crate::game::{self, Game};
use crate::objects::{self, Ball, Paddle};
use crate::reg::Reg;
use ggez::audio;
use ggez::audio::SoundSource;
use ggez::graphics::{self, Canvas, DrawMode, DrawParam, Rect};
use ggez::input::keyboard::KeyCode;
use ggez::nalgebra as na;
use ggez::Context;
use std::collections::HashMap;

pub enum StateResult {
    PushState(Box<dyn States>),
    PopState,
    Trans(Box<dyn States>),
    Void,
}

pub trait States {
    fn update(&mut self, ctx: &mut Context, reg: &mut Reg, dt: f32) -> StateResult;
    fn render(&mut self, ctx: &mut Context, reg: &mut Reg, buffer: &mut Canvas) -> StateResult;
}

pub fn play_sound(sound: &mut audio::Source) {
    if sound.playing() == false {
        sound.play();
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum InitStateMenu {
    Start,
    Exit,
}

pub struct InitState {
    status: InitStateMenu,
}

impl InitState {
    pub fn new(ctx: &mut Context, reg: &mut Reg) -> InitState {
        let font = ggez::graphics::Font::new(ctx, "/font.ttf").unwrap();
        let title = ggez::graphics::Text::new(("Break Out", font, 16.0));
        let start_menu = ggez::graphics::Text::new(("start game", font, 12.0));
        let exit_menu = ggez::graphics::Text::new(("exit", font, 12.0));

        reg.add_font("font".to_owned(), font);
        reg.add_text("title".to_owned(), title);
        reg.add_text("start_menu".to_owned(), start_menu);
        reg.add_text("exit_menu".to_owned(), exit_menu);
        reg.add_sound(
            "music".to_owned(),
            audio::Source::new(ctx, "/music.wav").unwrap(),
        );
        let state = InitState {
            status: InitStateMenu::Start,
        };

        state
    }
}

// 메뉴 화면
impl States for InitState {
    fn update(&mut self, ctx: &mut Context, reg: &mut Reg, _dt: f32) -> StateResult {
        // 음악을 플레이한다.

        let music = reg.get_sound_mut("music".to_owned()).unwrap();
        play_sound(music);

        // 화살표를 눌러 상태를 변경한다.
        let pressed_key = ggez::input::keyboard::pressed_keys(ctx);

        if !pressed_key.contains(&KeyCode::Up) {
            reg.just_released(KeyCode::Up);
        }

        if !pressed_key.contains(&KeyCode::Down) {
            reg.just_released(KeyCode::Down);
        }

        if pressed_key.contains(&KeyCode::Up) && reg.just_pressed(KeyCode::Up)
            || pressed_key.contains(&KeyCode::Down) && reg.just_pressed(KeyCode::Down)
        {
            //just_pressed 인지 확인

            if self.status == InitStateMenu::Exit {
                self.status = InitStateMenu::Start
            } else {
                self.status = InitStateMenu::Exit
            }

            StateResult::Void
        } else if pressed_key.contains(&KeyCode::Return) {
            // reg 초기화
            reg.clear_font();
            reg.clear_image();
            reg.clear_sound();
            reg.clear_text();

            match self.status {
                InitStateMenu::Start => {
                    let game_state = PlayState::new(ctx, reg);

                    StateResult::Trans(Box::new(game_state))
                }
                InitStateMenu::Exit => StateResult::PopState,
            }
        } else {
            StateResult::Void
        }
    }

    /// 모든 Render는 이제 자체에 포함된 buffer에만 그린다.
    fn render(&mut self, ctx: &mut Context, reg: &mut Reg, buffer: &mut Canvas) -> StateResult {
        ggez::graphics::set_canvas(ctx, Some(buffer));

        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        // 타이틀 (상단 5%, 각 메뉴 상단에서 85%, 95% 위치)
        let title = reg.get_text("title".to_owned()).unwrap();
        let start_menu = reg.get_text("start_menu".to_owned()).unwrap();
        let exit_menu = reg.get_text("exit_menu".to_owned()).unwrap();
        let span = title.width(ctx) as f32;
        graphics::draw(
            ctx,
            title,
            (
                na::Point2::new(
                    (game::VIRTUAL_WIDTH - span) / 2.0,
                    game::VIRTUAL_HEIGHT * 0.05,
                ),
                0.0,
                ggez::graphics::WHITE,
            ),
        )
        .unwrap();

        let span = start_menu.width(ctx) as f32;
        graphics::draw(
            ctx,
            start_menu,
            (
                na::Point2::new(
                    (game::VIRTUAL_WIDTH - span) / 2.0,
                    game::VIRTUAL_HEIGHT * 0.85,
                ),
                0.0,
                match self.status {
                    InitStateMenu::Start => ggez::graphics::Color::from_rgba(200, 200, 255, 255),
                    InitStateMenu::Exit => ggez::graphics::Color::from_rgba(255, 255, 255, 255),
                },
            ),
        )
        .unwrap();

        let span = exit_menu.width(ctx) as f32;
        graphics::draw(
            ctx,
            exit_menu,
            (
                na::Point2::new(
                    (game::VIRTUAL_WIDTH - span) / 2.0,
                    game::VIRTUAL_HEIGHT * 0.95,
                ),
                0.0,
                match self.status {
                    InitStateMenu::Exit => ggez::graphics::Color::from_rgba(200, 200, 255, 255),
                    InitStateMenu::Start => ggez::graphics::Color::from_rgba(255, 255, 255, 255),
                },
            ),
        )
        .unwrap();

        graphics::present(ctx);

        ggez::graphics::set_canvas(ctx, None);
        StateResult::Void
    }
}

pub struct PauseState {}

impl PauseState {
    pub fn new() -> PauseState {
        PauseState {}
    }
}

impl States for PauseState {
    fn update(&mut self, ctx: &mut Context, _reg: &mut Reg, _dt: f32) -> StateResult {
        // X가 눌러지면 스테이트 종료
        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Q) {
            StateResult::PopState
        } else {
            StateResult::Void
        }
    }

    fn render(&mut self, ctx: &mut Context, _reg: &mut Reg, buffer: &mut Canvas) -> StateResult {
        ggez::graphics::set_canvas(ctx, Some(buffer));

        graphics::clear(ctx, [0.0, 1.0, 0.0, 1.0].into());

        graphics::present(ctx);

        ggez::graphics::set_canvas(ctx, None);
        StateResult::Void
    }
}

pub struct PlayState {
    paused: bool,
}

impl PlayState {
    pub fn new(ctx: &mut Context, reg: &mut Reg) -> PlayState {
        let default_font = ggez::graphics::Font::new(ctx, "/font.ttf").unwrap();

        reg.add_font("default".to_owned(), default_font);

        reg.add_object("paddle".to_owned(), Box::new(Paddle::new(ctx)));

        reg.add_object("ball".to_owned(), Box::new(Ball::new(ctx)));
        PlayState { paused: false }
    }
}
impl States for PlayState {
    fn update(&mut self, ctx: &mut Context, reg: &mut Reg, dt: f32) -> StateResult {
        let color: i32 = if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Key1) {
            objects::BLUE
        } else if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Key2) {
            objects::RED
        } else if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Key3) {
            objects::GREEN
        } else if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Key4) {
            objects::MAGENTA
        } else {
            0
        };

        let size: i32 = if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Key5) {
            objects::SMALL
        } else if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Key6) {
            objects::MEDIUM
        } else if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Key7) {
            objects::LARGE
        } else if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Key8) {
            objects::HUGE
        } else {
            0
        };

        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::X) {
            reg.clear_font();
            StateResult::PopState
        } else {
            if self.paused == false {
                if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::P) {
                    self.paused = true;
                }

                // paddle 처리
                let paddle = reg.get_object_mut("paddle".to_owned());
                if color != 0 || size != 0 {
                    paddle
                        .unwrap()
                        .set_sprite(objects::PADDLE_FLAG + color + size);
                }

                let paddle = reg.get_object_mut("paddle".to_owned());
                paddle.unwrap().update(ctx, dt);

                // 공처리
                let ball = reg.get_object_mut("ball".to_owned());
                ball.unwrap().update(ctx, dt);
                StateResult::Void
            } else {
                if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Return) {
                    self.paused = false;
                }

                StateResult::Void
            }
        }
    }

    fn render(&mut self, ctx: &mut Context, reg: &mut Reg, buffer: &mut Canvas) -> StateResult {
        graphics::set_canvas(ctx, Some(buffer));

        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        let paddle = reg.get_object_mut("paddle".to_owned());
        paddle.unwrap().draw(ctx);

        let ball = reg.get_object_mut("ball".to_owned());
        ball.unwrap().draw(ctx);
        if self.paused == true {
            let message = ggez::graphics::Text::new((
                "Game Paused\n\nPress [Enter] To Resume",
                *reg.get_font("default".to_owned()).unwrap(),
                16.0,
            ));

            let span = message.width(ctx) as f32;
            graphics::draw(
                ctx,
                &message,
                (
                    na::Point2::new(
                        (game::VIRTUAL_WIDTH - span) / 2.0,
                        game::VIRTUAL_HEIGHT / 2.0,
                    ),
                    0.0,
                    graphics::WHITE,
                ),
            );
        }

        graphics::present(ctx);

        graphics::set_canvas(ctx, None);

        StateResult::Void
    }
}
