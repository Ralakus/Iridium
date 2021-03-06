use super::core::na as na;

pub struct Mesh {
    vertices: Vec<na::Vector3<f32>>,
    normals: Vec<na::Vector3<f32>>,
    texture_coords: Vec<na::Vector2<f32>>,
    interleaved_data: Vec<f32>,
}

impl Mesh {
    pub fn new(
        vertices: Vec<na::Vector3<f32>>,
        normals: Vec<na::Vector3<f32>>,
        texture_coords: Vec<na::Vector2<f32>>
    ) -> Self {
        Mesh {
            vertices,
            normals,
            texture_coords,
            interleaved_data: Vec::new(),
        }
    }

    pub fn new_default() -> Self {
        Mesh {
            vertices        : Vec::new(),
            normals         : Vec::new(),
            texture_coords  : Vec::new(),
            interleaved_data: Vec::new(),
        }
    }
}

impl Mesh {
    pub fn interleave(&mut self) {
        self.interleaved_data.clear();
        let vertices_length = self.vertices.len();
        self.normals.resize(vertices_length, na::Vector3::new(0_f32, 0_f32, 0_f32));
        self.texture_coords.resize(vertices_length, na::Vector2::new(0_f32, 0_f32));

        for i in 0..vertices_length {

            self.interleaved_data.push(self.vertices[i].x);
            self.interleaved_data.push(self.vertices[i].y);
            self.interleaved_data.push(self.vertices[i].z);

            self.interleaved_data.push(self.normals[i].x);
            self.interleaved_data.push(self.normals[i].y);
            self.interleaved_data.push(self.normals[i].z);

            self.interleaved_data.push(self.texture_coords[i].x);
            self.interleaved_data.push(self.texture_coords[i].y);

        }
    }
}