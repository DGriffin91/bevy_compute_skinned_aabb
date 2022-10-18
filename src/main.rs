//! Skinned mesh example with mesh and joints data defined in code.
//! Example taken from <https://github.com/KhronosGroup/glTF-Tutorials/blob/master/gltfTutorial/gltfTutorial_019_SimpleSkin.md>

use std::f32::consts::PI;

use bevy::{
    pbr::{
        wireframe::{Wireframe, WireframePlugin},
        SkinnedMeshJoints,
    },
    prelude::*,
    render::{
        mesh::{
            skinning::{SkinnedMesh, SkinnedMeshInverseBindposes},
            Indices, PrimitiveTopology, VertexAttributeValues,
        },
        primitives::Aabb,
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WireframePlugin)
        .add_startup_system(setup)
        .add_system(joint_animation)
        .add_system(skinned_vertex_locations)
        .run();
}

/// Used to mark a joint to be animated in the [`joint_animation`] system.
#[derive(Component)]
struct AnimatedJoint;

/// Construct a mesh and a skeleton with 2 joints for that mesh,
///   and mark the second joint to be animated.
/// It is similar to the scene defined in `models/SimpleSkin/SimpleSkin.gltf`
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut skinned_mesh_inverse_bindposes_assets: ResMut<Assets<SkinnedMeshInverseBindposes>>,
) {
    // Create a camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Create inverse bindpose matrices for a skeleton consists of 2 joints
    let inverse_bindposes =
        skinned_mesh_inverse_bindposes_assets.add(SkinnedMeshInverseBindposes::from(vec![
            Mat4::from_translation(Vec3::new(-0.5, -1.0, 0.0)),
            Mat4::from_translation(Vec3::new(-0.5, -1.0, 0.0)),
        ]));

    // Create a mesh
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    // Set mesh vertex positions
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 0.5, 0.0],
            [1.0, 0.5, 0.0],
            [0.0, 1.0, 0.0],
            [1.0, 1.0, 0.0],
            [0.0, 1.5, 0.0],
            [1.0, 1.5, 0.0],
            [0.0, 2.0, 0.0],
            [1.0, 2.0, 0.0],
        ],
    );
    // Set mesh vertex normals
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 10]);
    // Set mesh vertex UVs. Although the mesh doesn't have any texture applied,
    //  UVs are still required by the render pipeline. So these UVs are zeroed out.
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0]; 10]);
    // Set mesh vertex joint indices for mesh skinning.
    // Each vertex gets 4 indices used to address the `JointTransforms` array in the vertex shader
    //  as well as `SkinnedMeshJoint` array in the `SkinnedMesh` component.
    // This means that a maximum of 4 joints can affect a single vertex.
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_JOINT_INDEX,
        vec![
            [0u16, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 1, 0, 0],
            [0, 1, 0, 0],
            [0, 1, 0, 0],
            [0, 1, 0, 0],
            [0, 1, 0, 0],
            [0, 1, 0, 0],
            [0, 1, 0, 0],
            [0, 1, 0, 0],
        ],
    );
    // Set mesh vertex joint weights for mesh skinning.
    // Each vertex gets 4 joint weights corresponding to the 4 joint indices assigned to it.
    // The sum of these weights should equal to 1.
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_JOINT_WEIGHT,
        vec![
            [1.00, 0.00, 0.0, 0.0],
            [1.00, 0.00, 0.0, 0.0],
            [0.75, 0.25, 0.0, 0.0],
            [0.75, 0.25, 0.0, 0.0],
            [0.50, 0.50, 0.0, 0.0],
            [0.50, 0.50, 0.0, 0.0],
            [0.25, 0.75, 0.0, 0.0],
            [0.25, 0.75, 0.0, 0.0],
            [0.00, 1.00, 0.0, 0.0],
            [0.00, 1.00, 0.0, 0.0],
        ],
    );
    // Tell bevy to construct triangles from a list of vertex indices,
    //  where each 3 vertex indices form an triangle.
    mesh.set_indices(Some(Indices::U16(vec![
        0, 1, 3, 0, 3, 2, 2, 3, 5, 2, 5, 4, 4, 5, 7, 4, 7, 6, 6, 7, 9, 6, 9, 8,
    ])));

    let mesh = meshes.add(mesh);

    // Create joint entities
    let joint_0 = commands
        .spawn_bundle((Transform::default(), GlobalTransform::identity()))
        .id();
    let joint_1 = commands
        .spawn_bundle((
            AnimatedJoint,
            Transform::identity(),
            GlobalTransform::identity(),
        ))
        .id();

    // Set joint_1 as a child of joint_0.
    commands.entity(joint_0).push_children(&[joint_1]);

    // Each joint in this vector corresponds to each inverse bindpose matrix in `SkinnedMeshInverseBindposes`.
    let joint_entities = vec![joint_0, joint_1];

    // Create skinned mesh renderer. Note that its transform doesn't affect the position of the mesh.
    commands
        .spawn_bundle(PbrBundle {
            mesh: mesh.clone(),
            material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
            ..default()
        })
        .insert(SkinnedMesh {
            inverse_bindposes: inverse_bindposes.clone(),
            joints: joint_entities,
        });

    // debug cubes for each vertex
    for _ in 0..10 {
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
                ..default()
            })
            .insert(DebugVertex);
    }

    // AABB debug cube
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgba(0.0, 0.0, 0.0, 0.0).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(
            // This enables wireframe drawing on this entity
            Wireframe,
        )
        .insert(AABBDebugCube);
}

