use crate::{SIQuantities, SIUnit};

impl SIUnit<T> {
    pub fn length<T>(name: impl Into<String>, value: T, magnification: T) -> Self {
        SIUnit {
            value: value * magnification,
            name: SIQuantities::length(name.into()),
        }
    }
    pub fn meter<T>(value: T) -> Self {
        SIUnit::length("meter", value, 1.0)
    }
}

impl SIQuantities {
    pub fn length(name: String) -> Self {
        SIQuantities {
            name,
            length: 1,
            mass: 0,
            time: 0,
            current: 0,
            thermodynamic: 0,
            amount: 0,
            luminous: 0,
        }
    }
}