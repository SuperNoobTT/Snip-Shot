use rand::prelude::*;
use bevy::{prelude::*, render::{render_asset::RenderAssetUsages, mesh::{PrimitiveTopology, Indices}}};
use noise::{Perlin, Fbm, NoiseFn};
use bevy_rapier3d::prelude::*;
use crate::utils::ENVIRONMENT_COLLISION;

const PLATFORM_THRESHOLD: f32 = 0.35;
const PLATFORM: Cuboid = Cuboid{
    half_size: Vec3 { x: 1.0, y: 2.0, z: 1.0 }
};

pub fn spawn_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let noise_func = Fbm::<Perlin>::new(thread_rng().next_u32());
    let y_scaling: f64 = 1.0;
    const TERRAIN_SIZE: f32 = 400.0;
    let num_vertices = (TERRAIN_SIZE * 2.0) as usize;

    let mut vertices = Vec::with_capacity(num_vertices * num_vertices);
    let mut indices = Vec::with_capacity(num_vertices * num_vertices * 6);
    let mut uvs = Vec::with_capacity(num_vertices * num_vertices);
    let mut platforms = Vec::with_capacity(num_vertices); //Probably way too hight but it's ok :B

    //Generate indices, vertices and uvs based on noise
    for z in 0..num_vertices {
        for x in 0..num_vertices {
            let x_pos = x as f32 - TERRAIN_SIZE;
            let z_pos = z as f32 - TERRAIN_SIZE;
            let y_pos = (noise_func.get([x_pos as f64, z_pos as f64]) * y_scaling) as f32;

            vertices.push([x_pos, y_pos, z_pos]);
            uvs.push([x as f32 / TERRAIN_SIZE, z as f32 / TERRAIN_SIZE]);

            if x < num_vertices - 1 && z < num_vertices - 1 {
                let idx = x + z * num_vertices;
                indices.push(idx as u32);
                indices.push((idx + num_vertices) as u32);
                indices.push((idx + 1) as u32);
                indices.push((idx + 1) as u32);
                indices.push((idx + num_vertices) as u32);
                indices.push((idx + num_vertices + 1) as u32);
            }

            if y_pos > PLATFORM_THRESHOLD {
                platforms.push([x_pos, y_pos + 2.0, z_pos]);
            }
        }
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    mesh.compute_normals();

    let terrain_mesh = Mesh::with_generated_tangents(mesh).expect("Failed to compute tangents for terrain mesh!");

    //Create a mesh to handle the platforms
    let platform_mesh = create_platform_mesh(&platforms);

    //Spawn the terrain
    commands.spawn((
        //Add collision with the environment collision & solver groups
        Collider::from_bevy_mesh(&terrain_mesh, &ComputedColliderShape::TriMesh).expect("Terrain collider uncomputable!"),
        ENVIRONMENT_COLLISION,
        PbrBundle {
            mesh: meshes.add(terrain_mesh),
            material: materials.add(Color::WHITE),
            ..Default::default()
        },
        Name::new("terrain")
    ));

    //Spawn the platforms
    commands.spawn((
        //Add collision with the environment collision & solver groups
        Collider::from_bevy_mesh(&platform_mesh, &ComputedColliderShape::TriMesh).expect("Platform collider uncomputable!"),
        ENVIRONMENT_COLLISION,
        Name::new("platforms"),
        PbrBundle {
            mesh: meshes.add(platform_mesh),
            material: materials.add(Color::srgb(0.0, 0.2, 0.8)),
            ..Default::default()
        }
    ));
} 

fn create_platform_mesh(positions: &[[f32; 3]]) -> Mesh {
    let platform: Mesh = Mesh::from(PLATFORM.clone());
    let platform_vertices: Vec<[f32; 3]> = platform.attribute(Mesh::ATTRIBUTE_POSITION).unwrap().as_float3().unwrap().to_vec();
    let platform_indices = platform.indices().unwrap();
    let platform_normals: Vec<[f32; 3]> = platform.attribute(Mesh::ATTRIBUTE_NORMAL).unwrap().as_float3().unwrap().to_vec();

    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut normals = Vec::new();

    for (i, &[x, y, z]) in positions.iter().enumerate() {
        let offset = (i * platform_vertices.len()) as u32;
        
        // Add vertices
        for &vertex in &platform_vertices {
            vertices.push([
                vertex[0] + x,
                vertex[1] + y,
                vertex[2] + z,
            ]);
        }
        
        // Add indices
        for index in platform_indices.iter() {
            indices.push(index as u32 + offset);
        }
        
        // Add normals
        normals.extend_from_slice(&platform_normals);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_indices(Indices::U32(indices));
    
    mesh
}