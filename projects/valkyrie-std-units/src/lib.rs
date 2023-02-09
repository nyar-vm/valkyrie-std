use std::fmt::Display;

mod te;

pub struct SIQuantities<T> {
    pub value: T,
    pub name: SIUnit,
}

// length, mass, time, electric current, thermodynamic temperature, amount of substance, and luminous intensity
pub struct SIUnit {
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
