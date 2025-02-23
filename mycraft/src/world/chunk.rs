use crate::world::biome;
use crate::world::biome::Biome;
use crate::world::block::{Block, BlockFactory};
use noise::{NoiseFn, Simplex};
use std::cmp::{max, min};
use std::collections::HashMap;

// Max values
pub const MAX_HEIGHT: i32 = 50;

// Increments
const HEIGHT_INCREMENT: f64 = 0.01;

const TEMPERATURE_INCREMENT: f64 = 0.013;
const HUMIDITY_INCREMENT: f64 = 0.0025;

const CAVE_INCREMENT_XY: f64 = 0.03;
const CAVE_INCREMENT_Z: f64 = 0.006;

const BLOCK_INCREMENT: f64 = 0.2;

const TREE_TRUNK_INCREMENT: f64 = 0.75;
const TREE_BRANCH_INCREMENT: f64 = 0.9;
const TREE_LEAF_INCREMENT: f64 = 0.6;

// Misc
const CHUNK_SIZE: i32 = 16;
const CAVE_MULTIPLIER: i32 = 3;
const MIN_HEIGHT: i32 = 1;
const HEIGHT_AMPLIFIER: f64 = 1.2;

pub struct Chunk {
    pub x: i32,
    pub y: i32,
    pub chunk_map: HashMap<(i32, i32, i32), Block>,
}

impl Chunk {
    pub fn get_block(&self, x: i32, y: i32, z: i32) -> Option<&Block> {
        self.chunk_map.get(&(x, y, z))
    }

    pub fn generate(noise: &Simplex, x: i32, y: i32) -> Self {
        let mut chunk = Chunk {
            x,
            y,
            chunk_map: HashMap::new(),
        };

        chunk.generate_terrain(noise);
        chunk.generate_caves(noise);
        chunk.generate_block_types(noise);
        chunk.generate_trees(noise);

        chunk
    }

    fn generate_terrain(&mut self, noise: &Simplex) {
        let mut height_x_offset: f64 = (self.x * CHUNK_SIZE) as f64 * HEIGHT_INCREMENT;

        for i in 0..CHUNK_SIZE {
            let mut height_y_offset: f64 = (self.y * CHUNK_SIZE) as f64 * HEIGHT_INCREMENT;

            for j in 0..CHUNK_SIZE {
                let height_noise_value: f64 =
                    noise.get([height_x_offset, height_y_offset, 0.0]).abs();

                let height: i32 = max(
                    MIN_HEIGHT,
                    (MAX_HEIGHT as f64 * HEIGHT_AMPLIFIER * height_noise_value).floor() as i32,
                );

                for k in 0..min(MAX_HEIGHT, height) {
                    self.chunk_map
                        .insert((i, j, k), BlockFactory::create_stone());
                }

                height_y_offset += HEIGHT_INCREMENT;
            }

            height_x_offset += HEIGHT_INCREMENT;
        }
    }

    fn generate_caves(&mut self, noise: &Simplex) {
        let height_x_offset: f64 = (self.x * CHUNK_SIZE) as f64 * HEIGHT_INCREMENT;
        let mut cave_x_offset: f64 = (self.x * CHUNK_SIZE) as f64 * CAVE_INCREMENT_XY;

        for i in 0..CHUNK_SIZE {
            let mut cave_y_offset: f64 = (self.y * CHUNK_SIZE) as f64 * CAVE_INCREMENT_XY;
            let height_y_offset: f64 = (self.y * CHUNK_SIZE) as f64 * HEIGHT_INCREMENT;

            for j in 0..CHUNK_SIZE {
                let height_noise_value: f64 =
                    noise.get([height_x_offset, height_y_offset, 0.0]).abs();
                let height: i32 = (MAX_HEIGHT as f64 * height_noise_value).floor() as i32;

                for k in 0..height {
                    let cave_z_offset = (k * CHUNK_SIZE) as f64 * CAVE_INCREMENT_Z;

                    let cave_noise_value: f64 = noise
                        .get([cave_x_offset, cave_y_offset, cave_z_offset])
                        .abs();

                    if k > 0 && cave_noise_value < 0.2 {
                        self.dig_cross_section(i, j, k);
                    }
                }

                cave_y_offset += CAVE_INCREMENT_XY;
            }

            cave_x_offset += CAVE_INCREMENT_XY;
        }
    }

    fn dig_cross_section(&mut self, x: i32, y: i32, z: i32) {
        for iz in -CAVE_MULTIPLIER..=CAVE_MULTIPLIER {
            for iy in -iz..=iz {
                let dx: i32 = (iz * iz - iy * iy).isqrt();
                for ix in -dx..=dx {
                    let i = x + ix;
                    let j = y + iy;
                    let k = z + iz;
                    if k > 0 && self.chunk_map.contains_key(&(i, j, k)) {
                        self.chunk_map.remove(&(i, j, k));
                    }
                }
            }
        }
    }

