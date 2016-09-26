use pipeline::Vertex;

pub struct Level {
    level_info: Vec<i32>,
}

impl Level {
    pub fn new() -> Level {
        let level_info = Vec::<i32>::new();

        Level {
            level_info: level_info,
        }
    }

    pub fn generate_mesh(&self) -> (Vec<Vertex>, Vec<u32>) {
        let mut vertex_data = Vec::<Vertex>::new();
        vertex_data.push(Vertex { pos: [ -0.5,  0.0 ], color: [1.0, 1.0, 0.0] });
        vertex_data.push(Vertex { pos: [  0.5,  0.0 ], color: [1.0, 0.0, 0.0] });
        let mut index_data = Vec::<u32>::new();
        index_data.push(0);
        index_data.push(1);
        (vertex_data, index_data)
    }
}
