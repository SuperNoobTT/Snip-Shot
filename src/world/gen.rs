use rand::prelude::*;
use bevy::{prelude::*, render::{render_asset::RenderAssetUsages, mesh::{PrimitiveTopology, Indices}}};
use noise::{Perlin, Fbm, NoiseFn};
use bevy_rapier3d::prelude::*;
use crate::utils::{ENVIRONMENT_COLLISION, PLATFORM_COLLISION};

const PLATFORM_THRESHOLD: f32 = 0.35;
const PLATFORM: Cuboid = Cuboid{
    half_size: Vec3 { x: 1.0, y: 2.0, z: 1.0 }
};
const TERRAIN_SIZE: f32 = 400.0;

#[derive(Bundle, Clone, Default)]
struct PlatformBundle {
    collider: Collider, 
    coll_group: CollisionGroups, 
    name: Name, 
    bundle: PbrBundle,
}

pub fn spawn_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let noise_func = Fbm::<Perlin>::new(thread_rng().next_u32());
    let y_scaling: f64 = 1.0;
    const NUM_VERTICES: usize = (TERRAIN_SIZE * 2.0) as usize;

    let mut vertices = Vec::with_capacity(NUM_VERTICES * NUM_VERTICES);
    let mut indices = Vec::with_capacity(NUM_VERTICES * NUM_VERTICES * 6);
    let mut uvs = Vec::with_capacity(NUM_VERTICES * NUM_VERTICES);
    let mut platforms = Vec::with_capacity(NUM_VERTICES); //Probably way too high but it's ok :B

    //Generate indices, vertices and uvs based on noise
    for z in 0..NUM_VERTICES {
        for x in 0..NUM_VERTICES {
            let x_pos = x as f32 - TERRAIN_SIZE;
            let z_pos = z as f32 - TERRAIN_SIZE;
            let y_pos = (noise_func.get([x_pos as f64, z_pos as f64]) * y_scaling) as f32;

            vertices.push([x_pos, y_pos, z_pos]);
            uvs.push([x as f32 / TERRAIN_SIZE, z as f32 / TERRAIN_SIZE]);

            if x < NUM_VERTICES - 1 && z < NUM_VERTICES - 1 {
                let idx = x + z * NUM_VERTICES;
                indices.push(idx as u32);
                indices.push((idx + NUM_VERTICES) as u32);
                indices.push((idx + 1) as u32);
                indices.push((idx + 1) as u32);
                indices.push((idx + NUM_VERTICES) as u32);
                indices.push((idx + NUM_VERTICES + 1) as u32);
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
    let platform_meshes = create_platform_meshes(
        &platforms, &mut meshes, &mut materials
    );

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
    commands.spawn_batch(platform_meshes);

    //FIXME: Move this to the `create_platform_meshes()` function
    //Spawn the platforms
} 

#[deprecated(note = "We should have seperate meshes for each platform")]
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

fn create_platform_meshes (
    positions: &[[f32; 3]],
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>
) -> Vec<PlatformBundle> {
    let platform: Mesh = Mesh::from(PLATFORM.clone());
    let platform_vertices: Vec<[f32; 3]> = platform
        .attribute(Mesh::ATTRIBUTE_POSITION)
        .unwrap()
        .as_float3()
        .unwrap()
        .to_vec();
    // let platform_indices = platform.indices().unwrap();
    // let platform_normals: Vec<[f32; 3]> = platform
    //     .attribute(Mesh::ATTRIBUTE_NORMAL)
    //     .unwrap()
    //     .as_float3()
    //     .unwrap()
    //     .to_vec();

    // Create a vector to store individual platform meshes
    positions
        .iter()
        .map(|&[x, y, z]| {
            let vertices: Vec<[f32; 3]> = platform_vertices
                .iter()
                .map(|&vertex| [vertex[0] + x, vertex[1] + y, vertex[2] + z])
                .collect();

            //TODO: Check which one works!
            // let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
            // mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
            // mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, platform_normals.clone());
            // mesh.insert_indices(platform_indices.clone());
            let mut mesh = Mesh::from(PLATFORM.clone());
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);

            let mesh_handle = meshes.add(mesh.clone());

            PlatformBundle{
                collider: Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh)
                    .expect("Platform collider uncomputable!"),
                coll_group: PLATFORM_COLLISION,
                name: Name::new("platform"),
                bundle: PbrBundle {
                    mesh: mesh_handle,
                    material: materials.add(Color::srgb(0.0, 0.2, 0.8)),
                    transform: Transform::from_xyz(x, y, z),
                    ..Default::default()
                },
            }
        })
        .collect()
}


