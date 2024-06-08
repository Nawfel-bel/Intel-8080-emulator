#[derive(Clone)]
pub enum Registers {
    A, // accumulator
    B,
    C,
    D,
    E,
    H,
    L,
}

impl Registers {
    pub fn next(&self) -> Registers {
        match self {
            Registers::A => Registers::B,
            Registers::B => Registers::C,
            Registers::C => Registers::D,
            Registers::D => Registers::E,
            Registers::E => Registers::H,
            Registers::H => Registers::L,
            Registers::L => panic!("Invalid register index"),
        }
    }
}
