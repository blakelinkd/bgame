use std::f32::consts::PI;

use bevy::prelude::*;
use avian3d::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::render::{
    render_asset::RenderAssetUsages,
    render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy::input::ButtonInput;
use bevy::prelude::Res;
use bevy::window::CursorGrabMode;
use ui::GameUiPlugin;




mod projectile;
mod ui;
#[derive(Debug, Component)]
struct Player;

#[derive(Debug, Component)]
struct WorldModelCamera;

#[derive(Component)]
struct Health {
    is_alive: bool,
}


#[derive(Component)]
struct MyMusic;



fn main() {
    App::new()
        .add_plugins((GameUiPlugin, DefaultPlugins, PhysicsPlugins::default(),
        projectile::ProjectilePlugin,
        PhysicsDebugPlugin::default()))
        .insert_gizmo_config(
            PhysicsGizmos {
                aabb_color: Some(Color::WHITE),
                ..default()
            },
            GizmoConfig::default(),
        )
        .insert_resource(Gravity(Vec3::NEG_Y * 19.6))
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .add_systems(Startup, lock_and_hide_cursor)
        .run();
}


fn lock_and_hide_cursor(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;
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

fn move_player(
    mut mouse_motion: EventReader<MouseMotion>,
    mut player: Query<(&mut Transform, &mut ExternalImpulse, &mut Health), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let (mut transform, mut external_impulse, health) = player.single_mut();
    for motion in mouse_motion.read() {
        let yaw = -motion.delta.x * 0.003;
        let pitch = -motion.delta.y * 0.002;
        // Order of rotations is important, see <https://gamedev.stackexchange.com/a/136175/103059>
        transform.rotate_y(yaw);
        transform.rotate_local_x(pitch);
    }

    if keyboard_input.pressed(KeyCode::KeyW) {
        let forward = transform.forward();
        transform.translation += forward * 0.1; // Adjust the speed as needed
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        let backward = -transform.forward();
        transform.translation += backward * 0.1; // Adjust the speed as needed
    }

    if keyboard_input.pressed(KeyCode::KeyA) {
        let left = -transform.right();
        transform.translation += left * 0.1; // Adjust the speed as needed
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        let right = transform.right();
        transform.translation += right * 0.1; // Adjust the speed as needed
    }

    if keyboard_input.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }



    if keyboard_input.just_pressed(KeyCode::Space) && health.is_alive {
        // Apply an upward force to simulate a jump
        external_impulse.apply_impulse(Vec3::Y * 80.0);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
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
        // meshes.add(Sphere::default().mesh().ico(5).unwrap()),
        // meshes.add(Sphere::default().mesh().uv(32, 18)),
    ];

    let num_shapes = shapes.len();

    commands.spawn((
        AudioBundle {
            source: asset_server.load("sounds/lazer.ogg"),
            ..default()
        },
        MyMusic,
    ));

    commands.spawn((
        Player,
        SpatialBundle {
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::capsule(1.0, 0.5), // Add a collision capsule to the player
        ExternalImpulse::default(),
        Health { is_alive: true }, // Initialize the player as alive
        LockedAxes::ROTATION_LOCKED, // Lock the rotation axes
        DebugRender::default().with_collider_color(Color::srgb(0.0, 1.0, 0.0)),
    )).with_children(|parent| {
        parent.spawn((
            WorldModelCamera,
            Camera3dBundle {
                projection: PerspectiveProjection {
                    fov: 90.0_f32.to_radians(),
                    ..default()
                }
                .into(),
                ..default()
            },
            FogSettings {
                color: Color::srgb(0.25, 0.25, 0.25),
                falloff: FogFalloff::Linear {
                    start: 5.0,
                    end: 20.0,
                },
                ..default()
            },
        ));
    });

    for (i, shape) in shapes.into_iter().enumerate() {
        let collider = match shape {
            _ if shape == meshes.add(Cuboid::default()) => Collider::cuboid(0.5, 0.5, 0.5),
            _ if shape == meshes.add(Capsule3d::default()) => Collider::capsule(1.0, 0.5),
            _ if shape == meshes.add(Torus::default()) => Collider::capsule(1.0, 0.3),
            _ if shape == meshes.add(Cylinder::default()) => Collider::cylinder(1.0, 0.5),
            _ if shape == meshes.add(Cone::default()) => Collider::cone(1.0, 0.5),
            _ => Collider::capsule(1.0, 0.5), // Default case
        };

        commands.spawn((
            PbrBundle {
            mesh: shape,
            material: debug_material.clone(),
            transform: Transform::from_xyz(
                -SHAPES_X_EXTENT + i as f32 * (SHAPES_X_EXTENT / (num_shapes - 1) as f32 * 2.0),
                2.0,
                Z_EXTENT / 2.,
            )
            .with_rotation(Quat::from_rotation_x(-PI / 4.)),
            ..default()
            },
            Shape,
            RigidBody::Dynamic,
            collider,
            DebugRender::default().with_collider_color(Color::srgb(1.0, 0.0, 0.0)),
        ));
    }

    
    
    
    // plane
    let plane_mesh_builder = Plane3d::default().mesh().size(2000., 2000.);
    let plane_mesh = Mesh::from(plane_mesh_builder);
    let plane_collider = Collider::trimesh_from_mesh(&plane_mesh).unwrap();
    commands.spawn((
        plane_collider,
        PbrBundle {
            mesh: meshes.add(plane_mesh),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.8, 0.8, 0.8),
                ..default()
            }),
            ..default()
        },
        Ground,
        RigidBody::Kinematic,
    ));

    // light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
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


