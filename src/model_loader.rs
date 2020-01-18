use crate::material::*;
use crate::triangle::*;
use cgmath::Vector3;
use std::path::Path;
use tobj;

pub fn load(file: &Path) -> Vec<Box<Triangle>> {
    let obj = tobj::load_obj(&file);
    let mut output: Vec<Box<Triangle>> = vec![];

    if obj.is_err() {
        return vec![];
    }

    let (models, _materials) = obj.unwrap();

    for (_i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;

        if mesh.positions.len() % 3 != 0 {
            continue;
        }

        if mesh.normals.len() % 3 != 0 {
            continue;
        }

        if mesh.indices.len() % 3 != 0 {
            continue;
        }

        let faces: Vec<(Vector3<f32>, Vector3<f32>)> = mesh
            .positions
            .chunks(3)
            .zip(mesh.normals.chunks(3))
            .map(|(i, n)| {
                (
                    Vector3::new(i[0], i[1], i[2]),
                    Vector3::new(n[0], n[1], n[2]),
                )
            })
            .collect::<Vec<(Vector3<f32>, Vector3<f32>)>>();

        let mut triangles = mesh
            .indices
            .chunks(3)
            .map(|i| {
                Box::new(Triangle {
                    vertices: [
                        faces[i[0] as usize].0,
                        faces[i[1] as usize].0,
                        faces[i[2] as usize].0,
                    ],
                    normal: faces[i[0] as usize].1,
                    material: Box::new(Lambertian {
                        albedo: Vector3::new(0.25, 0.35, 0.6),
                    }),
                })
            })
            .collect();

        output.append(&mut triangles);
    }

    output
}
