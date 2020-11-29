use std::env;
// use std::fmt;
use bevy::{
    input::keyboard::KeyboardInput,
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    prelude::*,
    window::CursorMoved,
};

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // add entities to the world
    commands
        // plane
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        
        
        // cube
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)),
            ..Default::default()
        })
        // light
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        })
        // camera rig
        .spawn(PbrComponents {
            transform: Transform::from_translation(Vec3::new(-3.0, 5.0, 8.0)),
            ..Default::default()
        })
        .with(CameraRig)
        .with_children(|parent| {
            // camera
            parent.spawn(Camera3dComponents {
                transform: Transform::from_translation(Vec3::new(-3.0, 5.0, 8.0))
                    .looking_at(Vec3::default(), Vec3::unit_y()),
                ..Default::default()
            });
        });
}

#[derive(Default)]
struct MouseState {
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    mouse_motion_event_reader: EventReader<MouseMotion>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
    mouse_wheel_event_reader: EventReader<MouseWheel>,
}

/// This system prints out all mouse events as they come in
fn print_mouse_events_system(
    mut state: Local<MouseState>,
    mouse_button_input_events: Res<Events<MouseButtonInput>>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    mouse_wheel_events: Res<Events<MouseWheel>>,
) {
    for event in state
        .mouse_button_event_reader
        .iter(&mouse_button_input_events) {
        println!("{:?}", event);
    }

    for event in state.mouse_motion_event_reader.iter(&mouse_motion_events) {
        println!("{:?}", event);
    }

    for event in state.cursor_moved_event_reader.iter(&cursor_moved_events) {
        println!("{:?}", event);
    }

    for event in state.mouse_wheel_event_reader.iter(&mouse_wheel_events) {
        println!("{:?}", event);
    }
}

#[derive(Default)]
struct KeyboardState {
    event_reader: EventReader<KeyboardInput>,
}

/// This system prints out all keyboard events as they come in
fn print_keyboard_event_system(
    mut state: Local<KeyboardState>,
    keyboard_input_events: Res<Events<KeyboardInput>>,
) {
    for event in state.event_reader.iter(&keyboard_input_events) {
        println!("{:?}", event);
    }
}


struct CameraRig;

fn camera_rig_controller_system(time: Res<Time>, mut query: Query<(&CameraRig, &mut Transform)>) {
    
    for (_rotator, mut transform) in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_x(3.0 * time.delta_seconds);
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
        .add_system(print_mouse_events_system.system())
        .add_system(print_keyboard_event_system.system())
        .add_startup_system(setup.system())
        .run();
}

// fn main() {
//     App::build()
//         .add_plugins(MinimalPlugins)
//         .add_plugin(HelloPlugin)
//         .run();
// }

// impl fmt::Display for VehicleStatus {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        match *self {
//            VehicleStatus::Parked => write!(f, "Parked"),
//            VehicleStatus::Driving => write!(f, "Driving"),
//        }
//     }
// }

// enum VehicleStatus {
//     Parked,
//     Driving
// }

// impl fmt::Display for VehicleType {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        match *self {
//            VehicleType::Car => write!(f, "Car"),
//            VehicleType::Truck => write!(f, "Truck"),
//            VehicleType::Bike => write!(f, "Bike"),
//        }
//     }
// }

// enum VehicleType {
//     Car,
//     Truck,
//     Bike
// }

// struct GreetTimer(Timer);

// struct Vehicle;
// struct Type(VehicleType);
// struct Status(VehicleStatus);

// fn setup(mut commands: Commands) {
//     commands
//         .spawn((Vehicle, Type(VehicleType::Car), Status(VehicleStatus::Parked)))
//         .spawn((Vehicle, Type(VehicleType::Car), Status(VehicleStatus::Driving)))
//         .spawn((Vehicle, Type(VehicleType::Truck), Status(VehicleStatus::Parked)))
//         .spawn((Vehicle, Type(VehicleType::Truck), Status(VehicleStatus::Driving)))
//         .spawn((Vehicle, Type(VehicleType::Bike), Status(VehicleStatus::Parked)))
//         .spawn((Vehicle, Type(VehicleType::Bike), Status(VehicleStatus::Driving)));
// }

// fn print_status(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<(&Vehicle, &Type, &Status)>) {
//     timer.0.tick(time.delta_seconds);
//     if timer.0.finished {
//         for (_vehicle, _type, _status) in query.iter() {
//             println!("--> {} {}", _type.0, _status.0);
//         }
//     }
// }

// pub struct HelloPlugin;

// impl Plugin for HelloPlugin {
//     fn build(&self, app: &mut AppBuilder) {
//         app.add_resource(GreetTimer(Timer::from_seconds(2.0, true)))
//            .add_startup_system(setup.system())
//            .add_system(print_status.system());
//     }
// }
