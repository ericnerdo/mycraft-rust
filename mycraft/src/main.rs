mod world;

use world::chunk::ChunkProvider;
use crate::world::block::{BlockType};
use crate::world::chunk::{Chunk, MAX_HEIGHT};

fn main() {
    let mut chunk_provider = ChunkProvider::new(123456789);
    for x in -5..=5 {
        for y in -5..=5 {
            let chunk = chunk_provider.get_chunk(x, y);
            print_chunk_overview(chunk);
            chunk_provider.unload_chunk(x, y);
        }
        println!();
    }
}


fn print_chunk_overview(chunk: &Chunk) {
    for x in 0..16 {
        for y in 0..16 {
            let mut height = 0;
            for z in 0..MAX_HEIGHT {
                match chunk.get_block(x, y, z) {
                    Some(_) => height += 1,
                    _ => {}
                }
            }
            print!("{}  ", match chunk.get_block(x, y, height) { Some(block) => block.get_block_type(), _ => &BlockType::Stone });
        }
    }
}