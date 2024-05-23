use std::f32::consts::TAU;

use bevy::prelude::*;

const BLOCKS_SPREAD: f32 = 0.25;
const BLOCKS_SIZE: f32 = 1.0;
const CUBE_SIZE: u32 = 3; // 3 for 3x3, 6 for 6x6 etc

const COLOR_INSIDE_R: f32 = 0.1;
const COLOR_INSIDE_G: f32 = 0.1;
const COLOR_INSIDE_B: f32 = 0.1;

pub struct RubiksCubePlugin;

impl Plugin for RubiksCubePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_light, spawn_rubiks_cube))
            .add_systems(Update, rotate_whole_cube);
    }
}

#[derive(Component)]
struct CubeBlock {
    pub faces: [Entity; 6],
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

fn spawn_light(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
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

                let face_left = commands
                    .spawn((
                        PbrBundle {
                            mesh: cube_face_mesh.clone(),
                            transform: transform,
                            material: materials.add(StandardMaterial {
                                base_color: Color::rgb(1.0, 0.0, 0.0),
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
                transform.rotate_local_y(TAU / 4.0);
                let face_right = commands
                    .spawn((
                        PbrBundle {
                            mesh: cube_face_mesh.clone(),
                            transform: transform,
                            material: materials.add(StandardMaterial {
                                base_color: Color::rgb(0.0, 1.0, 0.0),
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
                let face_top = commands
                    .spawn((
                        PbrBundle {
                            mesh: cube_face_mesh.clone(),
                            transform: transform,
                            material: materials.add(StandardMaterial {
                                base_color: Color::rgb(0.0, 0.0, 1.0),
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
                transform.rotate_x(-TAU / 4.0);
                let face_bottom = commands
                    .spawn((
                        PbrBundle {
                            mesh: cube_face_mesh.clone(),
                            transform: transform,
                            material: materials.add(StandardMaterial {
                                base_color: Color::rgb(1.0, 1.0, 0.0),
                                ..default()
                            }),
                            ..default()
                        },
                        CubeFace,
                    ))
                    .id();
                // front
                let face_front = commands
                    .spawn((
                        PbrBundle {
                            mesh: cube_face_mesh.clone(),
                            transform: Transform::from_translation(
                                middle_point + Vec3::new(0.0, 0.0, face_offset),
                            ),
                            material: materials.add(StandardMaterial {
                                base_color: Color::rgb(0.0, 1.0, 1.0),
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
                let face_back = commands
                    .spawn((
                        PbrBundle {
                            mesh: cube_face_mesh.clone(),
                            transform: transform,
                            material: materials.add(StandardMaterial {
                                base_color: Color::rgb(1.0, 1.0, 0.0),
                                ..default()
                            }),
                            ..default()
                        },
                        CubeFace,
                    ))
                    .id();

                // commands.spawn(CubeBlock {
                //     faces: [
                //         face_left,
                //         face_right,
                //         face_top,
                //         face_bottom,
                //         face_front,
                //         face_back,
                //     ],
                // });
            }
        }
    }
}

fn rotate_whole_cube(mut query: Query<&mut Transform, With<CubeFace>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_rotation_y(time.delta_seconds() / 1.5),
        );
    }
}
