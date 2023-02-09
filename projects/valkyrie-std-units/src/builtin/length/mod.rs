use crate::{SIQuantities, SIUnit};

impl SIUnit<T> {
    pub fn length<T>(name: impl Into<String>, value: T, magnification: T) -> Self {
        SIUnit {
            value: value * magnification,
            name: SIQuantities::length(name.into()),
        }
    }
    // km
    pub fn kilometer<T>(value: T) -> Self {
        SIUnit::length("kilometer", value, 1000.0)
    }
    // m
    pub fn meter<T>(value: T) -> Self {
        SIUnit::length("meter", value, 1.0)
    }
    // cm
    pub fn centimeter<T>(value: T) -> Self {
        SIUnit::length("centimeter", value, 0.01)
    }
    // mm
    pub fn millimeter<T>(value: T) -> Self {
        SIUnit::length("millimeter", value, 0.001)
    }
    // μm
    pub fn micrometer<T>(value: T) -> Self {
        SIUnit::length("micrometer", value, 0.000001)
    }
    // nm
    pub fn nanometer<T>(value: T) -> Self {
        SIUnit::length("nanometer", value, 0.000000001)
    }
    // pm
    pub fn picometer<T>(value: T) -> Self {
        SIUnit::length("picometer", value, 0.000000000001)
    }
    // fm
    pub fn femtometer<T>(value: T) -> Self {
        SIUnit::length("femtometer", value, 0.000000000000001)
    }
    // am
    pub fn attometer<T>(value: T) -> Self {
        SIUnit::length("attometer", value, 0.000000000000000001)
    }
    // zm
    pub fn zeptometer<T>(value: T) -> Self {
        SIUnit::length("zeptometer", value, 0.000000000000000000001)
    }
    // ym
    pub fn yoctometer<T>(value: T) -> Self {
        SIUnit::length("yoctometer", value, 0.000000000000000000000001)
    }
}

impl SIUnit<T> {

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