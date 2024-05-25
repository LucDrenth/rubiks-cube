use std::f32::consts::TAU;

use bevy::prelude::*;

use super::controller::ControllerPlugin;

const BLOCKS_SPREAD: f32 = 0.05;
const BLOCKS_SIZE: f32 = 1.0;
const CUBE_SIZE: u32 = 3; // 3 for 3x3, 6 for 6x6 etc

const COLOR_INSIDE_R: f32 = 0.1;
const COLOR_INSIDE_G: f32 = 0.1;
const COLOR_INSIDE_B: f32 = 0.1;

pub struct RubiksCubePlugin;

impl Plugin for RubiksCubePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CubeRotationEvent>()
            .add_plugins(ControllerPlugin)
            .add_systems(Startup, (spawn_light, spawn_rubiks_cube))
            .add_systems(Update, rotation_events_handler)
            // .add_systems(Update, rotate_whole_cube)
            ;
    }
}

#[derive(Component, Debug)]
struct CubeBlock {
    pub faces: [Entity; 6],
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Component, Debug)]
struct RubiksCube {
    blocks: Vec<CubeBlock>,
}

impl RubiksCube {
    fn get_cube_indices_with_coords(
        &self,
        x: Option<i32>,
        y: Option<i32>,
        z: Option<i32>,
    ) -> Vec<usize> {
        let mut result = vec![];

        for (i, block) in self.blocks.iter().enumerate() {
            if let Some(x) = x {
                if block.x != x {
                    continue;
                }
            }

            if let Some(y) = y {
                if block.y != y {
                    continue;
                }
            }

            if let Some(z) = z {
                if block.z != z {
                    continue;
                }
            }

            result.push(i);
        }

        result
    }
}

#[derive(Component)]
struct CubeFace;

pub enum Face {
    Left = 0,
    Right = 2,
    Top = 3,
    Bottom = 4,
    Front = 5,
    Back = 6,
}

#[derive(Event)]
pub struct CubeRotationEvent {
    pub rotation: CubeRotation3x3,
    pub counter_clockwise: bool,
    pub twice: bool,
}

/// Cube rotations in true cubing notation. Tailored for 3x3 cubes.
pub enum CubeRotation3x3 {
    /// Left side. Clockwise is towards the camera
    Left,
    /// Left and middle (SliceM)
    WideLeft,
    /// Right side. Clockwise is away from the camera
    Right,
    /// Right and middle (SliceM)
    WideRight,
    /// Top side. When holding the cube in front of you, the front row goes to the left
    Top,
    /// Top and middle (SliceE)
    WideTop,
    // Bottom side. When holding the cube in front of you, the front row goes to the right
    Bottom,
    /// Bottom and middle (SliceE)
    WideBottom,
    /// TODO comment
    Back,
    /// TODO comment
    Front,
    /// Between left and right (M stand for Middle). // TODO comment direction
    SliceM,
    /// Between top and bottom (E stand for Equoator). // TODO comment direction
    SliceE,
    /// Between front and back (S stand for Standing). // TODO comment direction
    SliceS,
    /// Rotate whole cube in the x direction // TODO comment direction
    X,
    /// Rotate whole cube in the y direction // TODO comment direction
    Y,
    /// Rotate whole cube in the z direction // TODO comment direction
    Z,
}

fn spawn_light(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 300.0,
            shadow_depth_bias: 0.8,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 15.0, 15.0),
        ..default()
    });
}

