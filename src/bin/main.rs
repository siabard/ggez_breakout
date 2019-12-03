use ggez::{self, event, filesystem, GameResult};
use ggez_breakout;
use std::path;

use ggez_breakout::save::*;

use std::io::{Read, Write};

use ggez_breakout::game::{WINDOW_HEIGHT, WINDOW_WIDTH};

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Breakout", "siabard");
    let (ctx, event_loop) = &mut cb
        .add_resource_path("./resources")
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .window_setup(ggez::conf::WindowSetup::default().title("Breakout CS50 GGEZ porting"))
        .build()?;

    ggez::graphics::set_default_filter(ctx, ggez::graphics::FilterMode::Linear);
    let state = &mut ggez_breakout::game::Game::new(ctx)?;

    //파일 저장 테스트

    println!("Resource stats:");
    filesystem::print_all(ctx);

    let file_opt = filesystem::OpenOptions::new().create(true).append(true);
    let mut file = filesystem::open_options(ctx, path::Path::new("/testfile.txt"), file_opt)?;

    let bytes = "테스트".as_bytes();
    file.write_all(bytes)?;

    let mut save = Save::new();
    save.init(ctx);

    event::run(ctx, event_loop, state)
}
