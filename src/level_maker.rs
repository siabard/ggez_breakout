use crate::objects::Block;
use rand::*;

pub fn create_map(_level: i32) -> Vec<Block> {
    let mut blocks = Vec::<Block>::new();

    let mut rng = thread_rng();
    let rows = rng.gen_range(2, 6);
    let cols = rng.gen_range(7, 13);

    for y in 1..rows {
        for x in 1..cols {
            let block = Block::new(
                ((x - 1) * 32 + 8 + (13 - cols) * 16) as f32,
                (y * 16) as f32,
            );

            blocks.push(block);
        }
    }

    blocks
}
