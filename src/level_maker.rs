use crate::objects::Block;
use crate::reg::Reg;
use ggez::Context;
use rand::*;

pub fn create_map(ctx: &mut Context, reg: &mut Reg, _level: i32) -> Vec<Block> {
    let mut blocks = Vec::<Block>::new();

    let mut rng = thread_rng();
    let rows = rng.gen_range(1, 5);
    let cols = rng.gen_range(7, 13);

    for y in 1..rows {
        for x in 1..cols {
            let block = Block::new(
                ctx,
                reg,
                ((x - 1) * 32 + 8 + (13 - cols) * 16) as f32,
                (y * 16) as f32,
            );

            blocks.push(block);
        }
    }

    blocks
}
