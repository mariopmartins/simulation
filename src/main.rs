use bevy::{prelude::*};
use std::fmt;

fn main() {
    App::build()
        .add_plugins(MinimalPlugins)
        .add_plugin(HelloPlugin)
        .run();
}

impl fmt::Display for VehicleStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           VehicleStatus::Parked => write!(f, "Parked"),
           VehicleStatus::Driving => write!(f, "Driving"),
       }
    }
}

enum VehicleStatus {
    Parked,
    Driving
}

impl fmt::Display for VehicleType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           VehicleType::Car => write!(f, "Car"),
           VehicleType::Truck => write!(f, "Truck"),
           VehicleType::Bike => write!(f, "Bike"),
       }
    }
}

enum VehicleType {
    Car,
    Truck,
    Bike
}

struct GreetTimer(Timer);


struct Vehicle;
struct Type(VehicleType);
struct Status(VehicleStatus);

fn setup(mut commands: Commands) {
    commands
        .spawn((Vehicle, Type(VehicleType::Car), Status(VehicleStatus::Parked)))
        .spawn((Vehicle, Type(VehicleType::Car), Status(VehicleStatus::Driving)))
        .spawn((Vehicle, Type(VehicleType::Truck), Status(VehicleStatus::Parked)))
        .spawn((Vehicle, Type(VehicleType::Truck), Status(VehicleStatus::Driving)))
        .spawn((Vehicle, Type(VehicleType::Bike), Status(VehicleStatus::Parked)))
        .spawn((Vehicle, Type(VehicleType::Bike), Status(VehicleStatus::Driving)));
}

fn print_status(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<(&Vehicle, &Type, &Status)>) {
    timer.0.tick(time.delta_seconds);
    if timer.0.finished {
        for (_vehicle, _type, _status) in query.iter() {
            println!("--> {} {}", _type.0, _status.0);
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(GreetTimer(Timer::from_seconds(2.0, true)))
           .add_startup_system(setup.system())
           .add_system(print_status.system());
    }
}

