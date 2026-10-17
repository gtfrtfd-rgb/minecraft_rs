use super::{Chunk, CHUNK_SIZE};
use crate::render::Vertex;

pub struct World {
    chunks: Vec<Chunk>,
}

impl World {
    pub fn new() -> Self {
        let mut chunks = Vec::new();
        for _cx in 0..2 {
            for _cz in 0..2 {
                chunks.push(Chunk::new_flat());
            }
        }
        Self { chunks }
    }

    pub fn generate_mesh_data(&self) -> Vec<Vertex> {
        let mut vertices = Vec::new();

        for (chunk_idx, chunk) in self.chunks.iter().enumerate() {
            let offset_x = (chunk_idx % 2) as f32 * CHUNK_SIZE as f32;
            let offset_z = (chunk_idx / 2) as f32 * CHUNK_SIZE as f32;

            for y in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    for z in 0..CHUNK_SIZE {
                        if !chunk.get(x, y, z).is_solid() {
                            continue;
                        }

                        let wx = offset_x + x as f32;
                        let wy = y as f32;
                        let wz = offset_z + z as f32;

                        // Верхняя грань (Y+)
                        let top = [
                            [wx, wy + 1.0, wz],
                            [wx, wy + 1.0, wz + 1.0],
                            [wx + 1.0, wy + 1.0, wz + 1.0],
                            [wx + 1.0, wy + 1.0, wz],
                        ];
                        add_quad(&mut vertices, top, [0.3, 0.8, 0.2]);

                        // Нижняя грань (Y-) - исправлен обход (CCW)
                        let bottom = [
                            [wx + 1.0, wy, wz],
                            [wx, wy, wz],
                            [wx, wy, wz + 1.0],
                            [wx + 1.0, wy, wz + 1.0],
                        ];
                        add_quad(&mut vertices, bottom, [0.3, 0.8, 0.2]);

                        // Передняя грань (Z+)
                        let front = [
                            [wx, wy, wz + 1.0],
                            [wx + 1.0, wy, wz + 1.0],
                            [wx + 1.0, wy + 1.0, wz + 1.0],
                            [wx, wy + 1.0, wz + 1.0],
                        ];
                        add_quad(&mut vertices, front, [0.2, 0.6, 0.1]);

                        // Задняя грань (Z-) - исправлен обход (CCW)
                        let back = [
                            [wx, wy, wz],
                            [wx + 1.0, wy, wz],
                            [wx + 1.0, wy + 1.0, wz],
                            [wx, wy + 1.0, wz],
                        ];
                        add_quad(&mut vertices, back, [0.2, 0.6, 0.1]);

                        // Правая грань (X+) - исправлен обход (CCW)
                        let right = [
                            [wx + 1.0, wy, wz],
                            [wx + 1.0, wy, wz + 1.0],
                            [wx + 1.0, wy + 1.0, wz + 1.0],
                            [wx + 1.0, wy + 1.0, wz],
                        ];
                        add_quad(&mut vertices, right, [0.15, 0.5, 0.1]);

                        // Левая грань (X-) - исправлен обход (CCW)
                        let left = [
                            [wx, wy, wz + 1.0],
                            [wx, wy, wz],
                            [wx, wy + 1.0, wz],
                            [wx, wy + 1.0, wz + 1.0],
                        ];
                        add_quad(&mut vertices, left, [0.15, 0.5, 0.1]);
                    }
                }
            }
        }

        vertices
    }
}

fn add_quad(vertices: &mut Vec<Vertex>, positions: [[f32; 3]; 4], color: [f32; 3]) {
    let indices = [0, 1, 2, 0, 2, 3];
    for &i in &indices {
        vertices.push(Vertex {
            position: positions[i],
            color,
        });
    }
}