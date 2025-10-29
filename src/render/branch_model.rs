use crate::render::Vertex;
use crate::render::mat4_def::Mat4;

pub fn generate_branch_model(color: [f32; 4], matrix: Mat4, start_index: u32) -> (Vec<Vertex>, Vec<u32>) {
    let vertices: Vec<Vertex> = vec![
        Vertex {
            position: matrix.mul_vec3_as_slice([0.0, -0.1, 0.0]),
            color: color.clone()
        },

        Vertex {
            position: matrix.mul_vec3_as_slice([-0.1, 0.0, 0.1]),
            color: color.clone()
        },
        Vertex {
            position: matrix.mul_vec3_as_slice([0.1, 0.0, 0.1]),
            color: color.clone()
        },
        Vertex {
            position: matrix.mul_vec3_as_slice([0.1, 0.0, -0.1]),
            color: color.clone()
        },
        Vertex {
            position: matrix.mul_vec3_as_slice([-0.1, 0.0, -0.1]),
            color: color.clone()
        },

        Vertex {
            position: matrix.mul_vec3_as_slice([-0.1, 1.0, 0.1]),
            color: color.clone()
        },
        Vertex {
            position: matrix.mul_vec3_as_slice([0.1, 1.0, 0.1]),
            color: color.clone()
        },
        Vertex {
            position: matrix.mul_vec3_as_slice([0.1, 1.0, -0.1]),
            color: color.clone()
        },
        Vertex {
            position: matrix.mul_vec3_as_slice([-0.1, 1.0, -0.1]),
            color: color.clone()
        },

        Vertex {
            position: matrix.mul_vec3_as_slice([0.0, 1.1, 0.0]),
            color: color.clone()
        }
    ];

    let indices: Vec<u32> = vec![
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
    ].iter().map(|x| *x + start_index).collect();

    return (vertices, indices);
}

pub fn generate_branch_model_ldm(color: [f32; 4], matrix: Mat4, start_index: u32) -> (Vec<Vertex>, Vec<u32>) {
    let vertices: Vec<Vertex> = vec![
        Vertex {
            position: matrix.mul_vec3_as_slice([-0.1, 0.0, 0.1]),
            color: color.clone()
        },
        Vertex {
            position: matrix.mul_vec3_as_slice([0.1, 0.0, 0.1]),
            color: color.clone()
        },
        Vertex {
            position: matrix.mul_vec3_as_slice([0.1, 0.0, -0.1]),
            color: color.clone()
        },
        Vertex {
            position: matrix.mul_vec3_as_slice([-0.1, 0.0, -0.1]),
            color: color.clone()
        },

        Vertex {
            position: matrix.mul_vec3_as_slice([-0.1, 1.0, 0.1]),
            color: color.clone()
        },
        Vertex {
            position: matrix.mul_vec3_as_slice([0.1, 1.0, 0.1]),
            color: color.clone()
        },
        Vertex {
            position: matrix.mul_vec3_as_slice([0.1, 1.0, -0.1]),
            color: color.clone()
        },
        Vertex {
            position: matrix.mul_vec3_as_slice([-0.1, 1.0, -0.1]),
            color: color.clone()
        },
    ];

    let indices: Vec<u32> = vec![
        0, 4, 6,
        0, 6, 4,

        0, 6, 2,
        0, 2, 6,

        1, 7, 5,
        1, 5, 7,

        1, 7, 3,
        1, 3, 7
    ].iter().map(|x| *x + start_index).collect();

    return (vertices, indices);
}