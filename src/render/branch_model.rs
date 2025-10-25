use crate::render::Vertex;

pub fn generate_branch_model(color: [f32; 4]) -> (Vec<Vertex>, Vec<u32>) {
    let vertices: Vec<Vertex> = vec![
        Vertex {
            position: [0.0, -0.1, 0.0],
            color: color.clone()
        },
        Vertex {
            position: [0.1, 0.0, 0.1],
            color: color.clone()
        },
        Vertex {
            position: [-0.1, 0.0, 0.1],
            color: color.clone()
        },
        Vertex {
            position: [-0.1, 0.0, -0.1],
            color: color.clone()
        },
        Vertex {
            position: [0.1, 0.0, -0.1],
            color: color.clone()
        },

        
        Vertex {
            position: [0.1, 1.0, 0.1],
            color: color.clone()
        },
        Vertex {
            position: [-0.1, 1.0, 0.1],
            color: color.clone()
        },
        Vertex {
            position: [-0.1, 1.0, -0.1],
            color: color.clone()
        },
        Vertex {
            position: [0.1, 1.0, -0.1],
            color: color.clone()
        },
        Vertex {
            position: [0.0, 1.1, 0.0],
            color: color.clone()
        }
    ];

    let indices: Vec<u32> = vec![
        0, 2, 1,
        0, 3, 2,
        0, 4, 3,
        0, 1, 4,
        1, 2, 5,
        2, 6, 5,
        1, 5, 8,
        4, 1, 8,
        2, 7, 6,
        3, 7, 2,
        4, 8, 3,
        3, 8, 7,
        8, 9, 7,
        7, 9, 6,
        6, 9, 5,
        5, 9, 8        
    ];

    return (vertices, indices);
}