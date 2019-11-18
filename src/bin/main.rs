use ggez;
use ggez::event;
use ggez::GameResult;
use ggez_breakout;

use ggez_breakout::game::{WINDOW_HEIGHT, WINDOW_WIDTH};

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Hello World", "siabard");
    let (ctx, event_loop) = &mut cb
        .add_resource_path("./resources")
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .window_setup(ggez::conf::WindowSetup::default().title("Skeleton: ggez"))
        .build()?;

    ggez::graphics::set_default_filter(ctx, ggez::graphics::FilterMode::Linear);
    let state = &mut ggez_breakout::game::Game::new(ctx)?;

    event::run(ctx, event_loop, state)
}
