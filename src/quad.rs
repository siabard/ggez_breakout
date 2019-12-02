use ggez::nalgebra as na;
use ggez::Context;
use std::collections::HashMap;
use std::path::Path;

pub struct Quad {
    source: ggez::graphics::Image,
    width: f32,
    height: f32,
    sprite: HashMap<i32, ggez::graphics::Rect>,
}

impl Quad {
    pub fn new(ctx: &mut Context, path: &Path) -> Quad {
        let source = ggez::graphics::Image::new(ctx, path).unwrap();
        let width = source.width() as f32;
        let height = source.height() as f32;
        let sprite = HashMap::<i32, ggez::graphics::Rect>::new();

        Quad {
            source,
            width,
            height,
            sprite,
        }
    }

    pub fn add_sprite(&mut self, key: i32, x: f32, y: f32, w: f32, h: f32) {
        let x_ratio = if x >= self.width { 1. } else { x / self.width };

        let y_ratio = if y >= self.height {
            1.
        } else {
            y / self.height
        };

        let w_ratio = if w >= self.width { 1. } else { w / self.width };

        let h_ratio = if h >= self.height {
            1.
        } else {
            h / self.height
        };

        self.sprite.insert(
            key,
            ggez::graphics::Rect::new(x_ratio, y_ratio, w_ratio, h_ratio),
        );
    }

    pub fn draw_sprite(&mut self, ctx: &mut Context, key: i32, x: f32, y: f32) {
        let dest = na::Point2::new(x, y);

        ggez::graphics::draw(
            ctx,
            &self.source,
            ggez::graphics::DrawParam::new()
                .dest(dest)
                .src(*self.sprite.get(&key).unwrap()),
        )
        .unwrap();
    }
}
