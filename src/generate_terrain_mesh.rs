use crate::terrain::{Terrain, TERRAIN_CELL_WIDTH, TERRAIN_GRID_ROWS};
use crate::vertex_def::Vertex;

pub fn generate_terrain_mesh(terrain: &Terrain) -> (Vec<Vertex>, Vec<u32>, Vec<Vertex>, Vec<u32>) {
    let mut vertices: Vec<Vertex> = Vec::with_capacity((TERRAIN_GRID_ROWS + 1) * (TERRAIN_GRID_ROWS + 1));
    let mut indices: Vec<u32> = Vec::new();

    let mut water_vertices: Vec<Vertex> = Vec::with_capacity((TERRAIN_GRID_ROWS + 1) * (TERRAIN_GRID_ROWS + 1));
    let mut water_indices: Vec<u32> = Vec::new();

    for x in 0..(TERRAIN_GRID_ROWS + 1) {
        for z in 0..(TERRAIN_GRID_ROWS + 1) {
            let x_mapped = x.rem_euclid(TERRAIN_GRID_ROWS);
            let z_mapped = z.rem_euclid(TERRAIN_GRID_ROWS);

            vertices.push(Vertex {
                position: [
                            x as f32 * TERRAIN_CELL_WIDTH, 
                            terrain.grid[x_mapped][z_mapped].height, 
                            z as f32 * TERRAIN_CELL_WIDTH
                            ],
                color: terrain.grid[x_mapped][z_mapped].ground_type.color()
            });

            water_vertices.push(Vertex {
                position: [
                            x as f32 * TERRAIN_CELL_WIDTH, 
                            0.0, 
                            z as f32 * TERRAIN_CELL_WIDTH
                            ],
                color: [0.0, 0.5, 1.0, 0.5]
            });

            if x < TERRAIN_GRID_ROWS && z < TERRAIN_GRID_ROWS {
                let top_left        = (x *          (TERRAIN_GRID_ROWS + 1) +   z       ) as u32;
                let top_right       = (x *          (TERRAIN_GRID_ROWS + 1) +   (z + 1) ) as u32;
                let bottom_left     = ((x + 1) *    (TERRAIN_GRID_ROWS + 1) +   z       ) as u32;
                let bottom_right    = ((x + 1) *    (TERRAIN_GRID_ROWS + 1) +   (z + 1) ) as u32;
                
                if (x ^ z << 3).rem_euclid(3) & 1 == 0 {
                    indices.push(top_left);
                    indices.push(bottom_left);
                    indices.push(top_right);

                    water_indices.push(top_left);
                    water_indices.push(bottom_left);
                    water_indices.push(top_right);

                    // second triangle
                    indices.push(bottom_left);
                    indices.push(bottom_right);
                    indices.push(top_right);

                    water_indices.push(bottom_left);
                    water_indices.push(bottom_right);
                    water_indices.push(top_right);
                    continue;
                }
                // first triangle
                indices.push(top_left);
                indices.push(bottom_left);
                indices.push(bottom_right);

                water_indices.push(top_left);
                water_indices.push(bottom_left);
                water_indices.push(bottom_right);

                // second triangle
                indices.push(top_left);
                indices.push(bottom_right);
                indices.push(top_right);

                water_indices.push(top_left);
                water_indices.push(bottom_right);
                water_indices.push(top_right);
            }
        }
    }

    return (vertices, indices, water_vertices, water_indices);
        
}