fn spawn_rubiks_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube_face_mesh = meshes.add(Rectangle {
        half_size: Vec2::ONE * BLOCKS_SIZE / 2.0,
    });

    if CUBE_SIZE < 2 {
        panic!("Invalid cube size {}", CUBE_SIZE)
    }

    let mut offset = 0.0;

    let range = if CUBE_SIZE % 2 == 1 {
        -(CUBE_SIZE as i32 - 1) / 2..=(CUBE_SIZE as i32 - 1) / 2
    } else {
        offset = BLOCKS_SIZE / 2.0;
        -(CUBE_SIZE as i32 / 2) + 1..=CUBE_SIZE as i32 / 2
    };

    let spread_factor = 1.0 + BLOCKS_SPREAD;
    let face_offset = BLOCKS_SIZE / 2.0;

    let color_inside = Color::rgb(COLOR_INSIDE_R, COLOR_INSIDE_G, COLOR_INSIDE_B);

    let mut rubiks_cube = RubiksCube {
        blocks: Vec::with_capacity((CUBE_SIZE * CUBE_SIZE * CUBE_SIZE) as usize),
    };

    for x in range.clone() {
        for y in range.clone() {
            for z in range.clone() {
                let middle_point = Vec3::new(
                    x as f32 * BLOCKS_SIZE - offset,
                    y as f32 * BLOCKS_SIZE - offset,
                    z as f32 * BLOCKS_SIZE - offset,
                ) * spread_factor;

                // left
                let mut transform =
                    Transform::from_translation(middle_point - Vec3::new(face_offset, 0.0, 0.0));
                transform.rotate_local_y(-TAU / 4.0);

                // TODO this will only hold for 3x3
                let color = if x == -1 {
                    Color::rgb(0.99, 0.49, 0.05) // orange
                } else {
                    color_inside
                };

                let face_left = commands
                    .spawn((
                        PbrBundle {
                            mesh: cube_face_mesh.clone(),
                            transform: transform,
                            material: materials.add(StandardMaterial {
                                base_color: color,
                                ..default()
                            }),
                            ..default()
                        },
                        CubeFace,
                    ))
                    .id();

                // right
                let mut transform =
                    Transform::from_translation(middle_point + Vec3::new(face_offset, 0.0, 0.0));

                // TODO this will only hold for 3x3
                let color = if x == 1 {
                    Color::rgb(0.99, 0.0, 0.0) // red
                } else {
                    color_inside
                };

                transform.rotate_local_y(TAU / 4.0);
                let face_right = commands
                    .spawn((
                        PbrBundle {
                            mesh: cube_face_mesh.clone(),
                            transform: transform,
                            material: materials.add(StandardMaterial {
                                base_color: color,
                                ..default()
                            }),
                            ..default()
                        },
                        CubeFace,
                    ))
                    .id();

                // top
                let mut transform =
                    Transform::from_translation(middle_point + Vec3::new(0.0, face_offset, 0.0));
                transform.rotate_x(-TAU / 4.0);

                // TODO this will only hold for 3x3
                let color = if y == 1 {
                    Color::rgb(0.99, 0.99, 0.99) // white
                } else {
                    color_inside
                };

                let face_top = commands
                    .spawn((
                        PbrBundle {
                            mesh: cube_face_mesh.clone(),
                            transform: transform,
                            material: materials.add(StandardMaterial {
                                base_color: color,
                                ..default()
                            }),
                            ..default()
                        },
                        CubeFace,
                    ))
                    .id();

                // bottom
                let mut transform =
                    Transform::from_translation(middle_point - Vec3::new(0.0, face_offset, 0.0));
                transform.rotate_x(TAU / 4.0);

                // TODO this will only hold for 3x3
                let color = if y == -1 {
                    Color::rgb(0.99, 0.99, 0.0) // yellow
                } else {
                    color_inside
                };

                let face_bottom = commands
                    .spawn((
                        PbrBundle {
                            mesh: cube_face_mesh.clone(),
                            transform: transform,
                            material: materials.add(StandardMaterial {
                                base_color: color,
                                ..default()
                            }),
                            ..default()
                        },
                        CubeFace,
                    ))
                    .id();

                // front
                // TODO this will only hold for 3x3
                let color = if z == 1 {
                    Color::rgb(7.0 / 255.0, 227.0 / 255.0, 55.0 / 255.0) // green
                } else {
                    color_inside
                };

                let face_front = commands
                    .spawn((
                        PbrBundle {
                            mesh: cube_face_mesh.clone(),
                            transform: Transform::from_translation(
                                middle_point + Vec3::new(0.0, 0.0, face_offset),
                            ),
                            material: materials.add(StandardMaterial {
                                base_color: color,
                                ..default()
                            }),
                            ..default()
                        },
                        CubeFace,
                    ))
                    .id();

                // back
                let mut transform =
                    Transform::from_translation(middle_point - Vec3::new(0.0, 0.0, face_offset));
                transform.rotate_local_y(-TAU / 2.0);

                // TODO this will only hold for 3x3
                let color = if z == -1 {
                    Color::rgb(0.0, 0.0, 0.99) // blue
                } else {
                    color_inside
                };

                let face_back = commands
                    .spawn((
                        PbrBundle {
                            mesh: cube_face_mesh.clone(),
                            transform: transform,
                            material: materials.add(StandardMaterial {
                                base_color: color,
                                ..default()
                            }),
                            ..default()
                        },
                        CubeFace,
                    ))
                    .id();

                rubiks_cube.blocks.push(CubeBlock {
                    faces: [
                        face_left,
                        face_right,
                        face_top,
                        face_bottom,
                        face_front,
                        face_back,
                    ],
                    x,
                    y,
                    z,
                });
            }
        }
    }

    commands.spawn(rubiks_cube);
}

