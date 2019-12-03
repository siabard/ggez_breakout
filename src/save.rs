use ggez::error::GameResult;
use ggez::filesystem;
use ggez::Context;
use std::io::{Read, Write};
use std::path;

pub const SAVE_FILE: &'static str = "/savefile.txt";

pub struct Save {
    pub contents: Vec<String>,
}

impl Save {
    pub fn new() -> Save {
        Save { contents: vec![] }
    }

    /// 해당 파일 존재 여부
    pub fn exists(&self, ctx: &mut Context) -> bool {
        let file = filesystem::open(ctx, path::Path::new(SAVE_FILE));

        return match file {
            Ok(_) => true,
            Err(_) => false,
        };
    }
    /// 강제 초기화 루틴
    pub fn init(&mut self, ctx: &mut Context) {
        let options = filesystem::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true);

        let mut file = filesystem::open_options(ctx, path::Path::new(SAVE_FILE), options).unwrap();
        let result = format!("{}\n{}\n", "test", "test2");
        file.write_all(result.as_bytes()).unwrap();
    }
}
