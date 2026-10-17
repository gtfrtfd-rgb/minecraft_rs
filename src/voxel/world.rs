use super::chunk::Block;
use super::{Chunk, CHUNK_SIZE};
use crate::render::Vertex;

pub struct World {
    chunks: Vec<Chunk>,
    world_size: usize,
}

impl World {
    pub fn new() -> Self {
        let mut chunks = Vec::new();
        let world_size = 4;

        for cx in 0..world_size {
            for cz in 0..world_size {
                chunks.push(Chunk::new_terrain(cx as i32, cz as i32));
            }
        }
        Self { chunks, world_size }
    }

    fn get_block(&self, x: i32, y: i32, z: i32) -> Block {
        if x < 0 || y < 0 || z < 0 { return Block::Air; }
        let x = x as usize; let y = y as usize; let z = z as usize;
        if y >= CHUNK_SIZE { return Block::Air; }

        let chunk_x = x / CHUNK_SIZE;
        let chunk_z = z / CHUNK_SIZE;
        if chunk_x >= self.world_size || chunk_z >= self.world_size { return Block::Air; }

        let local_x = x % CHUNK_SIZE;
        let local_z = z % CHUNK_SIZE;

        let chunk_idx = chunk_x + chunk_z * self.world_size;
        if let Some(chunk) = self.chunks.get(chunk_idx) {
            chunk.get(local_x, y, local_z)
        } else {
            Block::Air
        }
    }

    pub fn generate_mesh_data(&self) -> Vec<Vertex> {
        let mut vertices = Vec::new();

        for (chunk_idx, chunk) in self.chunks.iter().enumerate() {
            let chunk_x = (chunk_idx % self.world_size) as i32;
            let chunk_z = (chunk_idx / self.world_size) as i32;
            let offset_x = chunk_x * CHUNK_SIZE as i32;
            let offset_z = chunk_z * CHUNK_SIZE as i32;

            for y in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    for z in 0..CHUNK_SIZE {
                        let block = chunk.get(x, y, z);
                        if !block.is_solid() { continue; }

                        let wx = offset_x + x as i32;
                        let wy = y as i32;
                        let wz = offset_z + z as i32;
                        let fx = wx as f32; let fy = wy as f32; let fz = wz as f32;

                        // Добавляем Padding (отступ) в 0.001, чтобы UV не захватывали соседние пиксели (устраняет черные полосы)
                        let pad = 0.001;
                        let uv_rect = match block {
                            Block::Grass => [0.0 + pad, 0.0 + pad, 0.25 - pad, 0.25 - pad],
                            Block::Dirt => [0.25 + pad, 0.0 + pad, 0.5 - pad, 0.25 - pad],
                            Block::Stone => [0.5 + pad, 0.0 + pad, 0.75 - pad, 0.25 - pad],
                            _ => [0.0 + pad, 0.0 + pad, 0.25 - pad, 0.25 - pad],
                        };

                        let light_top = [1.0, 1.0, 1.0];
                        let light_side = [0.8, 0.8, 0.8];
                        let light_bottom = [0.5, 0.5, 0.5];

                        // Верхняя грань (Y+) - обход против часовой стрелки (CCW) при взгляде сверху
                        if !self.get_block(wx, wy + 1, wz).is_solid() {
                            let top = [
                                [fx, fy + 1.0, fz],
                                [fx, fy + 1.0, fz + 1.0],
                                [fx + 1.0, fy + 1.0, fz + 1.0],
                                [fx + 1.0, fy + 1.0, fz],
                            ];
                            let uv = [
                                [uv_rect[0], uv_rect[1]],
                                [uv_rect[0], uv_rect[3]],
                                [uv_rect[2], uv_rect[3]],
                                [uv_rect[2], uv_rect[1]],
                            ];
                            add_quad(&mut vertices, top, uv, light_top);
                        }

                        // Нижняя грань (Y-)
                        if !self.get_block(wx, wy - 1, wz).is_solid() {
                            let bottom = [
                                [fx + 1.0, fy, fz],
                                [fx, fy, fz],
                                [fx, fy, fz + 1.0],
                                [fx + 1.0, fy, fz + 1.0],
                            ];
                            let uv = [
                                [uv_rect[2], uv_rect[1]],
                                [uv_rect[0], uv_rect[1]],
                                [uv_rect[0], uv_rect[3]],
                                [uv_rect[2], uv_rect[3]],
                            ];
                            add_quad(&mut vertices, bottom, uv, light_bottom);
                        }

                        // Передняя грань (Z+)
                        if !self.get_block(wx, wy, wz + 1).is_solid() {
                            let front = [
                                [fx, fy, fz + 1.0],
                                [fx + 1.0, fy, fz + 1.0],
                                [fx + 1.0, fy + 1.0, fz + 1.0],
                                [fx, fy + 1.0, fz + 1.0],
                            ];
                            let uv = [
                                [uv_rect[0], uv_rect[3]],
                                [uv_rect[2], uv_rect[3]],
                                [uv_rect[2], uv_rect[1]],
                                [uv_rect[0], uv_rect[1]],
                            ];
                            add_quad(&mut vertices, front, uv, light_side);
                        }

                        // Задняя грань (Z-) - исправлен обход (CCW)
                        if !self.get_block(wx, wy, wz - 1).is_solid() {
                            let back = [
                                [fx + 1.0, fy, fz],
                                [fx, fy, fz],
                                [fx, fy + 1.0, fz],
                                [fx + 1.0, fy + 1.0, fz],
                            ];
                            let uv = [
                                [uv_rect[2], uv_rect[3]],
                                [uv_rect[0], uv_rect[3]],
                                [uv_rect[0], uv_rect[1]],
                                [uv_rect[2], uv_rect[1]],
                            ];
                            add_quad(&mut vertices, back, uv, light_side);
                        }

                        // Правая грань (X+)
                        if !self.get_block(wx + 1, wy, wz).is_solid() {
                            let right = [
                                [fx + 1.0, fy, fz],
                                [fx + 1.0, fy, fz + 1.0],
                                [fx + 1.0, fy + 1.0, fz + 1.0],
                                [fx + 1.0, fy + 1.0, fz],
                            ];
                            let uv = [
                                [uv_rect[0], uv_rect[3]],
                                [uv_rect[2], uv_rect[3]],
                                [uv_rect[2], uv_rect[1]],
                                [uv_rect[0], uv_rect[1]],
                            ];
                            add_quad(&mut vertices, right, uv, light_side);
                        }

                        // Левая грань (X-) - исправлен обход (CCW)
                        if !self.get_block(wx - 1, wy, wz).is_solid() {
                            let left = [
                                [fx, fy, fz + 1.0],
                                [fx, fy, fz],
                                [fx, fy + 1.0, fz],
                                [fx, fy + 1.0, fz + 1.0],
                            ];
                            let uv = [
                                [uv_rect[2], uv_rect[3]],
                                [uv_rect[0], uv_rect[3]],
                                [uv_rect[0], uv_rect[1]],
                                [uv_rect[2], uv_rect[1]],
                            ];
                            add_quad(&mut vertices, left, uv, light_side);
                        }
                    }
                }
            }
        }

        vertices
    }
}

fn add_quad(vertices: &mut Vec<Vertex>, positions: [[f32; 3]; 4], uvs: [[f32; 2]; 4], color: [f32; 3]) {
    let indices = [0, 1, 2, 0, 2, 3];
    for &i in &indices {
        vertices.push(Vertex {
            position: positions[i],
            color,
            uv: uvs[i],
        });
    }
}