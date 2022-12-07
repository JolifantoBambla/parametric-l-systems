use glam::{Vec2, Vec3};
use crate::framework::mesh::vertex::VertexType;
use crate::framework::util::math::PI_2;

pub struct Mesh<V: VertexType> {
    name: String,
    faces: Vec<[u32; 3]>,
    vertices: Vec<V>,
}

impl<V: VertexType> Mesh<V> {
    pub fn new_default_cylinder(centered: bool) -> Self {
        Mesh::new_cylinder(32, 1, centered)
    }

    // https://vorg.github.io/pex/docs/pex-gen/Cylinder.html
    pub fn new_cylinder(num_sides: usize, num_segments: usize, centered: bool) -> Self {
        let radius: f32 = 0.5;
        let height: f32 = 1.0;

        // => radius for top & bottom cap
        let r_top = radius;
        let r_bottom = radius;

        // => generate top & bottom cap
        let bottom_cap = true;
        let top_cap = true;

        let mut faces = Vec::new();
        let mut vertices = Vec::new();

        let mut index = 0;

        let offset_y = if centered {
            0.0
        } else {
            - height / 2.
        };

        for j in 0..num_segments + 1 {
            for i in 0..num_sides + 1 {
                let segment_ratio = j as f32 / num_segments as f32;
                let side_ratio = i as f32 / num_sides as f32;
                let r = r_bottom + (r_top - r_bottom) * segment_ratio;
                let y = offset_y + height * segment_ratio;
                let x = r * f32::cos(side_ratio * PI_2);
                let z = r * f32::sin(side_ratio * PI_2);
                vertices.push(V::from_pos_normal_tex_coords(
                    Vec3::new(x, y, z),
                    Vec3::new(x, 0.0, z),
                    Vec2::new(side_ratio, segment_ratio),
                ));
                if i < num_sides && j < num_segments {
                    let i0 = index + 1;
                    let i1 = index;
                    let i2 = index + num_sides as u32 + 1;
                    let i3 = index + num_sides as u32 + 2;
                    faces.push([i0, i1, i2]);
                    faces.push([i0, i2, i3]);
                }
                index += 1;
            }
        }

        if bottom_cap {
            vertices.push(V::from_pos_normal_tex_coords(
                Vec3::new(0.0, offset_y, 0.0),
                Vec3::new(0.0, -1.0, 0.0),
                Vec2::new(0.0, 0.0),
            ));
            let center_index = index;
            index += 1;
            for i in 0..num_sides + 1 {
                let y = offset_y;
                let x = r_bottom * f32::cos((i as f32 / num_sides as f32) * PI_2);
                let z = r_bottom * f32::sin((i as f32 / num_sides as f32) * PI_2);
                vertices.push(V::from_pos_normal_tex_coords(
                    Vec3::new(x, y, z),
                    Vec3::new(0.0, -1.0, 0.0),
                    Vec2::new(0.0, 0.0),
                ));
                if i < num_sides {
                    faces.push([index, index + 1, center_index]);
                }
                index += 1;
            }
        }

        if top_cap {
            vertices.push(V::from_pos_normal_tex_coords(
                Vec3::new(0.0, offset_y + height, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Vec2::new(0.0, 0.0),
            ));
            let center_index = index;
            index += 1;
            for i in 0..num_sides + 1 {
                let y = offset_y + height;
                let x = r_top * f32::cos((i as f32 / num_sides as f32) * PI_2);
                let z = r_top * f32::sin((i as f32 / num_sides as f32) * PI_2);
                vertices.push(V::from_pos_normal_tex_coords(
                    Vec3::new(x, y, z),
                    Vec3::new(0.0, 1.0, 0.0),
                    Vec2::new(1.0, 1.0),
                ));
                if i < num_sides {
                    faces.push([index + 1, index, center_index]);
                }
                index += 1;
            }
        }

        Self {
            name: String::from(format!("Cylinder (r={}, h={}, centered={})", radius, height, centered)),
            faces,
            vertices,
        }
    }

    pub fn new_cube() -> Self {
        let vertices = vec![
            // top (0, 0, 1)
            V::from_pos_normal_tex_coords(Vec3::new(-1.0, -1.0, 1.0), Vec3::Z, Vec2::ZERO),
            V::from_pos_normal_tex_coords(Vec3::new( 1.0, -1.0, 1.0), Vec3::Z, Vec2::ZERO),
            V::from_pos_normal_tex_coords(Vec3::new( 1.0, 1.0, 1.0), Vec3::Z, Vec2::ZERO),
            V::from_pos_normal_tex_coords(Vec3::new(-1.0, 1.0, 1.0), Vec3::Z, Vec2::ZERO),
            // bottom (0, 0, -1)
            V::from_pos_normal_tex_coords(Vec3::new(-1.0, 1.0, -1.0), Vec3::Z * -1., Vec2::ZERO),
            V::from_pos_normal_tex_coords(Vec3::new( 1.0, 1.0, -1.0), Vec3::Z * -1., Vec2::ZERO),
            V::from_pos_normal_tex_coords(Vec3::new( 1.0, -1.0, -1.0), Vec3::Z * -1., Vec2::ZERO),
            V::from_pos_normal_tex_coords(Vec3::new(-1.0, -1.0, -1.0), Vec3::Z * -1., Vec2::ZERO),
            // right (1, 0, 0)
            V::from_pos_normal_tex_coords(Vec3::new(1.0, -1.0, -1.0), Vec3::X, Vec2::ZERO),
            V::from_pos_normal_tex_coords(Vec3::new(1.0, 1.0, -1.0), Vec3::X, Vec2::ZERO),
            V::from_pos_normal_tex_coords(Vec3::new(1.0, 1.0, 1.0), Vec3::X, Vec2::ZERO),
            V::from_pos_normal_tex_coords(Vec3::new(1.0, -1.0, 1.0), Vec3::X, Vec2::ZERO),
            // left (-1, 0, 0)
            V::from_pos_normal_tex_coords(Vec3::new(-1.0, -1.0, 1.0), Vec3::X * -1., Vec2::ZERO),
            V::from_pos_normal_tex_coords(Vec3::new(-1.0, 1.0, 1.0), Vec3::X * -1., Vec2::ZERO),
            V::from_pos_normal_tex_coords(Vec3::new(-1.0, 1.0, -1.0), Vec3::X * -1., Vec2::ZERO),
            V::from_pos_normal_tex_coords(Vec3::new(-1.0, -1.0, -1.0), Vec3::X * -1., Vec2::ZERO),
            // front (0, 1, 0)
            V::from_pos_normal_tex_coords(Vec3::new( 1.0, 1.0, -1.0), Vec3::Y, Vec2::ZERO),
            V::from_pos_normal_tex_coords(Vec3::new(-1.0, 1.0, -1.0), Vec3::Y, Vec2::ZERO),
            V::from_pos_normal_tex_coords(Vec3::new(-1.0, 1.0, 1.0), Vec3::Y, Vec2::ZERO),
            V::from_pos_normal_tex_coords(Vec3::new( 1.0, 1.0, 1.0), Vec3::Y, Vec2::ZERO),
            // back (0, -1, 0)
            V::from_pos_normal_tex_coords(Vec3::new( 1.0, -1.0, 1.0), Vec3::Y * -1., Vec2::ZERO),
            V::from_pos_normal_tex_coords(Vec3::new(-1.0, -1.0, 1.0), Vec3::Y * -1., Vec2::ZERO),
            V::from_pos_normal_tex_coords(Vec3::new(-1.0, -1.0, -1.0), Vec3::Y * -1., Vec2::ZERO),
            V::from_pos_normal_tex_coords(Vec3::new( 1.0, -1.0, -1.0), Vec3::Y * -1., Vec2::ZERO),
        ];
        let faces: Vec<[u32; 3]> = vec![
            [0, 1, 2], [2, 3, 0], // top
            [4, 5, 6], [6, 7, 4], // bottom
            [8, 9, 10], [10, 11, 8], // right
            [12, 13, 14], [14, 15, 12], // left
            [16, 17, 18], [18, 19, 16], // front
            [20, 21, 22], [22, 23, 20], // back
        ];
        Self {
            name: "Cube".to_string(),
            faces,
            vertices,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn faces(&self) -> &Vec<[u32; 3]> {
        &self.faces
    }
    pub fn vertices(&self) -> &Vec<V> {
        &self.vertices
    }
}
