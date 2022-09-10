use glium::implement_vertex;

/**
 Rectangle using 2 triangles
 Screen space:
  -1,1   1,1

  -1,-1  1,-1

 Texture space:
  0,1     1,1

  0,0     1,0
 */
pub const RECT: [Vertex; 6] = [
    Vertex {
        // Top left
        position: [-1.0, 1.0],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        // Top right
        position: [1.0, 1.0],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        // Bottom left
        position: [-1.0, -1.0],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        // Bottom right
        position: [1.,-1.],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        // Top right
        position: [1.0, 1.0],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        // Bottom left
        position: [-1.0, -1.0],
        tex_coords: [0.0, 0.0],
    },
];

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);
