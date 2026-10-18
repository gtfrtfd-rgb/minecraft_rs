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
    // Генерация рельефа с холмами.
    pub fn new_terrain(chunk_x: i32, chunk_z: i32) -> Self {
        let mut blocks = [[[Block::Air; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let wx = chunk_x * CHUNK_SIZE as i32 + x as i32;
                let wz = chunk_z * CHUNK_SIZE as i32 + z as i32;

                let height_f = 8.0
                    + 4.0 * ((wx as f32 * 0.2).sin() + (wz as f32 * 0.2).cos());
                let height = height_f
                    .max(1.0)
                    .min((CHUNK_SIZE - 2) as f32) as usize;

                for y in 0..CHUNK_SIZE {
                    if y < height {
                        blocks[y][x][z] = if y == height - 1 {
                            Block::Grass
                        } else if y > height.saturating_sub(4) {
                            Block::Dirt
                        } else {
                            Block::Stone
                        };
                    }
                }
            }
        }

        Self { blocks }
    }

    #[inline]
    pub fn get(&self, x: usize, y: usize, z: usize) -> Block {
        self.blocks[y][x][z]
    }

    /// Установить блок. Понадобится для будущих фич (копание/строительство).
    #[allow(dead_code)]
    #[inline]
    pub fn set(&mut self, x: usize, y: usize, z: usize, block: Block) {
        self.blocks[y][x][z] = block;
    }
}