    fn generate_block_types(&mut self, noise: &Simplex) {
        for block_entry in self
            .chunk_map
            .keys()
            .copied()
            .collect::<Vec<(i32, i32, i32)>>()
        {
            let x_coord: i32 = block_entry.0;
            let y_coord: i32 = block_entry.1;
            let z_coord: i32 = block_entry.2;

            let temperature_x_offset: f64 =
                (self.x * CHUNK_SIZE + x_coord) as f64 * TEMPERATURE_INCREMENT;
            let temperature_y_offset: f64 =
                (self.y * CHUNK_SIZE + y_coord) as f64 * TEMPERATURE_INCREMENT;
            let humidity_x_offset: f64 =
                (self.x * CHUNK_SIZE + x_coord) as f64 * HUMIDITY_INCREMENT;
            let humidity_y_offset: f64 =
                (self.y * CHUNK_SIZE + y_coord) as f64 * HUMIDITY_INCREMENT;

            let block_x_offset: f64 = (self.x * CHUNK_SIZE + x_coord) as f64 * BLOCK_INCREMENT;
            let block_y_offset: f64 = (self.y * CHUNK_SIZE + y_coord) as f64 * BLOCK_INCREMENT;
            let block_z_offset: f64 = z_coord as f64 * BLOCK_INCREMENT;

            let temperature_noise_value: f64 = noise
                .get([temperature_x_offset, temperature_y_offset, 0.0])
                .abs();
            let humidity_noise_value: f64 =
                noise.get([humidity_x_offset, humidity_y_offset, 0.0]).abs();
            let block_noise_value: f64 = noise
                .get([block_x_offset, block_y_offset, block_z_offset])
                .abs();

            let biome = biome::get_biome_by_params(temperature_noise_value, humidity_noise_value);

            if z_coord > self.get_max_height(noise, x_coord, y_coord) - 5 {
                if block_noise_value < 0.95 {
                    self.chunk_map.remove(&(x_coord, y_coord, z_coord));

                    match biome {
                        Biome::Grass => {
                            if self
                                .chunk_map
                                .contains_key(&(x_coord, y_coord, z_coord + 1))
                            {
                                self.chunk_map.insert(
                                    (x_coord, y_coord, z_coord),
                                    BlockFactory::create_dirt(),
                                );
                            } else {
                                self.chunk_map.insert(
                                    (x_coord, y_coord, z_coord),
                                    BlockFactory::create_grass(),
                                );
                            }
                        }
                        Biome::Desert => {
                            self.chunk_map
                                .insert((x_coord, y_coord, z_coord), BlockFactory::create_sand());
                        }
                        Biome::Snow => {
                            self.chunk_map
                                .insert((x_coord, y_coord, z_coord), BlockFactory::create_snow());
                        }
                        _ => {
                            self.chunk_map
                                .insert((x_coord, y_coord, z_coord), BlockFactory::create_dirt());
                        }
                    }
                }
            } else if z_coord > 0 {
                if block_noise_value < 0.2 {
                    self.chunk_map
                        .insert((x_coord, y_coord, z_coord), BlockFactory::create_coal());
                }
            }
        }
    }

    fn get_max_height(&self, noise: &Simplex, x: i32, y: i32) -> i32 {
        let height_x_offset: f64 = (self.x * CHUNK_SIZE + x) as f64 * HEIGHT_INCREMENT;
        let height_y_offset: f64 = (self.y * CHUNK_SIZE + y) as f64 * HEIGHT_INCREMENT;

        let height_noise_value: f64 = noise.get([height_x_offset, height_y_offset, 0.0]).abs();
        let height: i32 = (MAX_HEIGHT as f64 * height_noise_value).floor() as i32;

        for z in height..=0 {
            if self.chunk_map.contains_key(&(x, y, z)) {
                return z;
            }
        }

        0
    }

    fn generate_trees(&mut self, noise: &Simplex) {
        let mut tree_x_offset: f64 = (self.x * CHUNK_SIZE) as f64 * TREE_TRUNK_INCREMENT;

        for i in 0..CHUNK_SIZE {
            let mut tree_y_offset: f64 = (self.y * CHUNK_SIZE) as f64 * TREE_TRUNK_INCREMENT;

            for j in 0..CHUNK_SIZE {
                let tree_trunk_noise_value: f64 =
                    noise.get([tree_x_offset, tree_y_offset, 0.0]).abs();

                if tree_trunk_noise_value > 0.5 && tree_trunk_noise_value < 0.50425 {
                    self.generate_tree(noise, i, j);
                }

                tree_y_offset += TREE_TRUNK_INCREMENT;
            }

            tree_x_offset += TREE_TRUNK_INCREMENT;
        }
    }

