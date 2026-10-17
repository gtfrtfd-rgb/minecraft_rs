pub const CHUNK_SIZE: usize = 16;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Block {
    Air,
    Grass,
    Dirt,
    Stone,
}

impl Block {
    pub fn is_solid(&self) -> bool {
        !matches!(self, Block::Air)
    }
}

pub struct Chunk {
    pub blocks: [[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
}

impl Chunk {
    pub fn new_flat() -> Self {
        let mut blocks = [[[Block::Air; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    blocks[y][x][z] = match y {
                        0 => Block::Stone,
                        1..=4 => Block::Dirt,
                        5 => Block::Grass,
                        _ => Block::Air,
                    };
                }
            }
        }

        Self { blocks }
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> Block {
        self.blocks[y][x][z]
    }
}