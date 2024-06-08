use core::panic;
use opcodes::Opcodes;
use registers::Registers;
use std::collections::HashMap;
mod opcodes;
mod registers;

#[derive(Hash, Debug, PartialEq, Eq)]
enum ConditionCodes {
    Z,  // when the result == 0
    S,  //sign
    P,  //parity
    CY, // carry
    AC, // auxilliary carry
}

const REGISTERS_COUNT: usize = 7;
const MEMORY_SIZE: usize = 0x10000;

pub struct Cpu {
    registers: [u8; REGISTERS_COUNT],
    sp: u16,
    pc: usize,
    memory: [u8; MEMORY_SIZE],
    cc: HashMap<ConditionCodes, bool>,
}

impl Cpu {
    pub fn new() -> Cpu {
        return Cpu {
            registers: [0; REGISTERS_COUNT],
            sp: 0,
            pc: 0,
            memory: [0; MEMORY_SIZE],
            cc: HashMap::new(),
        };
    }

    pub fn load_rom(&mut self, buffer: &Vec<u8>) {
        self.memory[..buffer.len()].clone_from_slice(&buffer[..]);
    }

    pub fn read(&mut self) {
        for _i in 0..17 {
            let cmd = self.memory[self.pc];
            println!("reading {} at command#: {}", cmd, _i);
            println!("PC: {:04x} SP: {:04x}", self.pc, self.sp);

            let opcode = self.memory[self.pc];
            let operands: [u8; 2] = [self.memory[self.pc + 1], self.memory[self.pc + 2]];
            let opcode = Opcodes::from_hex(opcode);
            let instruction_def = opcode.get_instruction_def();
            self.pc += instruction_def.size as usize;
            // self.to_string();
            match opcode {
                // Data transfer
                Opcodes::MOV_A_A => opcodes::mov_r_r(self, Registers::A, Registers::A),
                Opcodes::MOV_A_B => opcodes::mov_r_r(self, Registers::A, Registers::B),
                Opcodes::MOV_A_C => opcodes::mov_r_r(self, Registers::A, Registers::C),
                Opcodes::MOV_A_D => opcodes::mov_r_r(self, Registers::A, Registers::D),
                Opcodes::MOV_A_E => opcodes::mov_r_r(self, Registers::A, Registers::E),
                Opcodes::MOV_A_H => opcodes::mov_r_r(self, Registers::A, Registers::H),
                Opcodes::MOV_A_L => opcodes::mov_r_r(self, Registers::A, Registers::L),
                Opcodes::MOV_A_M => opcodes::mov_r_m(self, Registers::A),

                Opcodes::MOV_B_A => opcodes::mov_r_r(self, Registers::B, Registers::A),
                Opcodes::MOV_B_B => opcodes::mov_r_r(self, Registers::B, Registers::B),
                Opcodes::MOV_B_C => opcodes::mov_r_r(self, Registers::B, Registers::C),
                Opcodes::MOV_B_D => opcodes::mov_r_r(self, Registers::B, Registers::D),
                Opcodes::MOV_B_E => opcodes::mov_r_r(self, Registers::B, Registers::E),
                Opcodes::MOV_B_H => opcodes::mov_r_r(self, Registers::B, Registers::H),
                Opcodes::MOV_B_L => opcodes::mov_r_r(self, Registers::B, Registers::L),
                Opcodes::MOV_B_M => opcodes::mov_r_m(self, Registers::B),

                Opcodes::MOV_C_A => opcodes::mov_r_r(self, Registers::C, Registers::A),
                Opcodes::MOV_C_B => opcodes::mov_r_r(self, Registers::C, Registers::B),
                Opcodes::MOV_C_C => opcodes::mov_r_r(self, Registers::C, Registers::C),
                Opcodes::MOV_C_D => opcodes::mov_r_r(self, Registers::C, Registers::D),
                Opcodes::MOV_C_E => opcodes::mov_r_r(self, Registers::C, Registers::E),
                Opcodes::MOV_C_H => opcodes::mov_r_r(self, Registers::C, Registers::H),
                Opcodes::MOV_C_L => opcodes::mov_r_r(self, Registers::C, Registers::L),
                Opcodes::MOV_C_M => opcodes::mov_r_m(self, Registers::C),

                Opcodes::MOV_D_A => opcodes::mov_r_r(self, Registers::D, Registers::A),
                Opcodes::MOV_D_B => opcodes::mov_r_r(self, Registers::D, Registers::B),
                Opcodes::MOV_D_C => opcodes::mov_r_r(self, Registers::D, Registers::C),
                Opcodes::MOV_D_D => opcodes::mov_r_r(self, Registers::D, Registers::D),
                Opcodes::MOV_D_E => opcodes::mov_r_r(self, Registers::D, Registers::E),
                Opcodes::MOV_D_H => opcodes::mov_r_r(self, Registers::D, Registers::H),
                Opcodes::MOV_D_L => opcodes::mov_r_r(self, Registers::D, Registers::L),
                Opcodes::MOV_D_M => opcodes::mov_r_m(self, Registers::D),

                Opcodes::MOV_E_A => opcodes::mov_r_r(self, Registers::E, Registers::A),
                Opcodes::MOV_E_B => opcodes::mov_r_r(self, Registers::E, Registers::B),
                Opcodes::MOV_E_C => opcodes::mov_r_r(self, Registers::E, Registers::C),
                Opcodes::MOV_E_D => opcodes::mov_r_r(self, Registers::E, Registers::D),
                Opcodes::MOV_E_E => opcodes::mov_r_r(self, Registers::E, Registers::E),
                Opcodes::MOV_E_H => opcodes::mov_r_r(self, Registers::E, Registers::H),
                Opcodes::MOV_E_L => opcodes::mov_r_r(self, Registers::E, Registers::L),
                Opcodes::MOV_E_M => opcodes::mov_r_m(self, Registers::E),

                Opcodes::MOV_H_A => opcodes::mov_r_r(self, Registers::H, Registers::A),
                Opcodes::MOV_H_B => opcodes::mov_r_r(self, Registers::H, Registers::B),
                Opcodes::MOV_H_C => opcodes::mov_r_r(self, Registers::H, Registers::C),
                Opcodes::MOV_H_D => opcodes::mov_r_r(self, Registers::H, Registers::D),
                Opcodes::MOV_H_E => opcodes::mov_r_r(self, Registers::H, Registers::E),
                Opcodes::MOV_H_H => opcodes::mov_r_r(self, Registers::H, Registers::H),
                Opcodes::MOV_H_L => opcodes::mov_r_r(self, Registers::H, Registers::L),
                Opcodes::MOV_H_M => opcodes::mov_r_m(self, Registers::H),

                Opcodes::MOV_L_A => opcodes::mov_r_r(self, Registers::L, Registers::A),
                Opcodes::MOV_L_B => opcodes::mov_r_r(self, Registers::L, Registers::B),
                Opcodes::MOV_L_C => opcodes::mov_r_r(self, Registers::L, Registers::C),
                Opcodes::MOV_L_D => opcodes::mov_r_r(self, Registers::L, Registers::D),
                Opcodes::MOV_L_E => opcodes::mov_r_r(self, Registers::L, Registers::E),
                Opcodes::MOV_L_H => opcodes::mov_r_r(self, Registers::L, Registers::H),
                Opcodes::MOV_L_L => opcodes::mov_r_r(self, Registers::L, Registers::L),
                Opcodes::MOV_L_M => opcodes::mov_r_m(self, Registers::L),

                Opcodes::MOV_M_A => opcodes::mov_m_r(self, Registers::A),
                Opcodes::MOV_M_B => opcodes::mov_m_r(self, Registers::B),
                Opcodes::MOV_M_C => opcodes::mov_m_r(self, Registers::C),
                Opcodes::MOV_M_D => opcodes::mov_m_r(self, Registers::D),
                Opcodes::MOV_M_E => opcodes::mov_m_r(self, Registers::E),
                Opcodes::MOV_M_H => opcodes::mov_m_r(self, Registers::H),
                Opcodes::MOV_M_L => opcodes::mov_m_r(self, Registers::L),

                Opcodes::MVI_A => opcodes::mvi_r(self, Registers::A, operands[0]),
                Opcodes::MVI_B => opcodes::mvi_r(self, Registers::B, operands[0]),
                Opcodes::MVI_C => opcodes::mvi_r(self, Registers::C, operands[0]),
                Opcodes::MVI_D => opcodes::mvi_r(self, Registers::D, operands[0]),
                Opcodes::MVI_E => opcodes::mvi_r(self, Registers::E, operands[0]),
                Opcodes::MVI_H => opcodes::mvi_r(self, Registers::H, operands[0]),
                Opcodes::MVI_L => opcodes::mvi_r(self, Registers::L, operands[0]),
                Opcodes::MVI_M => opcodes::mvi_m(self, operands[0]),

                Opcodes::LXI_B => opcodes::lxi_r(self, Registers::B, operands),
                Opcodes::LXI_D => opcodes::lxi_r(self, Registers::D, operands),
                Opcodes::LXI_H => opcodes::lxi_r(self, Registers::H, operands),
                Opcodes::LXI_SP => opcodes::lxi_sp(self, operands),

                Opcodes::LDA => opcodes::lda(self, operands),
                Opcodes::STA => opcodes::sta(self, operands),
                Opcodes::LHLD => opcodes::lhld(self, operands),
                Opcodes::SHLD => opcodes::shld(self, operands),

                Opcodes::LDAX_B => opcodes::ldax(self, Registers::B),
                Opcodes::LDAX_D => opcodes::ldax(self, Registers::D),
                Opcodes::STAX_B => opcodes::stax(self, Registers::B),
                Opcodes::STAX_D => opcodes::stax(self, Registers::D),
                Opcodes::XCHG => opcodes::xchg(self),

                // arithmetic
                Opcodes::Add_A => opcodes::add_r(self, Registers::A),
                Opcodes::Add_B => opcodes::add_r(self, Registers::B),
                Opcodes::Add_C => opcodes::add_r(self, Registers::C),
                Opcodes::Add_D => opcodes::add_r(self, Registers::D),
                Opcodes::Add_E => opcodes::add_r(self, Registers::E),
                Opcodes::Add_H => opcodes::add_r(self, Registers::H),
                Opcodes::Add_L => opcodes::add_r(self, Registers::L),
                Opcodes::Add_M => opcodes::add_m(self),

                Opcodes::NOP => opcodes::nop(),
                _ => panic!("Opcode excecution not implemented"),
            }
            // self::opcodes::read_op_code(self);
        }
    }

