//! states는 다양한 게임내 State를 정의한다.
//! GameState : 게임 진행 상태
//! InitState : 초기 시작 상태
//! MenuState : 메뉴 상태

use crate::game::{self, Game};
use crate::objects::Paddle;
use ggez::graphics::{self, Canvas, DrawMode, DrawParam, Drawable, Rect};
use ggez::input::keyboard::KeyCode;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use std::collections::HashMap;

pub enum StateResult {
    PushState(Box<dyn States>),
    PopState,
    Trans(Box<dyn States>),
    Void,
}

pub trait States {
    fn update(&mut self, ctx: &mut Context, dt: f32) -> StateResult;
    fn render(&mut self, ctx: &mut Context, buffer: &mut Canvas) -> StateResult;
}

#[derive(Clone, PartialEq, Debug)]
pub enum InitStateMenu {
    Start,
    Exit,
}
#[derive(Clone)]
pub struct InitState {
    status: InitStateMenu,
    font: ggez::graphics::Font,
    title: ggez::graphics::Text,
    start_menu: ggez::graphics::Text,
    exit_menu: ggez::graphics::Text,
}

impl InitState {
    pub fn new(ctx: &mut Context) -> InitState {
        let font = ggez::graphics::Font::new(ctx, "/font.ttf").unwrap();
        let title = ggez::graphics::Text::new(("Break Out", font, 16.0));
        let start_menu = ggez::graphics::Text::new(("start game", font, 12.0));
        let exit_menu = ggez::graphics::Text::new(("exit", font, 12.0));
        InitState {
            status: InitStateMenu::Start,
            font,
            title,
            start_menu,
            exit_menu,
        }
    }
}

// 메뉴 화면
impl States for InitState {
    fn update(&mut self, ctx: &mut Context, _dt: f32) -> StateResult {
        // 화살표를 눌러 상태를 변경한다.
        let pressed_key = ggez::input::keyboard::pressed_keys(ctx);

        if pressed_key.contains(&KeyCode::Up) || pressed_key.contains(&KeyCode::Down) {
            if self.status == InitStateMenu::Exit {
                self.status = InitStateMenu::Start
            } else {
                self.status = InitStateMenu::Exit
            }

            StateResult::Void
        } else if pressed_key.contains(&KeyCode::Return) {
            match self.status {
                InitStateMenu::Start => {
                    let game_state = PlayState::new(ctx);
                    StateResult::Trans(Box::new(game_state))
                }
                InitStateMenu::Exit => StateResult::PopState,
            }
        } else {
            StateResult::Void
        }
    }

    /// 모든 Render는 이제 자체에 포함된 buffer에만 그린다.
    fn render(&mut self, ctx: &mut Context, buffer: &mut Canvas) -> StateResult {
        ggez::graphics::set_canvas(ctx, Some(buffer));

        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        // 타이틀 (상단 5%, 각 메뉴 상단에서 85%, 95% 위치)
        let span = self.title.width(ctx) as f32;
        graphics::draw(
            ctx,
            &self.title,
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

        let span = self.start_menu.width(ctx) as f32;
        graphics::draw(
            ctx,
            &self.start_menu,
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

        let span = self.exit_menu.width(ctx) as f32;
        graphics::draw(
            ctx,
            &self.exit_menu,
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

#[derive(Clone)]
pub struct PauseState {}

impl PauseState {
    pub fn new() -> PauseState {
        PauseState {}
    }
}

impl States for PauseState {
    fn update(&mut self, ctx: &mut Context, _dt: f32) -> StateResult {
        // X가 눌러지면 스테이트 종료
        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Q) {
            StateResult::PopState
        } else {
            StateResult::Void
        }
    }

    fn render(&mut self, ctx: &mut Context, buffer: &mut Canvas) -> StateResult {
        ggez::graphics::set_canvas(ctx, Some(buffer));

        graphics::clear(ctx, [0.0, 1.0, 0.0, 1.0].into());

        graphics::present(ctx);

        ggez::graphics::set_canvas(ctx, None);
        StateResult::Void
    }
}

#[derive(Clone)]
pub struct PlayState {
    sprite: ggez::graphics::Mesh,
    x: f32,
    y: f32,
    paddle: Paddle,
    // font
    font: HashMap<String, ggez::graphics::Font>,
    /// 게임의 일시 정지 상태
    paused: bool,
}

impl PlayState {
    pub fn new(ctx: &mut Context) -> PlayState {
        let default_font = ggez::graphics::Font::new(ctx, "/font.ttf").unwrap();

        let mut font = HashMap::<String, ggez::graphics::Font>::new();
        font.insert("default".to_owned(), default_font);
        PlayState {
            sprite: ggez::graphics::Mesh::new_rectangle(
                ctx,
                ggez::graphics::DrawMode::fill(),
                Rect::new(0., 0., 100., 100.),
                ggez::graphics::WHITE,
            )
            .unwrap(),
            x: 0.,
            y: 0.,
            paddle: Paddle::new(ctx),
            font,
            paused: false,
        }
    }
}
impl States for PlayState {
    fn update(&mut self, ctx: &mut Context, dt: f32) -> StateResult {
        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::X) {
            StateResult::PopState
        } else {
            if self.paused == false {
                if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::P) {
                    self.paused = true;
                }

                // paddle 처리
                self.paddle.update(ctx, dt);
                // X가 눌러지면 게임 스테이트 종료.
                if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Up) {
                    self.y = self.y - 100.;
                    StateResult::Void
                } else if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Down) {
                    self.y = self.y + 100.;
                    StateResult::Void
                } else {
                    StateResult::Void
                }
            } else {
                if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Return) {
                    self.paused = false;
                }

                StateResult::Void
            }
        }
    }

    fn render(&mut self, ctx: &mut Context, buffer: &mut Canvas) -> StateResult {
        graphics::set_canvas(ctx, Some(buffer));

        graphics::clear(ctx, [1.0, 0.0, 0.0, 1.0].into());

        let dest = na::Point2::new(self.x, self.y);
        self.sprite
            .draw(ctx, DrawParam::default().dest(dest))
            .unwrap();
        self.paddle.draw(ctx);

        if self.paused == true {
            let message = ggez::graphics::Text::new((
                "Game Paused\n\nPress [Enter] To Resume",
                *self.font.get("default").unwrap(),
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
