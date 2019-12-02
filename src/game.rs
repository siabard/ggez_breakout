//! 게임 뼈대 시스템
//! 게임 뼈대는 실제 Rendering을 수행한다.

use ggez;
use ggez::event;
use ggez::event::KeyCode;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::timer;
use ggez::{Context, GameResult};

use crate::reg::Reg;
use crate::states;
use crate::states::StateResult;

use std::path::Path;

/// 실제 물리적 해상도
pub const WINDOW_WIDTH: f32 = 1280.;
pub const WINDOW_HEIGHT: f32 = 720.;

/// 가상해상도
pub const VIRTUAL_WIDTH: f32 = 432.;
pub const VIRTUAL_HEIGHT: f32 = 243.;

/// 게임 구조체
pub struct Game {
    // 게임내 각 state의 벡터 (stack식)
    states: Vec<Box<dyn states::States>>,
    // 게임내 double bufferingmf 위한 버퍼
    buffer: ggez::graphics::Canvas,
    // 게임내 데이터 보존소
    reg: Reg,
}

impl Game {
    /// Game 객체를 반환한다.
    ///
    /// # Arguments
    ///
    /// * `ctx` - Context 객체
    ///
    pub fn new(ctx: &mut Context) -> GameResult<Game> {
        // 초기에는 InitState를 넣는다.

        let mut reg = Reg::new();
        reg.init_sprite(ctx, Path::new("/breakout.png"));
        reg.init_heart(ctx, Path::new("/hearts.png"));
        let init_state = states::InitState::new(ctx, &mut reg);

        let buffer = ggez::graphics::Canvas::new(
            ctx,
            VIRTUAL_WIDTH as u16,
            VIRTUAL_HEIGHT as u16,
            ggez::conf::NumSamples::One,
        )
        .unwrap();

        let s = Game {
            states: vec![Box::new(init_state)],
            buffer,
            reg,
        };
        Ok(s)
    }
}

impl event::EventHandler for Game {
    /// Game의 매 프레임마다 수행되는 루틴
    /// # Arguments
    ///
    /// * `ctx` - Context 객체
    ///
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // 닫기가 눌러지면 게임 종료한다.
        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Escape) {
            ggez::event::quit(ctx);
        }

        // dt(delta) 얻어오기
        // FPS를 60frame per seconds 로 함
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let dt = 1.0 / (DESIRED_FPS as f32);

            // 현재 states 값을 얻어와 해당 states의 update 를 실행한다.
            match self.states.last_mut() {
                Some(current_state) => {
                    match current_state.update(ctx, &mut self.reg, dt) {
                        // 새로운 State를 생성하고 해당 State로 수행권한을 넘긴다.
                        StateResult::PushState(s) => self.states.push(s),
                        // 기존의 State를 삭제하고, 이전 State로 이전한다.
                        StateResult::PopState => {
                            self.states.pop();
                            ()
                        }
                        // 기존의 state를 삭제하고 신규 State로 이전한다.
                        StateResult::Trans(s) => {
                            self.states.pop();
                            self.states.push(s);
                            ()
                        }
                        _ => (),
                    }
                }
                // 수행할 수 있는 state가 없다면 게임은 종료한다.
                None => {
                    ggez::event::quit(ctx);
                    ()
                }
            }

            // 더이상 남은 state가 없다면 종료한다.
            if self.states.is_empty() {
                ggez::event::quit(ctx);
            }
        }

        Ok(())
    }

    /// Game의 매프레임마다 Double Buffering 을 통해
    /// 화면에 그림을 그린다.
    ///
    /// * `ctx` - Context 객체
    ///
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // 이미지를 출력할 기점을 정한다.
        let dest_point = na::Point2::new(0., 0.);

        // 전체 화면을 가상의 크기로 설정한다.
        graphics::set_screen_coordinates(
            ctx,
            graphics::Rect::new(0.0, 0.0, VIRTUAL_WIDTH, VIRTUAL_HEIGHT),
        )
        .unwrap();

        // Canvas에 이미지를 그리도록 변경(double buffering)
        graphics::set_canvas(ctx, Some(&self.buffer));

        // 현재 states 값을 얻어와 해당 states의 render 를 실행한다.
        // 해당하는 renderings 은 buffer 저장된다.

        match self.states.last_mut() {
            Some(current_state) => {
                current_state.render(ctx, &mut self.reg, &mut self.buffer);

                // 이제 메인 윈도우에 그림
                graphics::set_canvas(ctx, None);

                // canvas buffer를 윈도우에 출력
                graphics::draw(
                    ctx,
                    &self.buffer,
                    graphics::DrawParam::new()
                        .dest(dest_point)
                        .src(graphics::Rect::new(0., 0., 1., 1.)),
                )?;
            }
            None => (),
        }

        // 게임이 일시 정지이면 화면에 일시 정지를 출력한다.

        graphics::present(ctx)?;

        Ok(())
    }
}