#[derive(Component)]
struct DebugVertex;

#[derive(Component)]
struct AABBDebugCube;

/// Animate the joint marked with [`AnimatedJoint`] component.
fn joint_animation(time: Res<Time>, mut query: Query<&mut Transform, With<AnimatedJoint>>) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_axis_angle(
            Vec3::Z,
            0.5 * PI * time.time_since_startup().as_secs_f32().sin(),
        );
    }
}

fn skinned_vertex_locations(
    query: Query<(&Handle<Mesh>, &SkinnedMesh)>,
    meshes: Res<Assets<Mesh>>,
    skinned_mesh_inverse_bindposes_assets: Res<Assets<SkinnedMeshInverseBindposes>>,
    joint_query: Query<&GlobalTransform>,
    mut debug_vertex_cubes: Query<&mut Transform, (With<DebugVertex>, Without<AABBDebugCube>)>,
    mut aabb_debug_cube: Query<&mut Transform, (With<AABBDebugCube>, Without<DebugVertex>)>,
) {
    for (mesh_h, skinned_mesh) in query.iter() {
        if let Some(mesh) = meshes.get(mesh_h) {
            // Get required vertex attributes
            let mesh_positions = if let Some(VertexAttributeValues::Float32x3(positions)) =
                mesh.attribute(Mesh::ATTRIBUTE_POSITION)
            {
                positions
            } else {
                continue;
            };
            let mesh_indices = if let Some(VertexAttributeValues::Uint16x4(indices)) =
                mesh.attribute(Mesh::ATTRIBUTE_JOINT_INDEX)
            {
                indices
            } else {
                continue;
            };
            let mesh_weights = if let Some(VertexAttributeValues::Float32x4(weights)) =
                mesh.attribute(Mesh::ATTRIBUTE_JOINT_WEIGHT)
            {
                weights
            } else {
                continue;
            };

            // get skinned mesh joint models
            let mut joints = Vec::new();
            if let Some(_) = SkinnedMeshJoints::build(
                skinned_mesh,
                &skinned_mesh_inverse_bindposes_assets,
                &joint_query,
                &mut joints,
            ) {
                // Use skin model to get world space vertex positions
                let ws_positions: Vec<Vec3> = mesh_positions
                    .iter()
                    .zip(mesh_indices)
                    .zip(mesh_weights)
                    .map(|((pos, indices), weights)| {
                        let model = skin_model(&joints, indices, Vec4::from(*weights));
                        model.transform_point3(Vec3::from(*pos))
                    })
                    .collect();

                // update debug cube positions to match world space vertices
                for (mut trans, ws_pos) in debug_vertex_cubes.iter_mut().zip(&ws_positions) {
                    trans.translation = *ws_pos;
                }

                //compute world space aabb
                let ws_aabb = compute_aabb(&ws_positions).unwrap();

                //update aabb debug cube
                if let Some(mut trans) = aabb_debug_cube.iter_mut().next() {
                    trans.translation = ws_aabb.center.into();
                    trans.scale = (ws_aabb.half_extents * 2.0).into();
                }
            }
        }
    }
}

fn skin_model(joint_matrices: &Vec<Mat4>, indexes: &[u16; 4], weights: Vec4) -> Mat4 {
    weights.x * joint_matrices[indexes[0] as usize]
        + weights.y * joint_matrices[indexes[1] as usize]
        + weights.z * joint_matrices[indexes[2] as usize]
        + weights.w * joint_matrices[indexes[3] as usize]
}

const VEC3_MIN: Vec3 = Vec3::splat(std::f32::MIN);
const VEC3_MAX: Vec3 = Vec3::splat(std::f32::MAX);

/// Compute the Axis-Aligned Bounding Box of the mesh vertices in model space
/// from https://github.com/bevyengine/bevy/blob/main/crates/bevy_render/src/mesh/mesh/mod.rs#L375
pub fn compute_aabb(values: &[Vec3]) -> Option<Aabb> {
    let mut minimum = VEC3_MAX;
    let mut maximum = VEC3_MIN;
    for p in values {
        minimum = minimum.min(*p);
        maximum = maximum.max(*p);
    }
    if minimum.x != std::f32::MAX
        && minimum.y != std::f32::MAX
        && minimum.z != std::f32::MAX
        && maximum.x != std::f32::MIN
        && maximum.y != std::f32::MIN
        && maximum.z != std::f32::MIN
    {
        return Some(Aabb::from_min_max(minimum, maximum));
    }

    None
}
