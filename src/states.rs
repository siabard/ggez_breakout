//! states는 다양한 게임내 State를 정의한다.
//! GameState : 게임 진행 상태
//! InitState : 초기 시작 상태
//! MenuState : 메뉴 상태

use crate::game;
use crate::level_maker;
use crate::objects::*;
use crate::objects::{self, Ball, Block, Object, Paddle};
use crate::reg::Reg;
use ggez::audio;
use ggez::audio::SoundSource;
use ggez::graphics::{self, Canvas};
use ggez::input::keyboard::KeyCode;
use ggez::nalgebra as na;
use ggez::Context;

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

pub fn play_sound_once(name: &String, reg: &mut Reg) {
    let sound = reg.get_sound_mut((*name).clone()).unwrap();
    if sound.playing() == false {
        sound.set_repeat(false);
        sound.play().unwrap();
    }
}

pub fn play_sound(name: &String, reg: &mut Reg) {
    let sound = reg.get_sound_mut((*name).clone()).unwrap();
    if sound.playing() == false {
        sound.play().unwrap();
    }
}

pub fn play_bgm(name: &String, reg: &mut Reg) {
    let sound = reg.get_sound_mut((*name).clone()).unwrap();
    sound.set_repeat(true);
    if sound.playing() == false {
        sound.play().unwrap();
    }
}

pub fn stop_sound(name: &String, reg: &mut Reg) {
    let sound = reg.get_sound_mut((*name).clone()).unwrap();
    if sound.playing() == true {
        sound.stop();
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

        init_global_sprite(reg);
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

        //let music = reg.get_sound_mut("music".to_owned()).unwrap();
        play_sound(&"music".to_owned(), reg);

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

            // 수치 정보 등록
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

        graphics::present(ctx).unwrap();

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

        graphics::present(ctx).unwrap();

        ggez::graphics::set_canvas(ctx, None);
        StateResult::Void
    }
}

pub struct PlayState {
    paused: bool,
    paddle: Paddle,
    ball: Ball,
    blocks: Vec<Block>,
    score: i32,
    health: i32,
    level: i32,
}

impl PlayState {
    pub fn new(ctx: &mut Context, reg: &mut Reg) -> PlayState {
        let default_font = ggez::graphics::Font::new(ctx, "/font.ttf").unwrap();

        reg.add_font("default".to_owned(), default_font);

        let paddle = Paddle::new();

        let ball = Ball::new();

        // 배경 음악
        reg.add_sound(
            "music".to_owned(),
            audio::Source::new(ctx, "/music.wav").unwrap(),
        );

        // 효과음
        reg.add_sound(
            "paddle-hit".to_owned(),
            audio::Source::new(ctx, "/paddle_hit.wav").unwrap(),
        );

        reg.add_sound(
            "score".to_owned(),
            audio::Source::new(ctx, "/score.wav").unwrap(),
        );

        reg.add_sound(
            "wall-hit".to_owned(),
            audio::Source::new(ctx, "/wall_hit.wav").unwrap(),
        );

        reg.add_sound(
            "brick-hit-1".to_owned(),
            audio::Source::new(ctx, "/brick-hit-1.wav").unwrap(),
        );

        reg.add_sound(
            "brick-hit-2".to_owned(),
            audio::Source::new(ctx, "/brick-hit-2.wav").unwrap(),
        );

        reg.add_sound(
            "hurt".to_owned(),
            audio::Source::new(ctx, "/hurt.wav").unwrap(),
        );

        reg.add_sound(
            "victory".to_owned(),
            audio::Source::new(ctx, "/victory.wav").unwrap(),
        );
        reg.add_sound(
            "recover".to_owned(),
            audio::Source::new(ctx, "/recover.wav").unwrap(),
        );
        reg.add_sound(
            "high-score".to_owned(),
            audio::Source::new(ctx, "/high_score.wav").unwrap(),
        );
        reg.add_sound(
            "pause".to_owned(),
            audio::Source::new(ctx, "/pause.wav").unwrap(),
        );

        play_bgm(&"music".to_owned(), reg);

        // 블럭 초기화하기
        let blocks = level_maker::create_map(1);

        // score, health, level 값 가져오기
        reg.add_i32("score".to_owned(), 0);
        reg.add_i32("health".to_owned(), 3);

        PlayState {
            paused: false,
            paddle,
            ball,
            blocks,
            health: 3,
            level: 1,
            score: 0,
        }
    }
}
impl States for PlayState {
    fn update(&mut self, ctx: &mut Context, reg: &mut Reg, dt: f32) -> StateResult {
        // 나중에 지울거임..
        // 키 입력할 때 크기 바꿀라구..
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

        // 나중에 지울꺼임..
        // 키 입력할 때 색상 바꿀라구..
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

                    //let music = reg.get_sound_mut("music".to_owned()).unwrap();
                    stop_sound(&("music".to_owned()), reg);

                    //let sound = reg.get_sound_mut("pause".to_owned()).unwrap();
                    play_sound_once(&("pause".to_owned()), reg);
                }

