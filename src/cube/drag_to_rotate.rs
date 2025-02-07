use std::f32::consts::TAU;

use bevy::{picking::pointer::PointerInteraction, prelude::*};

use crate::{cube::slice::column_index_to_slice, schedules::CubeScheduleSet};

use super::{Cube, SequenceResource};

pub struct DragToRotatePlugin;

impl Plugin for DragToRotatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_picking_hover.in_set(CubeScheduleSet::HandleUserInput),
        );
    }
}

#[derive(Component)]
pub enum Face {
    Top,
    Front,
    Right,
}

#[derive(Component)]
pub struct FaceTop;
#[derive(Component)]
pub struct FaceFront;
#[derive(Component)]
pub struct FaceRight;

#[derive(Component)]
pub struct OriginalMeshSize(f32);

pub fn spawn(
    parent: &mut ChildBuilder<'_>,
    _materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    cube_size: usize,
    block_size: f32,
    piece_spread: f32,
) {
    let face_size = (cube_size as f32 * block_size) + ((cube_size - 1) as f32 * piece_spread);
    let distance_from_center = face_size / 2.0;
    let collider_mesh = meshes.add(Rectangle {
        half_size: Vec2::ONE * face_size / 2.0,
    });

    // let debug_material: MeshMaterial3d<StandardMaterial> =
    //     MeshMaterial3d(materials.add(Color::srgb(0.5019608, 0.0, 0.5019608)));
    let debug_material = ();

    // top
    let mut transform = Transform::from_translation(Vec3 {
        x: 0.0,
        y: distance_from_center,
        z: 0.0,
    });
    transform.rotate_x(-TAU / 4.0);

    parent.spawn((
        Mesh3d(collider_mesh.clone()),
        debug_material.clone(),
        transform,
        FaceTop,
        OriginalMeshSize(face_size),
        Face::Top,
    ));

    // front
    let transform = Transform::from_translation(Vec3 {
        x: 0.0,
        y: 0.0,
        z: distance_from_center,
    });

    parent.spawn((
        Mesh3d(collider_mesh.clone()),
        debug_material.clone(),
        transform,
        FaceFront,
        OriginalMeshSize(face_size),
        Face::Front,
    ));

    // right
    let mut transform = Transform::from_translation(Vec3 {
        x: distance_from_center,
        y: 0.0,
        z: 0.0,
    });
    transform.rotate_local_y(TAU / 4.0);

    parent.spawn((
        Mesh3d(collider_mesh.clone()),
        debug_material.clone(),
        transform,
        FaceRight,
        OriginalMeshSize(face_size),
        Face::Right,
    ));
}

fn handle_picking_hover(
    pointers: Query<&PointerInteraction>,
    top_face_query: Query<(Entity, &GlobalTransform, &OriginalMeshSize, &Face), With<FaceTop>>,
    front_face_query: Query<(Entity, &GlobalTransform, &OriginalMeshSize, &Face), With<FaceFront>>,
    right_face_query: Query<(Entity, &GlobalTransform, &OriginalMeshSize, &Face), With<FaceRight>>,
    cube_query: Query<&Cube>,
    current_sequence: Res<SequenceResource>,
) {
    let Ok(cube) = cube_query.get_single() else {
        warn!("couldn't find cube");
        return;
    };

    if cube.is_animating_rotation || !current_sequence.is_done() {
        return;
    }

    let Ok(top_face) = top_face_query.get_single() else {
        warn!("couldn't find top face");
        return;
    };
    let Ok(front_face) = front_face_query.get_single() else {
        warn!("couldn't find front face");
        return;
    };
    let Ok(right_face) = right_face_query.get_single() else {
        warn!("couldn't find right face");
        return;
    };

    for (entity, hit_data) in pointers
        .iter()
        .filter_map(|interaction| interaction.get_nearest_hit())
    {
        let (_, global_transform, original_mesh_size, face) = if *entity == top_face.0 {
            top_face
        } else if *entity == front_face.0 {
            front_face
        } else if *entity == right_face.0 {
            right_face
        } else {
            // not a hit we are interested in
            return;
        };

        let Some(hit_position) = hit_data.position else {
            warn!("hit position is None, expected Some");
            return;
        };

        let cube_size = cube.size().0;

        // global_transform.scale() its x, y and z will always be the same so it doesn't matter which one we pick.
        let scale = global_transform.scale().x;

        let face_hit_position_world_vec3 = hit_position - global_transform.translation();
        let face_hit_position_world_vec2 = match face {
            Face::Top => face_hit_position_world_vec3.xz() * Vec2::new(1.0, -1.0),
            Face::Front => face_hit_position_world_vec3.xy(),
            Face::Right => face_hit_position_world_vec3.zy() * Vec2::new(-1.0, 1.0),
        } / scale;
        let range = original_mesh_size.0;
        let face_hit_position_normalised = face_hit_position_world_vec2 + range / 2.0;

        // ranging from `0..cube_size`. Not what we can use to rotate cube slice slice yet
        let (relative_slice_index_x, relative_slice_index_y) = (
            face_hit_position_normalised.x.floor() as i32,
            face_hit_position_normalised.y.floor() as i32,
        );

        // picking the very edge of the face will result in the index being 1 too high.
        let column_index_x = relative_slice_index_x.clamp(0, cube_size - 1);
        let column_index_y = relative_slice_index_y.clamp(0, cube_size - 1);

        let slice_x = column_index_to_slice(column_index_x, cube_size as usize);
        let slice_y = column_index_to_slice(column_index_y, cube_size as usize);

        info!("x: {}, y: {}", slice_x, slice_y);
    }
}
