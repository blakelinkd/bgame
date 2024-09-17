//! This example demonstrates how to use the `Camera::viewport_to_world` method.

use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::input::mouse::MouseButton;
use bevy::render::{
    render_asset::RenderAssetUsages,
    render_resource::{Extent3d, TextureDimension, TextureFormat},
};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_cursor)
        .add_systems(Update, update_projectiles)
        .run();
}


fn update_projectiles(
    mut projectiles: Query<(&mut Transform, &Projectile)>,
    time: Res<Time>,
) {
    for (mut transform, projectile) in projectiles.iter_mut() {
        transform.translation += projectile.velocity * time.delta_seconds();
    }
}


fn draw_cursor(
    mut commands: Commands,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let (camera, camera_transform) = camera_query.single();

    if mouse_buttons.just_pressed(MouseButton::Left) {
        let Some(cursor_position) = windows.single().cursor_position() else {
            return;
        };

        // Calculate a ray pointing from the camera into the world based on the cursor's position.
        let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
            return;
        };

        // Set a default distance or get the distance from somewhere
        let distance = 100.0; // Adjust as needed

        let point = ray.get_point(distance);

        println!("hit at {:?}", point);

        // Spawn a projectile and set its velocity towards the hit point.
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Sphere::default().mesh().ico(5).unwrap())),
            material: materials.add(StandardMaterial { base_color: Color::WHITE, ..default() }),
            transform: Transform::from_translation(camera_transform.translation()), // Start from the camera position
            ..Default::default()
        })
        .insert(Projectile {
            velocity: (point - camera_transform.translation()).normalize() * 10.0, // adjust the speed as needed
            ..default()
        });
    }
}



#[derive(Component)]
struct Ground;

#[derive(Component)]
struct Shape;

#[derive(Component)]
struct Projectile {
    velocity: Vec3,
}

impl Default for Projectile {
    fn default() -> Self {
        Self {
            velocity: Vec3::ZERO,
        }
    }
}


const SHAPES_X_EXTENT: f32 = 14.0;
const Z_EXTENT: f32 = 5.0;


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });
    let shapes = [
        meshes.add(Cuboid::default()),
        meshes.add(Tetrahedron::default()),
        meshes.add(Capsule3d::default()),
        meshes.add(Torus::default()),
        meshes.add(Cylinder::default()),
        meshes.add(Cone::default()),
        meshes.add(ConicalFrustum::default()),
        meshes.add(Sphere::default().mesh().ico(5).unwrap()),
        meshes.add(Sphere::default().mesh().uv(32, 18)),
    ];

    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        commands.spawn((
            PbrBundle {
                mesh: shape,
                material: debug_material.clone(),
                transform: Transform::from_xyz(
                    -SHAPES_X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * SHAPES_X_EXTENT,
                    2.0,
                    Z_EXTENT / 2.,
                )
                .with_rotation(Quat::from_rotation_x(-PI / 4.)),
                ..default()
            },
            Shape,
        ));
    }
    // plane
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(20., 20.)),
            material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
            ..default()
        },
        Ground,
    ));

    // light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(15.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
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