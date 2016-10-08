use pipeline::Vertex;

pub struct Level {
    _level_info: Vec<i32>,
}

impl Level {
    pub fn new() -> Level {
        let level_info = Vec::<i32>::new();

        Level { _level_info: level_info }
    }

    pub fn generate_mesh(&self) -> (Vec<Vertex>, Vec<u32>) {
        let vertex_data = vec![Vertex {
                                   pos: [-0.5, 0.0],
                                   color: [1.0, 1.0, 0.0],
                               },
                               Vertex {
                                   pos: [0.5, 0.8],
                                   color: [1.0, 0.0, 0.0],
                               },
                               Vertex {
                                   pos: [-0.3, -0.1],
                                   color: [1.0, 1.0, 0.0],
                               },
                               Vertex {
                                   pos: [0.4, -0.5],
                                   color: [0.0, 1.0, 1.0],
                               },
                               Vertex {
                                   pos: [0.5, 0.4],
                                   color: [0.0, 1.0, 0.0],
                               },
                               Vertex {
                                   pos: [0.9, -0.2],
                                   color: [1.0, 0.0, 1.0],
                               }];
        let index_data = vec![0, 1, 2, 3, 4, 5];
        (vertex_data, index_data)
    }
}
