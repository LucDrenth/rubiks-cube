use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};

const BLOCKS_SPREAD: f32 = 0.05;
const BLOCKS_SIZE: f32 = 1.0;
const CUBE_SIZE: u32 = 4;

pub struct RubiksCubePlugin;

impl Plugin for RubiksCubePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_light, spawn_rubiks_cube))
            .add_systems(Update, rotate_whole_cube);
    }
}

#[derive(Component)]
struct CubeBlock;

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
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Cuboid {
        half_size: Vec3::ONE * BLOCKS_SIZE / 2.0,
    });
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    let spread_factor = 1.0 + BLOCKS_SPREAD;

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

    for x in range.clone() {
        for y in range.clone() {
            for z in range.clone() {
                commands.spawn((
                    PbrBundle {
                        mesh: mesh.clone(),
                        material: material.clone(),
                        transform: Transform::from_translation(
                            Vec3::new(
                                x as f32 * BLOCKS_SIZE - offset,
                                y as f32 * BLOCKS_SIZE - offset,
                                z as f32 * BLOCKS_SIZE - offset,
                            ) * spread_factor,
                        ),
                        ..default()
                    },
                    CubeBlock,
                ));
            }
        }
    }
}

fn rotate_whole_cube(mut query: Query<&mut Transform, With<CubeBlock>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_rotation_y(time.delta_seconds() / 3.0),
        );
    }
}

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}