                // paddle 처리
                if color != 0 || size != 0 {
                    self.paddle.set_sprite(objects::PADDLE_FLAG + color + size);
                }
                self.paddle.update(ctx, reg, dt);

                // 공처리
                self.ball.update(ctx, reg, dt);

                if self.ball.y > game::VIRTUAL_HEIGHT {
                    // 죽음..
                    self.health = self.health - 1;
                    self.ball.reset();
                }
                // 두 물체의 충돌처리
                let collide = objects::collide_aabb(&self.paddle, &self.ball);
                if collide.contains(&CollideFlag::TOP) {
                    self.ball.dy = -self.ball.dy;
                    //let sound = reg.get_sound_mut("paddle-hit".to_owned()).unwrap();
                    play_sound_once(&("paddle-hit".to_owned()), reg);
                }

                // 블럭하고 충돌처리
                for block in self.blocks.iter_mut() {
                    if block.inplay == true {
                        let collide = objects::collide_aabb(&self.ball, block);
                        if collide.len() > 0 {
                            block.hit(reg);

                            // 공 상단 / 하단
                            if collide.contains(&CollideFlag::TOP) && self.ball.dy < 0.
                                || collide.contains(&CollideFlag::BOTTOM) && self.ball.dy > 0.
                            {
                                self.ball.dy = -self.ball.dy;
                            }
                            // 공 좌측 / 우측
                            if collide.contains(&CollideFlag::LEFT) && self.ball.dx < 0.
                                || collide.contains(&CollideFlag::RIGHT) && self.ball.dx > 0.
                            {
                                self.ball.dx = -self.ball.dx;
                            }
                        }
                    }
                }

                StateResult::Void
            } else {
                if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Return) {
                    self.paused = false;
                    //let music = reg.get_sound_mut("music".to_owned()).unwrap();
                    //music.set_repeat(true);
                    stop_sound(&("music".to_owned()), reg);
                }

                StateResult::Void
            }
        }
    }

    fn render(&mut self, ctx: &mut Context, reg: &mut Reg, buffer: &mut Canvas) -> StateResult {
        graphics::set_canvas(ctx, Some(buffer));

        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        for block in self.blocks.iter_mut() {
            block.draw(ctx, reg);
        }

        self.paddle.draw(ctx, reg);

        self.ball.draw(ctx, reg);
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
            )
            .unwrap();
        }

        // 생명 출력하기
        let health = self.health;

        let mut hx = 0.;

        for _ in 0..health {
            reg.draw_heart(ctx, objects::HEARTS_FLAG, hx, 0.);
            hx = hx + 11.;
        }

        for _ in health..3 {
            reg.draw_heart(ctx, objects::HEARTS_FLAG + 1, hx, 0.);
            hx = hx + 11.;
        }

        graphics::present(ctx).unwrap();

        graphics::set_canvas(ctx, None);

        StateResult::Void
    }
}