fn rotate_whole_cube(mut query: Query<&mut Transform, With<CubeFace>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_rotation_y(time.delta_seconds() / 1.5),
        );
    }
}

fn rotation_events_handler(
    mut query: Query<&mut RubiksCube>,
    mut faces_query: Query<&mut Transform, With<CubeFace>>,
    mut event_reader: EventReader<CubeRotationEvent>,
) {
    let Ok(mut rubiks_cube) = query.get_single_mut() else {
        error!("expected exactly 1 RubiksCube entity");
        return;
    };

    let lowest_cube_coord = -1; // TODO only holds for 3x3
    let highest_cube_coord = 1; // TODO only holds for 3x3
    let mut rotation_amount = TAU / 4.0;

    for cube_rotation in event_reader.read() {
        if cube_rotation.twice {
            rotation_amount *= 2.;
        }

        match cube_rotation.rotation {
            CubeRotation3x3::Left => {
                if cube_rotation.counter_clockwise {
                    rotation_amount *= -1.;
                }

                let pivot_point = Vec3::new(-(CUBE_SIZE as f32 + BLOCKS_SPREAD), 0.0, 0.0); // TODO this only holds for 3x3
                let cubes_indices_to_rotate =
                    rubiks_cube.get_cube_indices_with_coords(Some(lowest_cube_coord), None, None);

                for cube_index_to_rotate in cubes_indices_to_rotate {
                    for face in rubiks_cube.blocks[cube_index_to_rotate].faces {
                        match faces_query.get_mut(face) {
                            Ok(mut transform) => {
                                transform.rotate_around(
                                    pivot_point,
                                    Quat::from_rotation_x(rotation_amount),
                                );
                            }
                            Err(err) => {
                                error!("failed to get cube face: {}", err);
                            }
                        }
                    }
                }

                // TODO update cube indices
            }
            CubeRotation3x3::WideLeft => todo!(),
            CubeRotation3x3::Right => todo!(),
            CubeRotation3x3::WideRight => todo!(),
            CubeRotation3x3::Top => todo!(),
            CubeRotation3x3::WideTop => todo!(),
            CubeRotation3x3::Bottom => todo!(),
            CubeRotation3x3::WideBottom => todo!(),
            CubeRotation3x3::Back => todo!(),
            CubeRotation3x3::Front => todo!(),
            CubeRotation3x3::SliceM => todo!(),
            CubeRotation3x3::SliceE => todo!(),
            CubeRotation3x3::SliceS => todo!(),
            CubeRotation3x3::X => todo!(),
            CubeRotation3x3::Y => todo!(),
            CubeRotation3x3::Z => todo!(),
        }
    }
}
