// projectile.rs

use avian3d::prelude::{Collider, DebugRender, RigidBody};
use bevy::prelude::*;
use bevy::input::{mouse::MouseButton, ButtonInput};

use crate::MyMusic;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, fire_weapon)
            .add_systems(Update, update_projectiles);
    }
}

#[derive(Component)]
pub struct Projectile {
    pub velocity: Vec3,
}

impl Default for Projectile {
    fn default() -> Self {
        Projectile { velocity: Vec3::ZERO }
    }
}

fn fire_weapon(
    mut commands: Commands,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    asset_server: Res<AssetServer>,
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

        let collider = Collider::sphere(0.5);
        commands.spawn((
            RigidBody::Dynamic,
            collider,
            DebugRender::default().with_collider_color(Color::srgb(0.0, 1.0, 0.0)),
            PbrBundle {
                mesh: meshes.add(Mesh::from(Sphere::default()
                    .mesh()
                    .ico(5)
                    .unwrap())),
                material: materials.add(StandardMaterial { base_color: Color::WHITE, ..default() }),
                transform: Transform::from_translation(camera_transform.translation()),
                ..Default::default()
            },
            Projectile {
                velocity: (point - camera_transform.translation()).normalize() * 40.0, // Adjust the speed as needed
            }
        ));

        commands.spawn((
            AudioBundle {
                source: asset_server.load("sounds/queef.ogg"),
                ..default()
            },
            MyMusic,
        ));
    }
}

fn update_projectiles(
    mut projectiles: Query<(&mut Transform, &Projectile)>,
    time: Res<Time>,
) {
    for (mut transform, projectile) in projectiles.iter_mut() {
        transform.translation += projectile.velocity * time.delta_seconds();
    }
}
