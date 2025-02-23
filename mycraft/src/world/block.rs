use std::fmt;

pub enum BlockType {
    Dirt,
    Grass,
    Sand,
    Snow,
    Stone,
    Coal,
    TreeLog,
    TreeLeaf,
}

impl fmt::Display for BlockType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            BlockType::Dirt => "Dirt",
            BlockType::Grass => "Grass",
            BlockType::Sand => "Sand",
            BlockType::Snow => "Snow",
            BlockType::Stone => "Stone",
            BlockType::Coal => "Coal",
            BlockType::TreeLog => "TreeLog",
            BlockType::TreeLeaf => "TreeLeaf",
        };
        write!(f, "{name}")
    }
}

pub struct Block {
    block_type: BlockType,
    material_id: i32,
    opaque: bool,
}

impl Block {
    pub fn get_block_type(&self) -> &BlockType {
        &self.block_type
    }

    pub fn get_material_id(&self) -> i32 {
        self.material_id
    }

    pub fn is_opaque(&self) -> bool {
        self.opaque
    }
}

pub struct BlockFactory;

impl BlockFactory {
    pub fn create_dirt() -> Block {
        Block {
            block_type: BlockType::Dirt,
            material_id: 0,
            opaque: false,
        }
    }

    pub fn create_grass() -> Block {
        Block {
            block_type: BlockType::Grass,
            material_id: 1,
            opaque: false,
        }
    }

    pub fn create_sand() -> Block {
        Block {
            block_type: BlockType::Sand,
            material_id: 2,
            opaque: false,
        }
    }

    pub fn create_snow() -> Block {
        Block {
            block_type: BlockType::Snow,
            material_id: 3,
            opaque: false,
        }
    }

    pub fn create_stone() -> Block {
        Block {
            block_type: BlockType::Stone,
            material_id: 4,
            opaque: false,
        }
    }

    pub fn create_coal() -> Block {
        Block {
            block_type: BlockType::Coal,
            material_id: 5,
            opaque: false,
        }
    }

    pub fn create_tree_log() -> Block {
        Block {
            block_type: BlockType::TreeLog,
            material_id: 6,
            opaque: false,
        }
    }

    pub fn create_tree_leaf() -> Block {
        Block {
            block_type: BlockType::TreeLeaf,
            material_id: 7,
            opaque: true,
        }
    }
}
