use std::f32::consts::TAU;

use bevy::{ecs::system::SystemId, prelude::*};

use crate::schedules::CubeStartupSet;

use super::{
    axis::Axis,
    controller::ControllerPlugin,
    cube_state::CubeState,
    interact_to_rotate::{self, InteractToRotatePlugin},
    rotation::CubeRotationPlugin,
};

pub const DEFAULT_CUBE_SIZE: usize = 3;
const SPACE_BETWEEN_PIECES: f32 = 0.04;
const PIECE_SIZE: f32 = 1.0;

/// orange
pub const COLOR_LEFT: Color = Color::srgb(0.99, 0.49, 0.05);
/// red
pub const COLOR_RIGHT: Color = Color::srgb(0.99, 0.0, 0.0);
/// white
pub const COLOR_TOP: Color = Color::srgb(0.99, 0.99, 0.99);
/// yellow
pub const COLOR_BOTTOM: Color = Color::srgb(0.99, 0.99, 0.0);
/// green
pub const COLOR_FRONT: Color = Color::srgb(0.027, 0.89, 0.215);
/// blue
pub const COLOR_BACK: Color = Color::srgb(0.0, 0.0, 0.99);

pub struct CubePlugin;

impl Plugin for CubePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentCubeSizeResource(DEFAULT_CUBE_SIZE))
            .init_resource::<CubeCommandsResource>()
            .add_plugins(ControllerPlugin)
            .add_plugins(CubeRotationPlugin)
            .add_plugins(InteractToRotatePlugin)
            .add_systems(Startup, spawn.in_set(CubeStartupSet::SpawnCube));
    }
}

/// For this simple example, we will just organize our systems
/// using string keys in a hash map.
#[derive(Resource)]
pub struct CubeCommandsResource {
    pub spawn: SystemId,
    pub despawn: SystemId,
}

impl FromWorld for CubeCommandsResource {
    fn from_world(world: &mut World) -> Self {
        CubeCommandsResource {
            spawn: world.register_system(spawn),
            despawn: world.register_system(despawn),
        }
    }
}

#[derive(Resource)]
pub struct CurrentCubeSizeResource(pub usize);

/// A 3D representation of a cube
#[derive(Component, Debug, Clone)]
pub struct Cube {
    /// For example, 3 for 3x3
    cube_size: CubeSize,
    /// The space between 2 pieces
    space_between_pieces: f32,
    piece_size: f32,
    inner_material: Handle<StandardMaterial>,
    pub is_animating_rotation: bool,
}

impl Cube {
    pub fn size(&self) -> &CubeSize {
        &self.cube_size
    }

    pub fn piece_size(&self) -> f32 {
        self.piece_size
    }

    /// The space between 2 pieces
    pub fn space_between_pieces(&self) -> f32 {
        self.space_between_pieces
    }
}

#[derive(Clone, Debug)]
pub struct CubeSize(pub i32);

impl CubeSize {
    pub fn lowest_piece_index(&self) -> i32 {
        if self.0 % 2 == 1 {
            -(self.0 as i32 - 1) / 2
        } else {
            -self.0 as i32 / 2
        }
    }

    pub fn highest_piece_index(&self) -> i32 {
        if self.0 % 2 == 1 {
            (self.0 as i32 - 1) / 2
        } else {
            self.0 as i32 / 2
        }
    }
}

#[derive(Component, Clone, Debug)]
#[require(InheritedVisibility)]
pub struct Piece {
    pub current_x: i32,
    pub current_y: i32,
    pub current_z: i32,
}

impl Piece {
    /// Get all piece indicies of a slice
    pub fn get_piece_indices(pieces: &Vec<&mut Piece>, axis: Axis, slice_index: i32) -> Vec<usize> {
        let mut result = vec![];

        for (i, piece) in pieces.iter().enumerate() {
            let current_piece_index = match axis {
                Axis::X => piece.current_x,
                Axis::Y => piece.current_y,
                Axis::Z => piece.current_z,
            };

            if current_piece_index != slice_index {
                continue;
            }

            result.push(i);
        }

        result
    }
}

#[derive(Component)]
pub struct PieceFace;

fn despawn(mut commands: Commands, query: Query<Entity, With<Cube>>) {
    let cube_entity = match query.get_single() {
        Ok(entity) => entity,
        Err(err) => {
            warn!("failed to despawn cube: {err}");
            return;
        }
    };

    commands.entity(cube_entity).despawn_recursive();
}

fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cube_size_resource: Res<CurrentCubeSizeResource>,
) {
    let cube_size = cube_size_resource.0;

    if cube_size <= 1 {
        error!("can not spawn cube with invalid size: {cube_size}");
        return;
    }

    let cube = Cube {
        cube_size: CubeSize(cube_size as i32),
        space_between_pieces: SPACE_BETWEEN_PIECES,
        piece_size: PIECE_SIZE,
        inner_material: materials.add(Color::srgb(0.1, 0.1, 0.1)),
        is_animating_rotation: false,
    };

    let piece_face_mesh = meshes.add(Rectangle {
        half_size: (Vec2::ONE * cube.piece_size) / 2.0,
    });

    let range = cube.size().lowest_piece_index()..=cube.size().highest_piece_index();

    let face_offset = cube.piece_size / 2.0;

    let mut cube_entity = commands.spawn((
        cube.clone(),
        CubeState::new(cube_size),
        Transform::default(),
        Visibility::Visible,
    ));

    // Spawn pieces
    for x in range.clone() {
        for y in range.clone() {
            for z in range.clone() {
                if cube.size().0 % 2 == 0 && (x == 0 || y == 0 || z == 0) {
                    continue;
                }

                // The middle point of the cube piece
                let middle_point = if cube.size().0 % 2 == 0 {
                    // even sized cubes like 2x2 and 4x4

                    let mut result = Vec3::new(
                        x as f32 * cube.piece_size + x as f32 * cube.space_between_pieces,
                        y as f32 * cube.piece_size + y as f32 * cube.space_between_pieces,
                        z as f32 * cube.piece_size + z as f32 * cube.space_between_pieces,
                    );

                    let offset = cube.piece_size / 2.0 + cube.space_between_pieces / 2.0;
                    result -= Vec3::new(
                        x.clamp(-1, 1) as f32 * offset,
                        y.clamp(-1, 1) as f32 * offset,
                        z.clamp(-1, 1) as f32 * offset,
                    );

                    result
                } else {
                    // even sized cubes like 3x3 and 5x5

                    Vec3::new(
                        x as f32 * cube.piece_size + x as f32 * cube.space_between_pieces,
                        y as f32 * cube.piece_size + y as f32 * cube.space_between_pieces,
                        z as f32 * cube.piece_size + z as f32 * cube.space_between_pieces,
                    )
                };

                cube_entity.with_children(|parent| {
                    let mut piece_entity = parent.spawn((
                        Piece {
                            current_x: x,
                            current_y: y,
                            current_z: z,
                        },
                        Transform::from_translation(middle_point),
                    ));

                    piece_entity.with_children(|parent| {
                        // left face
                        let mut transform =
                            Transform::from_translation(-Vec3::new(face_offset, 0.0, 0.0));
                        transform.rotate_local_y(-TAU / 4.0);

                        let material = if x == cube.size().lowest_piece_index() {
                            materials.add(COLOR_LEFT)
                        } else {
                            cube.inner_material.clone()
                        };

                        parent.spawn((
                            Mesh3d(piece_face_mesh.clone()),
                            transform,
                            MeshMaterial3d(material),
                            PieceFace,
                            PickingBehavior::IGNORE,
                        ));

                        // right face
                        let mut transform =
                            Transform::from_translation(Vec3::new(face_offset, 0.0, 0.0));
                        transform.rotate_local_y(TAU / 4.0);

                        let material = if x == cube.size().highest_piece_index() {
                            materials.add(COLOR_RIGHT)
                        } else {
                            cube.inner_material.clone()
                        };

                        parent.spawn((
                            Mesh3d(piece_face_mesh.clone()),
                            transform,
                            MeshMaterial3d(material),
                            PieceFace,
                            PickingBehavior::IGNORE,
                        ));

                        // top face
                        let mut transform =
                            Transform::from_translation(Vec3::new(0.0, face_offset, 0.0));
                        transform.rotate_x(-TAU / 4.0);

                        let material = if y == cube.size().highest_piece_index() {
                            materials.add(COLOR_TOP)
                        } else {
                            cube.inner_material.clone()
                        };

                        parent.spawn((
                            Mesh3d(piece_face_mesh.clone()),
                            transform,
                            MeshMaterial3d(material),
                            PieceFace,
                            PickingBehavior::IGNORE,
                        ));

                        // bottom face
                        let mut transform =
                            Transform::from_translation(-Vec3::new(0.0, face_offset, 0.0));
                        transform.rotate_x(TAU / 4.0);

                        let material = if y == cube.size().lowest_piece_index() {
                            materials.add(COLOR_BOTTOM)
                        } else {
                            cube.inner_material.clone()
                        };

                        parent.spawn((
                            Mesh3d(piece_face_mesh.clone()),
                            transform,
                            MeshMaterial3d(material),
                            PieceFace,
                            PickingBehavior::IGNORE,
                        ));

                        // front face
                        let transform =
                            Transform::from_translation(Vec3::new(0.0, 0.0, face_offset));

                        let material = if z == cube.size().highest_piece_index() {
                            materials.add(COLOR_FRONT)
                        } else {
                            cube.inner_material.clone()
                        };

                        parent.spawn((
                            Mesh3d(piece_face_mesh.clone()),
                            transform,
                            MeshMaterial3d(material),
                            PieceFace,
                            PickingBehavior::IGNORE,
                        ));

                        // back face
                        let mut transform =
                            Transform::from_translation(-Vec3::new(0.0, 0.0, face_offset));
                        transform.rotate_local_y(-TAU / 2.0);

                        let material = if z == cube.size().lowest_piece_index() {
                            materials.add(COLOR_BACK)
                        } else {
                            cube.inner_material.clone()
                        };

                        parent.spawn((
                            Mesh3d(piece_face_mesh.clone()),
                            transform,
                            MeshMaterial3d(material),
                            PieceFace,
                            PickingBehavior::IGNORE,
                        ));
                    });
                });
            }
        }
    }

    // Spawn collider squares
    cube_entity.with_children(|parent| {
        interact_to_rotate::spawn(
            parent,
            &mut materials,
            &mut meshes,
            cube_size,
            cube.piece_size,
            cube.space_between_pieces,
        );
    });
}
