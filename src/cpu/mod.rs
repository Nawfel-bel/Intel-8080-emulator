use std::collections::HashMap;

mod opcodes;

#[derive(Clone)]
enum registers {
    A, // accumulator
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Hash, Debug, PartialEq, Eq)]
enum ConditionCodes {
    z,  // when the result == 0
    s,  //sign
    p,  //parity
    cy, // carry
    ac, // auxilliary carry
}

impl registers {
    fn from_usize(value: usize) -> registers {
        match value {
            0 => registers::A,
            1 => registers::B,
            2 => registers::C,
            3 => registers::D,
            4 => registers::E,
            5 => registers::H,
            6 => registers::L,
            _ => panic!("Invalid register index: {}", value),
        }
    }

    fn next(&self) -> registers {
        match self {
            registers::A => registers::B,
            registers::B => registers::C,
            registers::C => registers::D,
            registers::D => registers::E,
            registers::E => registers::H,
            registers::H => registers::L,
            registers::L => panic!("Invalid register index"),
        }
    }
}

const REGISTERS_COUNT: usize = 7;
const MEMORY_SIZE: usize = 0x10000;

pub struct State8080 {
    registers: [u8; REGISTERS_COUNT],
    sp: u16,
    pc: usize,
    memory: [u8; MEMORY_SIZE],
    cc: HashMap<ConditionCodes, u8>,
    int_enable: u8,
}

impl State8080 {
    pub fn new() -> State8080 {
        return State8080 {
            registers: [0; REGISTERS_COUNT],
            sp: 0,
            pc: 0,
            memory: [0; MEMORY_SIZE],
            cc: HashMap::new(),
            int_enable: 0,
        };
    }

    pub fn read(&mut self) {
        let cmd = self.memory[self.pc];
        println!("reading {}", cmd);
        for i in 0..20 {
            self::opcodes::readOpcode(self);
            self.pc += 1;
        }
    }

    pub fn loadROM(&mut self, buffer: &Vec<u8>) {
        self.memory[..buffer.len()].clone_from_slice(&buffer[..]);
    }

    fn getRegisterPair(&self, r1: registers, r2: registers) -> u16 {
        return (self.registers[r1 as usize] as u16) << 8 | self.registers[r2 as usize] as u16;
    }

    fn setRegisterPair(&mut self, r1: registers, r2: registers, value: u16) {
        self.registers[r1 as usize] = (value >> 8) as u8;
        self.registers[r2 as usize] = (value & 0xff) as u8;
    }
}
