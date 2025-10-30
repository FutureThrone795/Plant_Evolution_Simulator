use crate::plant::Plant;

use crate::render::Vertex;
use crate::render::mat4_def::Mat4;

#[derive(Copy, Clone)]
pub enum PlantModelMode {
    NoModelUpdate,
    Normal,
    Ldm,
    SuperLdm
}

impl Plant {
    pub fn push_branch_model(
        &self, 
        branch_index: usize, 
        plant_vertices: &mut Vec<Vertex>,
        plant_indices: &mut Vec<u32>,
        matrix: Mat4,
        model_mode: PlantModelMode
    ) {
        let color: [f32; 4] = [self.branches[branch_index].strength, self.branches[branch_index].photoreceptiveness, self.branches[branch_index].water_intake, 1.0];
        let branch_length_real = 1.0 + self.branches[branch_index].length * 10.0;

        match model_mode {
            PlantModelMode::Normal => {
                branch_model(color, Mat4::scale(4.0, branch_length_real, 4.0) * matrix, plant_vertices, plant_indices);
            },
            PlantModelMode::Ldm => {
                branch_model_ldm(color, Mat4::scale(4.0, branch_length_real, 4.0) * matrix, plant_vertices, plant_indices);
            },
            PlantModelMode::SuperLdm => {
                branch_model_simple_line(color, Mat4::scale(4.0, branch_length_real, 4.0) * matrix, plant_vertices, plant_indices);
            },
            PlantModelMode::NoModelUpdate => {
                panic!("push_branch_model called with model mode set to NoModelUpdate");
            }
        }
    }
}

pub fn branch_model_simple_line(color: [f32; 4], matrix: Mat4, vertices: &mut Vec<Vertex>, indices: &mut Vec<u32>) {
    let start_index = vertices.len() as u32;

    vertices.push(
        Vertex {
            position: matrix.mul_vec3_as_slice([0.0, -0.1, 0.0]),
            color: color.clone()
        });
    vertices.push(Vertex {
            position: matrix.mul_vec3_as_slice([0.0, 1.1, 0.0]),
            color: color.clone()
        });

    const INDICES: &[u32] = &[0, 1];

    for x in INDICES {
        indices.push(*x + start_index);
    }
}

pub fn branch_model(color: [f32; 4], matrix: Mat4, vertices: &mut Vec<Vertex>, indices: &mut Vec<u32>) {
    let start_index = vertices.len() as u32;

    vertices.push(
        Vertex {
            position: matrix.mul_vec3_as_slice([0.0, -0.1, 0.0]),
            color: color.clone()
        });
    
    vertices.push(
        Vertex {
            position: matrix.mul_vec3_as_slice([-0.1, 0.0, 0.1]),
            color: color.clone()
        });
    
    vertices.push(
        Vertex {
            position: matrix.mul_vec3_as_slice([0.1, 0.0, 0.1]),
            color: color.clone()
        });
    
    vertices.push(
        Vertex {
            position: matrix.mul_vec3_as_slice([0.1, 0.0, -0.1]),
            color: color.clone()
        });
    
    vertices.push(
        Vertex {
            position: matrix.mul_vec3_as_slice([-0.1, 0.0, -0.1]),
            color: color.clone()
        });
    
    vertices.push(
        Vertex {
            position: matrix.mul_vec3_as_slice([-0.1, 1.0, 0.1]),
            color: color.clone()
        });
    
    vertices.push(
        Vertex {
            position: matrix.mul_vec3_as_slice([0.1, 1.0, 0.1]),
            color: color.clone()
        });
    
    vertices.push(
        Vertex {
            position: matrix.mul_vec3_as_slice([0.1, 1.0, -0.1]),
            color: color.clone()
        });
    
    vertices.push(
        Vertex {
            position: matrix.mul_vec3_as_slice([-0.1, 1.0, -0.1]),
            color: color.clone()
        });

    vertices.push(
        Vertex {
            position: matrix.mul_vec3_as_slice([0.0, 1.1, 0.0]),
            color: color.clone()
        });

    const INDICES: &[u32] = &[
        0, 1, 2,
        0, 2, 3,
        0, 3, 4,
        0, 4, 1,

        1, 5, 2,
        2, 5, 6,

        2, 6, 3,
        3, 6, 7,

        3, 7, 4,
        4, 7, 8,

        4, 8, 1,
        1, 8, 5,

        9, 5, 8,
        9, 8, 7,
        9, 7, 6,
        9, 6, 5
    ];

    for x in INDICES {
        indices.push(*x + start_index);
    }
}

pub fn branch_model_ldm(color: [f32; 4], matrix: Mat4, vertices: &mut Vec<Vertex>, indices: &mut Vec<u32>) {
    let start_index = vertices.len() as u32;

    vertices.push(
        Vertex {
            position: matrix.mul_vec3_as_slice([-0.1, 0.0, 0.1]),
            color: color.clone()
        });
    
    vertices.push(
        Vertex {
            position: matrix.mul_vec3_as_slice([0.1, 0.0, 0.1]),
            color: color.clone()
        });
    
    vertices.push(
        Vertex {
            position: matrix.mul_vec3_as_slice([0.1, 0.0, -0.1]),
            color: color.clone()
        });
    
    vertices.push(
        Vertex {
            position: matrix.mul_vec3_as_slice([-0.1, 0.0, -0.1]),
            color: color.clone()
        });

    vertices.push(
        Vertex {
            position: matrix.mul_vec3_as_slice([-0.1, 1.0, 0.1]),
            color: color.clone()
        });
    
    vertices.push(
        Vertex {
            position: matrix.mul_vec3_as_slice([0.1, 1.0, 0.1]),
            color: color.clone()
        });
    
    vertices.push(
        Vertex {
            position: matrix.mul_vec3_as_slice([0.1, 1.0, -0.1]),
            color: color.clone()
        });
    
    vertices.push(
        Vertex {
            position: matrix.mul_vec3_as_slice([-0.1, 1.0, -0.1]),
            color: color.clone()
        });

    const INDICES: &[u32] = &[
        0, 4, 6,
        0, 6, 4,

        0, 6, 2,
        0, 2, 6,

        1, 7, 5,
        1, 5, 7,

        1, 7, 3,
        1, 3, 7
    ];

    for x in INDICES {
        indices.push(*x + start_index);
    }
}