use bevy::prelude::*;
use std::fmt;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
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

fn greet_people(_vehicle: &Vehicle, _type: &Type, _status: &Status) {
    println!("--> {} {}", _type.0, _status.0);
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(greet_people.system());
    }
}

