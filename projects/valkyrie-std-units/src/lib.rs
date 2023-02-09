use std::fmt::Display;

mod builtin;

pub struct SIUnit<T> {
    pub value: T,
    pub name: SIQuantities,
}

// length, mass, time, electric current, thermodynamic temperature, amount of substance, and luminous intensity
pub struct SIQuantities {
    // common name
    pub name: String,
    //
    pub length: i8,
    //
    pub mass: i8,
    pub time: i8,
    // electric current
    pub current: i8,
    // thermodynamic temperature
    pub thermodynamic: i8,
    // amount of substance
    pub amount: i8,
    // luminous intensity
    pub luminous: i8,
}



use uom::si::f32::*;
use uom::si::length::kilometer;
use uom::si::time::second;

fn main() {
    let length = Length::new::<kilometer>(5.0);
    let time = Time::new::<second>(15.0);
    let velocity/*: Velocity*/ = length / time;
    let _acceleration = calc_acceleration(velocity, time);
    //let error = length + time; // error[E0308]: mismatched types
}

fn calc_acceleration(velocity: Velocity, time: Time) -> Acceleration {
    velocity / time
}