    fn read_f_reg(&self) -> u8 {
        let psw: u8 = (*self.cc.get(&ConditionCodes::S).unwrap_or(&false) as u8) << 7
            | (*self.cc.get(&ConditionCodes::Z).unwrap_or(&false) as u8) << 6
            | 0 << 5
            | (*self.cc.get(&ConditionCodes::AC).unwrap_or(&false) as u8) << 4
            | 0 << 3
            | (*self.cc.get(&ConditionCodes::P).unwrap_or(&false) as u8) << 2
            | 0 << 1
            | (*self.cc.get(&ConditionCodes::CY).unwrap_or(&false) as u8);
        return psw;
    }

    fn to_string(&mut self) {
        println!(
            "A: {:02x} F: {:02x} B: {:02x} C: {:02x} D: {:02x} E: {:02x} H: {:02x} L: {:02x}",
            self.registers[crate::cpu::Registers::A as usize],
            self.read_f_reg(),
            self.registers[Registers::B as usize],
            self.registers[Registers::C as usize],
            self.registers[Registers::D as usize],
            self.registers[Registers::E as usize],
            self.registers[Registers::H as usize],
            self.registers[Registers::L as usize]
        );
        println!(
            "Z: {} S: {} P: {} CY: {} AC: {}",
            self.cc.get(&ConditionCodes::Z).unwrap_or(&false),
            self.cc.get(&ConditionCodes::S).unwrap_or(&false),
            self.cc.get(&ConditionCodes::P).unwrap_or(&false),
            self.cc.get(&ConditionCodes::CY).unwrap_or(&false),
            self.cc.get(&ConditionCodes::AC).unwrap_or(&false)
        );
        println!("----------------------------------------------------------------");
    }

    fn get_register_pair(&self, r1: Registers, r2: Registers) -> u16 {
        return (self.registers[r1 as usize] as u16) << 8 | self.registers[r2 as usize] as u16;
    }

    fn swap_register_pairs(&mut self, r1: Registers, r2: Registers) {
        let temp = self.registers[r1.clone() as usize];
        self.registers[r1 as usize] = self.registers[r2.clone() as usize];
        self.registers[r2 as usize] = temp;
    }

    fn set_register_pair(&mut self, r1: Registers, r2: Registers, value: u16) {
        self.registers[r1 as usize] = (value >> 8) as u8;
        self.registers[r2 as usize] = (value & 0xff) as u8;
    }
}
