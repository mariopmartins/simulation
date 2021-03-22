use std::env;
// use std::fmt;
use bevy::{
    prelude::*, render::{
        mesh::Mesh,
    }};

/// set up a simple 3D scene
fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // add entities to the world
    commands
        // plane
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        // cube
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)),
            ..Default::default()
        })
        // light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        })
        // camera rig
        .spawn((
                Transform::from_translation(Vec3::new(-3.0, 5.0, 8.0))
                    .looking_at(Vec3::default(), Vec3::unit_y()),
                GlobalTransform::default(),
                CameraRig,
        ))
        .with_children(|parent| {
            // camera
            parent.spawn(Camera3dBundle {
                transform: Transform::from_translation(Vec3::new(-3.0, 5.0, 8.0))
                    .looking_at(Vec3::default(), Vec3::unit_y()),
                ..Default::default()
            });
        });
}

struct CameraRig;

fn camera_rig_controller_system(time: Res<Time>, keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&CameraRig, &mut Transform)>) {
    let movement_speed = 5.0;
    let movement_time = 5.0;
    let rotation_amount = 1.0;
    let zoom_amount = Vec3::new(0.0, -10.0, 10.0);
    //let minimal_zoom = Vec3::new(0.0, 3.0, -3.0);
    //let maximum_zoom = Vec3::new(0.0, 50.0, -50.0);

    let mut new_move = false;
    let mut new_translation = Vec3::one();
    let mut new_rotation_amount = Quat::identity();
    let mut new_zoom = Vec3::default();

    for (_camera_rig, mut transform) in query.iter_mut() {
        // movement
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            new_translation += transform.forward() * movement_speed;
            new_move = true;
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            new_translation += transform.forward() * -movement_speed;
            new_move = true;
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            new_translation += transform.rotation * Vec3::unit_x() * movement_speed;
            new_move = true;
        }
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            new_translation += transform.rotation * Vec3::unit_x() * -movement_speed;
            new_move = true;
        }

        // rotation
        if keyboard_input.pressed(KeyCode::Q) {
            new_rotation_amount *= Quat::from_rotation_y(rotation_amount);
            new_move = true;
        }
        if keyboard_input.pressed(KeyCode::E) {
            new_rotation_amount *= Quat::from_rotation_y(-rotation_amount);
            new_move = true;
        }

        // zoom
        if keyboard_input.pressed(KeyCode::R) {
            new_zoom += zoom_amount;
            new_move = true;
        }
        if keyboard_input.pressed(KeyCode::F) {
            new_zoom -= zoom_amount;
            new_move = true;
        }

        // handle movement
        if new_move { 
            transform.translation = Vec3::lerp(transform.translation, new_translation, movement_time * time.delta_seconds());
            transform.rotation = Quat::lerp(transform.rotation, new_rotation_amount, time.delta_seconds());
            new_move = false;
        }
    }
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: env::var("CARGO_PKG_NAME").unwrap_or("Game".to_string()),
            ..Default::default()
        })
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_system(camera_rig_controller_system.system())
        .add_startup_system(setup.system())
        .run();
}