    fn generate_tree(&mut self, noise: &Simplex, x: i32, y: i32) {
        let mut height = self.get_max_height(noise, x, y) + 1;

        for _ in 0..4 {
            self.chunk_map
                .insert((x, y, height), BlockFactory::create_tree_log());
            height += 1;
        }

        let tree_branch_x_offset: f64 = (self.x * CHUNK_SIZE + x) as f64 * TREE_BRANCH_INCREMENT;
        let tree_branch_y_offset: f64 = (self.y * CHUNK_SIZE + y) as f64 * TREE_BRANCH_INCREMENT;
        let mut tree_branch_z_offset: f64 = height as f64 * TREE_BRANCH_INCREMENT;

        let mut tree_branch_noise_value: f64 = noise
            .get([
                tree_branch_x_offset,
                tree_branch_y_offset,
                tree_branch_z_offset,
            ])
            .abs();

        while tree_branch_noise_value < 0.4 {
            if self.chunk_map.contains_key(&(x, y, height)) {
                self.chunk_map.remove(&(x, y, height));
            }

            self.chunk_map
                .insert((x, y, height), BlockFactory::create_tree_log());

            let left_noise: f64 = noise
                .get([
                    tree_branch_x_offset + TREE_BRANCH_INCREMENT,
                    tree_branch_y_offset,
                    tree_branch_z_offset,
                ])
                .abs();
            let right_noise: f64 = noise
                .get([
                    tree_branch_x_offset - TREE_BRANCH_INCREMENT,
                    tree_branch_y_offset,
                    tree_branch_z_offset,
                ])
                .abs();
            let up_noise: f64 = noise
                .get([
                    tree_branch_x_offset,
                    tree_branch_y_offset + TREE_BRANCH_INCREMENT,
                    tree_branch_z_offset,
                ])
                .abs();
            let down_noise: f64 = noise
                .get([
                    tree_branch_x_offset,
                    tree_branch_y_offset - TREE_BRANCH_INCREMENT,
                    tree_branch_z_offset,
                ])
                .abs();

            if left_noise < 0.4 {
                self.generate_branch(noise, x - 1, y, height);
            }

            if right_noise < 0.4 {
                self.generate_branch(noise, x + 1, y, height);
            }

            if down_noise < 0.4 {
                self.generate_branch(noise, x, y - 1, height);
            }

            if up_noise < 0.4 {
                self.generate_branch(noise, x, y + 1, height);
            }

            height += 1;
            tree_branch_z_offset += TREE_BRANCH_INCREMENT;
            tree_branch_noise_value = noise
                .get([
                    tree_branch_x_offset,
                    tree_branch_y_offset,
                    tree_branch_z_offset,
                ])
                .abs();
        }

        self.generate_leaves(noise, x, y, height - 1, 2);
    }

    fn generate_branch(&mut self, noise: &Simplex, x: i32, y: i32, z: i32) {
        self.chunk_map
            .insert((x, y, y), BlockFactory::create_tree_log());
        self.generate_leaves(noise, x, y, z, 1);
    }

    fn generate_leaves(
        &mut self,
        noise: &Simplex,
        center_x: i32,
        center_y: i32,
        center_z: i32,
        radius: i32,
    ) {
        for iz in -radius..=radius {
            let r: i32 = radius - iz.abs();
            for iy in -r..=r {
                let dx: i32 = (r * r - iy * iy).abs().isqrt();
                for ix in -dx..=dx {
                    let i = center_x + ix;
                    let j = center_y + iy;
                    let k = center_z + iz;

                    if !self.chunk_map.contains_key(&(i, j, k)) {
                        let tree_leaf_x_offset: f64 =
                            (self.x * CHUNK_SIZE + i) as f64 * TREE_LEAF_INCREMENT;
                        let tree_leaf_y_offset: f64 =
                            (self.y * CHUNK_SIZE + j) as f64 * TREE_LEAF_INCREMENT;
                        let tree_leaf_z_offset: f64 = k as f64 * TREE_LEAF_INCREMENT;

                        let tree_leaf_noise: f64 = noise
                            .get([tree_leaf_x_offset, tree_leaf_y_offset, tree_leaf_z_offset])
                            .abs();

                        if tree_leaf_noise < 0.85 {
                            self.chunk_map
                                .insert((i, j, k), BlockFactory::create_tree_leaf());
                        }
                    }
                }
            }
        }
    }
}

pub struct ChunkProvider {
    seed: u32,
    noise: Simplex,
    loaded_chunks: HashMap<(i32, i32), Chunk>,
}

impl ChunkProvider {
    pub fn default() -> Self {
        Self::new(0)
    }

    pub fn new(seed: u32) -> Self {
        Self {
            seed,
            noise: Simplex::new(seed),
            loaded_chunks: HashMap::new(),
        }
    }

    pub fn get_chunk(&mut self, x: i32, y: i32) -> &Chunk {
        let chunk = self
            .loaded_chunks
            .entry((x, y))
            .or_insert_with(|| Chunk::generate(&self.noise, x, y));

        chunk
    }

    pub fn unload_chunk(&mut self, x: i32, y: i32) {
        self.loaded_chunks.remove(&(x, y));
    }
}
