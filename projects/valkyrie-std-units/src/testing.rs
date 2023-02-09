
impl PartialEq<Self> for SIUnit {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length
            && self.mass == other.mass
            && self.time == other.time
            && self.current == other.current
            && self.thermodynamic == other.thermodynamic
            && self.amount == other.amount
            && self.luminous == other.luminous
    }
}


impl Eq for SIUnit {}


impl Display for SIUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.name.is_empty() {
            return f.write_str(&self.name);
        }
        write!(
            f,
            "({}, {}, {}, {}, {}, {}, {})",
            self.length,
            self.mass,
            self.time,
            self.current,
            self.thermodynamic,
            self.amount,
            self.luminous,
        )
    }
}