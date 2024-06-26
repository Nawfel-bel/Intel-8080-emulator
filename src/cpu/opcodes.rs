use core::fmt;
use std::{any::Any, ffi::c_void, mem};

use super::{registers, ConditionCodes, Cpu, Registers};

const MAX_OPERANDS: usize = 2;

pub struct InstructionDef {
    cycles: u8,
    pub size: u8,
}

impl fmt::Display for Opcodes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[allow(nonstandard_style)]
#[derive(Debug)]
pub enum Opcodes {
    NOP,
    INX_B,
    INX_D,
    INX_H,
    INX_SP,
    INR_B,
    INR_D,
    INR_E,
    INR_H,
    INR_L,
    INR_M,
    INR_A,
    DCR_A,
    DCR_B,
    DCR_C,
    DCR_D,
    DCR_E,
    DCR_H,
    DCR_L,
    DCR_M,

    RLC,
    DAD_B,
    DAD_D,
    DAD_H,
    DAD_SP,
    DCX_B,
    DCX_D,
    DCX_H,
    DCX_SP,
    INR_C,
    RRC,
    RAR,
    CMA,
    STC,
    CMC,ORI,
    MOV_M_A,
    MOV_A_A,
    MOV_A_B,
    MOV_A_C,
    MOV_A_D,
    MOV_A_E,
    MOV_A_H,
    MOV_A_L,
    MOV_A_M,
    MOV_B_A,
    MOV_B_B,
    MOV_B_C,
    MOV_B_D,
    MOV_B_E,
    MOV_B_H,
    MOV_B_L,
    MOV_B_M,
    MOV_C_A,
    MOV_C_B,
    MOV_C_C,
    MOV_C_D,
    MOV_C_E,
    MOV_C_H,
    MOV_C_L,
    MOV_C_M,
    MOV_D_A,
    MOV_D_B,
    MOV_D_C,
    MOV_D_D,
    MOV_D_E,
    MOV_D_H,
    MOV_D_L,
    MOV_D_M,
    MOV_E_A,
    MOV_E_B,
    MOV_E_C,
    MOV_E_D,
    MOV_E_E,
    MOV_E_H,
    MOV_E_L,
    MOV_E_M,
    MOV_H_A,
    MOV_H_B,
    MOV_H_C,
    MOV_H_D,
    MOV_H_E,
    MOV_H_H,
    MOV_H_L,
    MOV_H_M,
    MOV_L_B,
    MOV_L_C,
    MOV_L_D,
    MOV_L_E,
    MOV_L_H,
    MOV_L_L,
    MOV_L_M,
    MOV_L_A,
    MOV_M_B,
    MOV_M_C,
    MOV_M_D,
    MOV_M_E,
    MOV_M_H,
    MOV_M_L,
    MVI_A,
    MVI_B,
    MVI_C,
    MVI_D,
    MVI_E,
    MVI_H,
    MVI_L,
    MVI_M,
    LXI_B,
    LXI_D,
    LXI_H,
    LXI_SP,
    ORA_B,
    ORA_C,
    ORA_D,
    ORA_E,
    ORA_H,
    ORA_L,
    ORA_M,
    ORA_A,
    LDA,
    ACI,
    XRI,
    STA,
    LHLD,
    SHLD,
    LDAX_B,
    LDAX_D,
    STAX_B,
    STAX_D,
    XRA_B,
    XRA_C,
    XRA_D,
    XRA_E,
    XRA_H,
    XRA_L,
    XRA_M,
    XRA_A,
    XCHG,
    ADD_A,
    ADD_B,
    ADD_C,
    ADD_D,
    ADD_E,
    ADD_H,
    ADD_L,
    ADD_M,
    ADC_B,
    ADC_C,
    ADC_D,
    ADC_E,
    ADC_H,
    ADC_L,
    ADC_M,
    ADC_A,
    SUB_B,
    SUB_C,
    SUB_D,
    SUB_E,
    SUB_H,
    SUB_L,
    SUB_M,
    SUB_A,
    SBB_B,
    SBB_C,
    SBB_D,
    SBB_E,
    SBB_H,
    SBB_L,
    SBB_M,
    SBB_A,
    CMP_B,
    CMP_C,
    CMP_D,
    CMP_E,
    CMP_H,
    CMP_L,
    CMP_M,
    CMP_A,
    POP_B,
    PUSH_B,
    ANA_B,
    ANA_C,
    ANA_D,
    ANA_E,
    ANA_H,
    ANA_L,
    ANA_M,
    ANA_A,
    JNZ,
    JMP,
    RAL,
    RST_0,
    RET,
    JZ,
    CALL,
    RST_1,
    JNC,
    RST_2,
    JC,
    RST_3,
    JPO,
    XTHL,
    ANI,
    RST_4,
    PCHL,
    JPE,
    RST_5,
    POP_PSW,
    JP,
    PUSH_PSW,
    RST_6,
    SPHL,
    JM,
    CPI,
    SUI,
    DAA,
    RST_7,
    CNZ,
    CZ,
    CNC,
    CC,
    CPO,
    CPE,
    CP,
    CM,
    RNZ,
    RZ,
    RNC,
    RC,
    RPO,
    RPE,
    RP,
    RM,
    PUSH_D,
    PUSH_H,
    POP_D,
    POP_H,
}
impl Opcodes {
    #[rustfmt::skip]
    pub fn from_hex(opcode: u8) -> Opcodes {
        match opcode {
            0x00 => Opcodes::NOP,
            0x01 => Opcodes::LXI_B,
            0x02 => Opcodes::STAX_B,
            0x03 => Opcodes::INX_B,
            0x04 => Opcodes::INR_B,
            0x05 => Opcodes::DCR_B,
            0x06 => Opcodes::MVI_B,
            0x07 => Opcodes::RLC,
            0x09 => Opcodes::DAD_B,
            0x0a => Opcodes::LDAX_B,
            0x0b => Opcodes::DCX_B,
            0x0c => Opcodes::INR_C,
            0x0d => Opcodes::DCR_C,
            0x0e => Opcodes::MVI_C,
            0x0f => Opcodes::RRC,
            0x11 => Opcodes::LXI_D,
            0x12 => Opcodes::STAX_D,
            0x13 => Opcodes::INX_D,
            0x14 => Opcodes::INR_D,
            0x15 => Opcodes::DCR_D,
            0x16 => Opcodes::MVI_D,
            0x17 => Opcodes::RAL,
            0x19 => Opcodes::DAD_D,
            0x1a => Opcodes::LDAX_D,
            0x1b => Opcodes::DCX_D,
            0x1c => Opcodes::INR_E,
            0x1d => Opcodes::DCR_E,
            0x1e => Opcodes::MVI_E,
            0x1f => Opcodes::RAR,
            0x21 => Opcodes::LXI_H,
            0x22 => Opcodes::SHLD,
            0x23 => Opcodes::INX_H,
            0x24 => Opcodes::INR_H,
            0x25 => Opcodes::DCR_H,
            0x26 => Opcodes::MVI_H,
            0x27 => Opcodes::DAA,
            0x29 => Opcodes::DAD_H,
            0x2a => Opcodes::LHLD,
            0x2b => Opcodes::DCX_H,
            0x2c => Opcodes::INR_L,
            0x2d => Opcodes::DCR_L,
            0x2e => Opcodes::MVI_L,
            0x2f => Opcodes::CMA,
            0x31 => Opcodes::LXI_SP,
            0x32 => Opcodes::STA,
            0x33 => Opcodes::INX_SP,
            0x34 => Opcodes::INR_M,
            0x35 => Opcodes::DCR_M,
            0x36 => Opcodes::MVI_M,
            0x37 => Opcodes::STC,
            0x39 => Opcodes::DAD_SP,
            0x3a => Opcodes::LDA,
            0x3b => Opcodes::DCX_SP,
            0x3c => Opcodes::INR_A,
            0x3d => Opcodes::DCR_A,
            0x3e => Opcodes::MVI_A,
            0x3f => Opcodes::CMC,
            0x40 => Opcodes::MOV_B_B,
            0x41 => Opcodes::MOV_B_C,
            0x42 => Opcodes::MOV_B_D,
            0x43 => Opcodes::MOV_B_E,
            0x44 => Opcodes::MOV_B_H,
            0x45 => Opcodes::MOV_B_L,
            0x46 => Opcodes::MOV_B_M,
            0x47 => Opcodes::MOV_B_A,
            0x48 => Opcodes::MOV_C_B,
            0x49 => Opcodes::MOV_C_C,
            0x4a => Opcodes::MOV_C_D,
            0x4b => Opcodes::MOV_C_E,
            0x4c => Opcodes::MOV_C_H,
            0x4d => Opcodes::MOV_C_L,
            0x4e => Opcodes::MOV_C_M,
            0x4f => Opcodes::MOV_C_A,
            0x50 => Opcodes::MOV_D_B,
            0x51 => Opcodes::MOV_D_C,
            0x52 => Opcodes::MOV_D_D,
            0x53 => Opcodes::MOV_D_E,
            0x54 => Opcodes::MOV_D_H,
            0x55 => Opcodes::MOV_D_L,
            0x56 => Opcodes::MOV_D_M,
            0x57 => Opcodes::MOV_D_A,
            0x58 => Opcodes::MOV_E_B,
            0x59 => Opcodes::MOV_E_C,
            0x5a => Opcodes::MOV_E_D,
            0x5b => Opcodes::MOV_E_E,
            0x5c => Opcodes::MOV_E_H,
            0x5d => Opcodes::MOV_E_L,
            0x5e => Opcodes::MOV_E_M,
            0x5f => Opcodes::MOV_E_A,
            0x60 => Opcodes::MOV_H_B,
            0x61 => Opcodes::MOV_H_C,
            0x62 => Opcodes::MOV_H_D,
            0x63 => Opcodes::MOV_H_E,
            0x64 => Opcodes::MOV_H_H,
            0x65 => Opcodes::MOV_H_L,
            0x66 => Opcodes::MOV_H_M,
            0x67 => Opcodes::MOV_H_A,
            0x68 => Opcodes::MOV_L_B,
            0x69 => Opcodes::MOV_L_C,
            0x6a => Opcodes::MOV_L_D,
            0x6b => Opcodes::MOV_L_E,
            0x6c => Opcodes::MOV_L_H,
            0x6d => Opcodes::MOV_L_L,
            0x6e => Opcodes::MOV_L_M,
            0x6f => Opcodes::MOV_L_A,
            0x70 => Opcodes::MOV_M_B,
            0x71 => Opcodes::MOV_M_C,
            0x72 => Opcodes::MOV_M_D,
            0x73 => Opcodes::MOV_M_E,
            0x74 => Opcodes::MOV_M_H,
            0x75 => Opcodes::MOV_M_L,
            // theres a halt here
            0x77 => Opcodes::MOV_M_A,
            0x78 => Opcodes::MOV_A_B,
            0x79 => Opcodes::MOV_A_C,
            0x7a => Opcodes::MOV_A_D,
            0x7b => Opcodes::MOV_A_E,
            0x7c => Opcodes::MOV_A_H,
            0x7d => Opcodes::MOV_A_L,
            0x7e => Opcodes::MOV_A_M,
            0x7f => Opcodes::MOV_A_A,

            0x80 => Opcodes::ADD_B,
            0x81 => Opcodes::ADD_C,
            0x82 => Opcodes::ADD_D,
            0x83 => Opcodes::ADD_E,
            0x84 => Opcodes::ADD_H,
            0x85 => Opcodes::ADD_L,
            0x86 => Opcodes::ADD_M,
            0x87 => Opcodes::ADD_A,

            0x88 => Opcodes::ADC_B,
            0x89 => Opcodes::ADC_C,
            0x8a => Opcodes::ADC_D,
            0x8b => Opcodes::ADC_E,
            0x8c => Opcodes::ADC_H,
            0x8d => Opcodes::ADC_L,
            0x8e => Opcodes::ADC_M,
            0x8f => Opcodes::ADC_A,
            
            0x90 => Opcodes::SUB_B,
            0x91 => Opcodes::SUB_C,
            0x92 => Opcodes::SUB_D,
            0x93 => Opcodes::SUB_E,
            0x94 => Opcodes::SUB_H,
            0x95 => Opcodes::SUB_L,
            0x96 => Opcodes::SUB_M,
            0x97 => Opcodes::SUB_A,

            0x98 => Opcodes::SBB_B,
            0x99 => Opcodes::SBB_C,
            0x9a => Opcodes::SBB_D,
            0x9b => Opcodes::SBB_E,
            0x9c => Opcodes::SBB_H,
            0x9d => Opcodes::SBB_L,
            0x9e => Opcodes::SBB_M,
            0x9f => Opcodes::SBB_A,
    
            0xa0 => Opcodes::ANA_B,
            0xa1 => Opcodes::ANA_C,
            0xa2 => Opcodes::ANA_D,
            0xa3 => Opcodes::ANA_E,
            0xa4 => Opcodes::ANA_H,
            0xa5 => Opcodes::ANA_L,
            0xa6 => Opcodes::ANA_M,
            0xa7 => Opcodes::ANA_A,
            0xa8 => Opcodes::XRA_B,
            0xa9 => Opcodes::XRA_C,
            0xaa => Opcodes::XRA_D,
            0xab => Opcodes::XRA_E,
            0xac => Opcodes::XRA_H,
            0xad => Opcodes::XRA_L,
            0xae => Opcodes::XRA_M,
            0xaf => Opcodes::XRA_A,
            0xb0 => Opcodes::ORA_B,
            0xb1 => Opcodes::ORA_C,
            0xb2 => Opcodes::ORA_D,
            0xb3 => Opcodes::ORA_E,
            0xb4 => Opcodes::ORA_H,
            0xb5 => Opcodes::ORA_L,
            0xb6 => Opcodes::ORA_M,
            0xb7 => Opcodes::ORA_A,

            0xb8 => Opcodes::CMP_B,
            0xb9 => Opcodes::CMP_C,
            0xba => Opcodes::CMP_D,
            0xbb => Opcodes::CMP_E,
            0xbc => Opcodes::CMP_H,
            0xbd => Opcodes::CMP_L,
            0xbe => Opcodes::CMP_M,
            0xbf => Opcodes::CMP_A,
            0xc0 => Opcodes::RNZ,
            0xc1 => Opcodes::POP_B,
            0xc2 => Opcodes::JNZ, 
            0xc3 => Opcodes::JMP,
            0xc4 => Opcodes::CNZ,
            0xc5 => Opcodes::PUSH_B,
            0xc7 => Opcodes::RST_0, 
            0xc8 => Opcodes::RZ,
            0xc9 => Opcodes::RET,
            0xca => Opcodes::JZ, 
            0xcc => Opcodes::CZ,
            0xcd => Opcodes::CALL,
            0xce => Opcodes::ACI,
            0xcf => Opcodes::RST_1,     
            0xd0 => Opcodes::RNC,   
            0xd1 => Opcodes::POP_D,       
            0xd2 => Opcodes::JNC, 
            0xd4 => Opcodes::CNC,
            0xd5 => Opcodes::PUSH_D,
            0xd6 => Opcodes::SUI,
            0xd7 => Opcodes::RST_2, 
            0xd8 => Opcodes::RC,                      
            0xda => Opcodes::JC,  
            0xdc => Opcodes::CC,
            0xdf => Opcodes::RST_3,                    
            0xe0 => Opcodes::RPO,
            0xe1 => Opcodes::POP_H,
            0xe2 => Opcodes::JPO, 
            0xe3 => Opcodes::XTHL,
            0xe4 => Opcodes::CPO,
            0xe5 => Opcodes::PUSH_H,
            0xe6 => Opcodes::ANI,
            0xe7 => Opcodes::RST_4, 
            0xe8 => Opcodes::RPE,

            0xe9 => Opcodes::PCHL,
            0xea => Opcodes::JPE, 
            0xeb => Opcodes::XCHG,
            0xec => Opcodes::CPE,
            0xee => Opcodes::XRI,
            0xef => Opcodes::RST_5,  
            0xf0 => Opcodes::RP,                 
            0xf1 => Opcodes::POP_PSW,
            0xf2 => Opcodes::JP, 
            0xf4 => Opcodes::CP,
            0xf5 => Opcodes::PUSH_PSW,
            0xf6 => Opcodes::ORI,
            0xf7 => Opcodes::RST_6, 
            0xf8 => Opcodes::RM,
            0xf9 => Opcodes::SPHL,
            0xfa => Opcodes::JM,
            0xfc => Opcodes::CM,
            0xfe => Opcodes::CPI,
            0xff => Opcodes::RST_7, 
            _ => panic!("[from_hex]: Unknown opcode with hex origin: 0x{:02X}", opcode),
        }
    }

    // pub fn get_instruction_def(&self) -> InstructionDef {
    //     match self {
    //         //LXI
    //         Opcodes::LXI_B | Opcodes::LXI_D | Opcodes::LXI_H | Opcodes::LXI_SP => InstructionDef { cycles: 10, size: 3 },
    //         Opcodes::NOP => InstructionDef { cycles : 4, size : 1 },
    //         Opcodes::STAX_B => InstructionDef { cycles : 7, size : 1 },
    //           Opcodes::MOV_A_A | Opcodes::MOV_A_B | Opcodes::MOV_A_C | Opcodes::MOV_A_D | Opcodes::MOV_A_E | Opcodes::MOV_A_H | Opcodes::MOV_A_L | Opcodes::MOV_A_M 
    //         | Opcodes::MOV_B_A | Opcodes::MOV_B_B | Opcodes::MOV_B_C | Opcodes::MOV_B_D | Opcodes::MOV_B_E | Opcodes::MOV_B_H | Opcodes::MOV_B_L | Opcodes::MOV_B_M
    //         | Opcodes::MOV_C_A | Opcodes::MOV_C_B | Opcodes::MOV_C_C | Opcodes::MOV_C_D | Opcodes::MOV_C_E | Opcodes::MOV_C_H | Opcodes::MOV_C_L | Opcodes::MOV_C_M
    //         | Opcodes::MOV_D_A | Opcodes::MOV_D_B | Opcodes::MOV_D_C | Opcodes::MOV_D_D | Opcodes::MOV_D_E | Opcodes::MOV_D_H | Opcodes::MOV_D_L | Opcodes::MOV_D_M
    //         | Opcodes::MOV_E_A | Opcodes::MOV_E_B | Opcodes::MOV_E_C | Opcodes::MOV_E_D | Opcodes::MOV_E_E | Opcodes::MOV_E_H | Opcodes::MOV_E_L | Opcodes::MOV_E_M
    //         | Opcodes::MOV_H_A | Opcodes::MOV_H_B | Opcodes::MOV_H_C | Opcodes::MOV_H_D | Opcodes::MOV_H_E | Opcodes::MOV_H_H | Opcodes::MOV_H_L | Opcodes::MOV_H_M
    //         | Opcodes::MOV_L_A | Opcodes::MOV_L_B | Opcodes::MOV_L_C | Opcodes::MOV_L_D | Opcodes::MOV_L_E | Opcodes::MOV_L_H | Opcodes::MOV_L_L | Opcodes::MOV_L_M
    //         | Opcodes::MOV_M_A | Opcodes::MOV_M_B | Opcodes::MOV_M_C | Opcodes::MOV_M_D | Opcodes::MOV_M_E | Opcodes::MOV_M_H | Opcodes::MOV_M_L 
    //         => InstructionDef { cycles : 5, size : 1 },
    //         Opcodes::INX_B | Opcodes::INX_D | Opcodes::INX_H => InstructionDef { cycles : 5, size : 1 },

    //         _ => panic!("[get_instruction_def]: Instruction not defined for opcode of type: {:?}", self),
    //     }
    // }

    pub fn get_instruction_def(&self) -> InstructionDef {
        match self {
            // NOP
            Opcodes::NOP => InstructionDef { cycles: 4, size: 1 },
    
            // LXI
            Opcodes::LXI_B | Opcodes::LXI_D | Opcodes::LXI_H | Opcodes::LXI_SP => InstructionDef { cycles: 10, size: 3 },
    
            // STAX
            Opcodes::STAX_B | Opcodes::STAX_D => InstructionDef { cycles: 7, size: 1 },
    
            // INX
            Opcodes::INX_B | Opcodes::INX_D | Opcodes::INX_H | Opcodes::INX_SP => InstructionDef { cycles: 5, size: 1 },
    
            // INR
            Opcodes::INR_A | Opcodes::INR_B | Opcodes::INR_C | Opcodes::INR_D | Opcodes::INR_E | Opcodes::INR_H | Opcodes::INR_L | Opcodes::INR_M => 
                InstructionDef { cycles: 5, size: 1 },
    
            // DCR
            Opcodes::DCR_A | Opcodes::DCR_B | Opcodes::DCR_C | Opcodes::DCR_D | Opcodes::DCR_E | Opcodes::DCR_H | Opcodes::DCR_L | Opcodes::DCR_M => 
                InstructionDef { cycles: 5, size: 1 },
    
            // MVI
            Opcodes::MVI_A | Opcodes::MVI_B | Opcodes::MVI_C | Opcodes::MVI_D | Opcodes::MVI_E | Opcodes::MVI_H | Opcodes::MVI_L | Opcodes::MVI_M => 
                InstructionDef { cycles: 7, size: 2 },
    
            // RLC, RRC, RAL, RAR
            Opcodes::RLC | Opcodes::RRC | Opcodes::RAL | Opcodes::RAR => InstructionDef { cycles: 4, size: 1 },
    
            // DAD
            Opcodes::DAD_B | Opcodes::DAD_D | Opcodes::DAD_H | Opcodes::DAD_SP => InstructionDef { cycles: 10, size: 1 },
    
            // LDAX
            Opcodes::LDAX_B | Opcodes::LDAX_D => InstructionDef { cycles: 7, size: 1 },
    
            // DCX
            Opcodes::DCX_B | Opcodes::DCX_D | Opcodes::DCX_H | Opcodes::DCX_SP => InstructionDef { cycles: 5, size: 1 },
    
            // MOV
            Opcodes::MOV_A_A | Opcodes::MOV_A_B | Opcodes::MOV_A_C | Opcodes::MOV_A_D | Opcodes::MOV_A_E | Opcodes::MOV_A_H | Opcodes::MOV_A_L | Opcodes::MOV_A_M |
            Opcodes::MOV_B_A | Opcodes::MOV_B_B | Opcodes::MOV_B_C | Opcodes::MOV_B_D | Opcodes::MOV_B_E | Opcodes::MOV_B_H | Opcodes::MOV_B_L | Opcodes::MOV_B_M |
            Opcodes::MOV_C_A | Opcodes::MOV_C_B | Opcodes::MOV_C_C | Opcodes::MOV_C_D | Opcodes::MOV_C_E | Opcodes::MOV_C_H | Opcodes::MOV_C_L | Opcodes::MOV_C_M |
            Opcodes::MOV_D_A | Opcodes::MOV_D_B | Opcodes::MOV_D_C | Opcodes::MOV_D_D | Opcodes::MOV_D_E | Opcodes::MOV_D_H | Opcodes::MOV_D_L | Opcodes::MOV_D_M |
            Opcodes::MOV_E_A | Opcodes::MOV_E_B | Opcodes::MOV_E_C | Opcodes::MOV_E_D | Opcodes::MOV_E_E | Opcodes::MOV_E_H | Opcodes::MOV_E_L | Opcodes::MOV_E_M |
            Opcodes::MOV_H_A | Opcodes::MOV_H_B | Opcodes::MOV_H_C | Opcodes::MOV_H_D | Opcodes::MOV_H_E | Opcodes::MOV_H_H | Opcodes::MOV_H_L | Opcodes::MOV_H_M |
            Opcodes::MOV_L_A | Opcodes::MOV_L_B | Opcodes::MOV_L_C | Opcodes::MOV_L_D | Opcodes::MOV_L_E | Opcodes::MOV_L_H | Opcodes::MOV_L_L | Opcodes::MOV_L_M |
            Opcodes::MOV_M_A | Opcodes::MOV_M_B | Opcodes::MOV_M_C | Opcodes::MOV_M_D | Opcodes::MOV_M_E | Opcodes::MOV_M_H | Opcodes::MOV_M_L => 
                InstructionDef { cycles: 7, size: 1 },
    
            // ADD
            Opcodes::ADD_A | Opcodes::ADD_B | Opcodes::ADD_C | Opcodes::ADD_D | Opcodes::ADD_E | Opcodes::ADD_H | Opcodes::ADD_L | Opcodes::ADD_M => 
                InstructionDef { cycles: 4, size: 1 },
    
            // ADC
            Opcodes::ADC_A | Opcodes::ADC_B | Opcodes::ADC_C | Opcodes::ADC_D | Opcodes::ADC_E | Opcodes::ADC_H | Opcodes::ADC_L | Opcodes::ADC_M => 
                InstructionDef { cycles: 4, size: 1 },
    
            // SUB
            Opcodes::SUB_A | Opcodes::SUB_B | Opcodes::SUB_C | Opcodes::SUB_D | Opcodes::SUB_E | Opcodes::SUB_H | Opcodes::SUB_L | Opcodes::SUB_M => 
                InstructionDef { cycles: 4, size: 1 },
    
            // SBB
            Opcodes::SBB_A | Opcodes::SBB_B | Opcodes::SBB_C | Opcodes::SBB_D | Opcodes::SBB_E | Opcodes::SBB_H | Opcodes::SBB_L | Opcodes::SBB_M => 
                InstructionDef { cycles: 4, size: 1 },
    
            // ANA
            Opcodes::ANA_A | Opcodes::ANA_B | Opcodes::ANA_C | Opcodes::ANA_D | Opcodes::ANA_E | Opcodes::ANA_H | Opcodes::ANA_L | Opcodes::ANA_M => 
                InstructionDef { cycles: 4, size: 1 },
    
            // XRA
            Opcodes::XRA_A | Opcodes::XRA_B | Opcodes::XRA_C | Opcodes::XRA_D | Opcodes::XRA_E | Opcodes::XRA_H | Opcodes::XRA_L | Opcodes::XRA_M => 
                InstructionDef { cycles: 4, size: 1 },
    
            // ORA
            Opcodes::ORA_A | Opcodes::ORA_B | Opcodes::ORA_C | Opcodes::ORA_D | Opcodes::ORA_E | Opcodes::ORA_H | Opcodes::ORA_L | Opcodes::ORA_M => 
                InstructionDef { cycles: 4, size: 1 },
    
            // CMP
            Opcodes::CMP_A | Opcodes::CMP_B | Opcodes::CMP_C | Opcodes::CMP_D | Opcodes::CMP_E | Opcodes::CMP_H | Opcodes::CMP_L | Opcodes::CMP_M => 
                InstructionDef { cycles: 4, size: 1 },
    
            // JMP
            Opcodes::JMP | Opcodes::JC | Opcodes::JNC | Opcodes::JZ | Opcodes::JNZ | Opcodes::JM | Opcodes::JP | Opcodes::JPE | Opcodes::JPO => 
                InstructionDef { cycles: 10, size: 3 },
    
            // CALL
            Opcodes::CALL | Opcodes::CC | Opcodes::CNC | Opcodes::CZ | Opcodes::CNZ | Opcodes::CM | Opcodes::CP | Opcodes::CPE | Opcodes::CPO => 
                InstructionDef { cycles: 17, size: 3 },
    
            // RET
            Opcodes::RET | Opcodes::RC | Opcodes::RNC | Opcodes::RZ | Opcodes::RNZ | Opcodes::RM | Opcodes::RP | Opcodes::RPE | Opcodes::RPO => 
                InstructionDef { cycles: 10, size: 1 },
    
            // RST
            Opcodes::RST_0 | Opcodes::RST_1 | Opcodes::RST_2 | Opcodes::RST_3 | Opcodes::RST_4 | Opcodes::RST_5 | Opcodes::RST_6 | Opcodes::RST_7 => 
                InstructionDef { cycles: 11, size: 1 },
    
            // PCHL
            Opcodes::PCHL => InstructionDef { cycles: 5, size: 1 },
    
            // POP
            Opcodes::POP_B | Opcodes::POP_D | Opcodes::POP_H | Opcodes::POP_PSW => InstructionDef { cycles: 10, size: 1 },
    
            // PUSH
            Opcodes::PUSH_B | Opcodes::PUSH_D | Opcodes::PUSH_H | Opcodes::PUSH_PSW => InstructionDef { cycles: 11, size: 1 },
    
            // XCHG
            Opcodes::XCHG => InstructionDef { cycles: 4, size: 1 },
    
            // XTHL
            Opcodes::XTHL => InstructionDef { cycles: 18, size: 1 },
    
            // SPHL
            Opcodes::SPHL => InstructionDef { cycles: 5, size: 1 },
    
            // EI
            // Opcodes::EI => InstructionDef { cycles: 4, size: 1 },
    
            // DI
            // Opcodes::DI => InstructionDef { cycles: 4, size: 1 },
    
            // HLT
            // Opcodes::HLT => InstructionDef { cycles: 7, size: 1 },
    
            // CMA
            Opcodes::CMA => InstructionDef { cycles: 4, size: 1 },
    
            // CMC
            Opcodes::CMC => InstructionDef { cycles: 4, size: 1 },
    
            // STC
            Opcodes::STC => InstructionDef { cycles: 4, size: 1 },
    
            // DAA
            Opcodes::DAA => InstructionDef { cycles: 4, size: 1 },
        
            _ => panic!("[get_instruction_def]: Instruction not defined for opcode of type: {:?}", self),
        }
    }
}


pub fn throw_unimplemented_instruction_error(state8080: &mut Cpu) {
    panic!(
        "Error: Unimplemented instruction: {:02x} at {}",
        state8080.memory[state8080.pc], state8080.pc
    );
}

pub fn nop() {
    return;
}


// data transfer 
pub fn lxi_r(state: &mut Cpu, dest: Registers, operands: [u8; MAX_OPERANDS]) {
    let result = (operands[1] as u16) << 8 | operands[0] as u16;
    state.set_register_pair(dest.clone(), dest.next(), result);
}

pub fn lxi_sp(state: &mut Cpu, operands: [u8; MAX_OPERANDS]) {
    let result = (operands[1] as u16) << 8 | operands[0] as u16;
    state.sp = result;
}

pub fn mov_r_r(state: &mut Cpu,dest : Registers,  src: Registers) {
    state.registers[dest as usize] = state.registers[src as usize];
}

pub fn mov_r_m(state: &mut Cpu, dest:Registers){
    let offset = state.get_register_pair(Registers::H, Registers::L);
    state.registers[dest as usize] = state.memory[offset as usize];
}

pub fn mov_m_r(state: &mut Cpu, src: Registers) {
    let offset = state.get_register_pair(Registers::H, Registers::L);
    state.memory[offset as usize] = state.registers[src as usize];
}

pub fn mvi_r(state: &mut Cpu, dest: Registers, operand: u8) {
    state.registers[dest as usize] = operand;
}

pub fn mvi_m(state: &mut Cpu, operand: u8){
    let offset = state.get_register_pair(Registers::H, Registers::L);
    state.memory[offset as usize] = operand;
}

pub fn lda (state: &mut Cpu, operands: [u8; MAX_OPERANDS]){
    let offset = (operands[1] as u16) << 8 | operands[0] as u16;
    state.registers[Registers::A as usize] = state.memory[offset as usize];
}

pub fn sta(state: &mut Cpu, operands: [u8; MAX_OPERANDS]){
    let offset = (operands[1] as u16) << 8 | operands[0] as u16;
    state.memory[offset as usize] = state.registers[Registers::A as usize];
}

pub fn shld (state: &mut Cpu, operands: [u8; MAX_OPERANDS]){
    let offset = (operands[1] as u16) << 8 | operands[0] as u16;
    state.memory[offset as usize] = state.registers[Registers::L as usize];
    state.memory[offset.wrapping_add(1) as usize] = state.registers[Registers::H as usize];
}

pub fn lhld(state: &mut Cpu, operands: [u8; MAX_OPERANDS]) {
    let offset = operands[1].rotate_left(8) as u16 | operands[0] as u16;
    state.registers[Registers::L as usize] = state.memory[offset as usize];
    state.registers[Registers::H as usize] = state.memory[offset.wrapping_add(1) as usize]
}

pub fn ldax(state: &mut Cpu, src: Registers) {
    let offset = state.get_register_pair(src.clone(), src.next());
    state.registers[Registers::A as usize] = state.memory[offset as usize];
}

pub fn stax(state: &mut Cpu, dest: Registers) {
    let offset = state.get_register_pair(dest.clone(), dest.next());
    state.memory[offset as usize] = state.registers[Registers::A as usize];
}

pub fn xchg (state: &mut Cpu) {
    state.swap_register_pairs(Registers::H, Registers::D);
    state.swap_register_pairs(Registers::L, Registers::E);
}

// --------------------

// arithmetic
pub fn add_r(state: &mut Cpu, dest:Registers){
    let val = state.registers[Registers::A as usize];
    update_conditions_add(state, val, state.registers[dest.clone() as usize], false);
    let result = val.wrapping_add(state.registers[dest as usize]);
    state.registers[Registers::A as usize] = result;

}

pub fn add_m(state: &mut Cpu){
    let offset = state.get_register_pair(Registers::H, Registers::L);

    let val = state.registers[Registers::A as usize];
    let val2 = state.memory[offset as usize];
    update_conditions_add(state, val, val2, false);
    let result = val.wrapping_add(val2);
    state.registers[Registers::A as usize] = result;
}

pub fn adc_r(state: &mut Cpu, dest:Registers){
    let val = state.registers[Registers::A as usize];
    update_conditions_add(state, val, state.registers[dest.clone() as usize], true);
    let carry = state.cc[&ConditionCodes::CY] as u8;
    let result = state.registers[dest as usize].wrapping_add(carry);
    state.registers[Registers::A as usize] = val.wrapping_add(result);
}

pub fn adc_m(state: &mut Cpu){
    let val = state.registers[Registers::A as usize];
    let offset = state.get_register_pair(Registers::H, Registers::L);
    let val2 = state.memory[offset as usize];
    update_conditions_add(state, val, val2, true);
    let carry = state.cc[&ConditionCodes::CY] as u8;
    let result = val2.wrapping_add(carry);
    state.registers[Registers::A as usize] = val.wrapping_add(result);
}

pub fn aci(state: &mut Cpu, operand: u8){
    let val = state.registers[Registers::A as usize];
    update_conditions_add(state, val, operand, true);
    let carry = state.cc[&ConditionCodes::CY] as u8;
    let result = operand.wrapping_add(carry);
    state.registers[Registers::A as usize] = val.wrapping_add(result);
}

pub fn sub_r(state: &mut Cpu, dest:Registers){
    let val = state.registers[Registers::A as usize];
    update_conditions_sub(state, val, state.registers[dest.clone() as usize], false);
    let result = val.wrapping_sub(state.registers[dest as usize]);
    state.registers[Registers::A as usize] = result;
}

pub fn sub_m(state: &mut Cpu){
    let val = state.registers[Registers::A as usize];
    let offset = state.get_register_pair(Registers::H, Registers::L);
    let val2 = state.memory[offset as usize];
    update_conditions_sub(state, val, val2, false);
    let result = val.wrapping_sub(val2);
    state.registers[Registers::A as usize] = result;
}

pub fn sui(state: &mut Cpu, operand: u8){
    let val = state.registers[Registers::A as usize];
    update_conditions_sub(state, val, operand, false);
    let result = val.wrapping_sub(operand);
    state.registers[Registers::A as usize] = result;
}

pub fn sbb_r(state: &mut Cpu, dest:Registers){
    let val = state.registers[Registers::A as usize];
    update_conditions_sub(state, val, state.registers[dest.clone() as usize], true);
    let carry = state.cc[&ConditionCodes::CY] as u8;
    let result = state.registers[dest as usize].wrapping_add(carry);
    state.registers[Registers::A as usize] = val.wrapping_sub(result);
}

pub fn sbb_m(state: &mut Cpu){
    let val = state.registers[Registers::A as usize];
    let offset = state.get_register_pair(Registers::H, Registers::L);
    let val2 = state.memory[offset as usize];
    update_conditions_sub(state, val, val2, true);
    let carry = state.cc[&ConditionCodes::CY] as u8;
    let result = val2.wrapping_add(carry);
    state.registers[Registers::A as usize] = val.wrapping_sub(result);
}

pub fn inr_r (state: &mut Cpu, dest:Registers){
    let val = state.registers[dest.clone() as usize];
    update_conditions_inc(state, val);
    state.registers[dest as usize] = val.wrapping_add(1);
}

pub fn inr_m(state: &mut Cpu){
    let offset = state.get_register_pair(Registers::H, Registers::L);
    let val = state.memory[offset as usize];
    update_conditions_inc(state, val);
    state.memory[offset as usize] = val.wrapping_add(1);
}

pub fn dcr_r(state: &mut Cpu, dest:Registers){
    let val = state.registers[dest.clone() as usize];
    update_conditions_dcr(state, val);
    state.registers[dest as usize] = val.wrapping_sub(1);
}

pub fn dcr_m(state: &mut Cpu){
    let offset = state.get_register_pair(Registers::H, Registers::L);
    let val = state.memory[offset as usize];
    update_conditions_dcr(state, val);
    state.memory[offset as usize] = val.wrapping_sub(1);
}

pub fn inx_rp(state: &mut Cpu, dest: Registers) {
    let mut result = state.get_register_pair(dest.clone(), dest.clone().next());
    result = result.wrapping_add(1);
    state.set_register_pair(dest.clone(), dest.clone().next(), result);
}

pub fn inx_sp (state: &mut Cpu){
    state.sp = state.sp.wrapping_add(1);
}

pub fn dcx_rp(state: &mut Cpu, dest: Registers) {
    let mut result = state.get_register_pair(dest.clone(), dest.clone().next());
    result = result.wrapping_sub(1);
    state.set_register_pair(dest.clone(), dest.clone().next(), result);
}

pub fn dcx_sp (state: &mut Cpu){
    state.sp = state.sp.wrapping_sub(1);
}

pub fn dad_rp(state: &mut Cpu, dest: Registers) {
    let result = (state.get_register_pair(Registers::H, Registers::L) as u32)
    .wrapping_add((state.get_register_pair(dest.clone(), dest.clone().next()) as u32));
    set_state_condition_code(state, ConditionCodes::CY, result > 0xffff);
    
    state.set_register_pair(Registers::H, Registers::L, result as u16);
}

pub fn dad_sp(state: &mut Cpu) {
    let result = (state.get_register_pair(Registers::H, Registers::L) as u32)
    .wrapping_add(state.sp as u32);
    set_state_condition_code(state, ConditionCodes::CY, result > 0xffff);
    
    state.set_register_pair(Registers::H, Registers::L, result as u16);
}

pub fn daa (state: &mut Cpu){
 let mut val : u8 = 0;
 let msb = state.registers[Registers::A as usize] >> 4;
 let lsb = state.registers[Registers::A as usize] & 0xf;

    if state.cc[&ConditionCodes::AC] || lsb > 9 {
        val = val.wrapping_add(0x06);
    }

    if state.cc[&ConditionCodes::CY] || msb > 9 || (msb >= 9 && lsb > 9) {
        val = val.wrapping_add(0x60);
        set_state_condition_code(state, ConditionCodes::CY, true);
    }

    let result = state.registers[Registers::A as usize].wrapping_add(val);
    set_z_condition(state, result);
    set_p_condition(state, result);
    set_s_condition(state, result);
    set_ac_condition_add(state, state.registers[Registers::A as usize], val, 0);
    state.registers[Registers::A as usize] = result;

}

pub fn ana_r(state: &mut Cpu, dest: Registers){
    let val = state.registers[Registers::A as usize];
    let val2 = state.registers[dest as usize];
    let result = val & val2;
    update_conditions_and(state, val, val2);
    state.registers[Registers::A as usize] = result;
}

pub fn ana_m (state: &mut Cpu){
    let val = state.registers[Registers::A as usize];
    let offset = state.get_register_pair(Registers::H, Registers::L);
    let val2 = state.memory[offset as usize];
    update_conditions_and(state, val, val2);
    let result = val & val2;
    state.registers[Registers::A as usize] = result;
}

pub fn ani (state: &mut Cpu, operand: u8){
    let val = state.registers[Registers::A as usize];
    let result = val & operand;
    update_conditions_and(state, val, operand);
    state.registers[Registers::A as usize] = result;
}

pub fn xra_r (state: &mut Cpu, dest: Registers){
    let val = state.registers[Registers::A as usize];
    let val2 = state.registers[dest as usize];
    let result = val ^ val2;
    update_conditions_or(state, result);
    state.registers[Registers::A as usize] = result;
}

pub fn xra_m (state: &mut Cpu){
    let val = state.registers[Registers::A as usize];
    let offset = state.get_register_pair(Registers::H, Registers::L);
    let val2 = state.memory[offset as usize];
    let result = val ^ val2;
    update_conditions_or(state, result);
    state.registers[Registers::A as usize] = result;
}

pub fn xri (state: &mut Cpu, operand: u8){
    let val = state.registers[Registers::A as usize];
    let result = val ^ operand;
    update_conditions_or(state, result);
    state.registers[Registers::A as usize] = result;
}

pub fn ora_r (state: &mut Cpu, dest: Registers){
    let val = state.registers[Registers::A as usize];
    let val2 = state.registers[dest as usize];
    let result = val | val2;
    update_conditions_or(state, result);
    state.registers[Registers::A as usize] = result;
}

pub fn ora_m (state: &mut Cpu){
    let val = state.registers[Registers::A as usize];
    let offset = state.get_register_pair(Registers::H, Registers::L);
    let val2 = state.memory[offset as usize];
    let result = val | val2;
    update_conditions_or(state, result);
    state.registers[Registers::A as usize] = result;
}

pub fn ori (state: &mut Cpu, operand: u8){
    let val = state.registers[Registers::A as usize];
    let result = val | operand;
    update_conditions_or(state, result);
    state.registers[Registers::A as usize] = result;
}

pub fn cmp_r (state: &mut Cpu, register: Registers){
    update_conditions_cmp(state, state.registers[Registers::A as usize], state.registers[register as usize]);
}

pub fn cmp_m (state: &mut Cpu){
    let offset = state.get_register_pair(Registers::H, Registers::L);
    update_conditions_cmp(state, state.registers[Registers::A as usize], state.memory[offset as usize]);
}

pub fn cpi (state: &mut Cpu, operand: u8){
    update_conditions_cmp(state, state.registers[Registers::A as usize], operand);
}

pub fn rlc (state: &mut Cpu){
    let val = state.registers[Registers::A as usize];
    let result = val.rotate_left(1);
    set_state_condition_code(state, ConditionCodes::CY, val & 0x80 != 0);
    state.registers[Registers::A as usize] = result;
}

pub fn rrc(state: &mut Cpu) {
    let val = state.registers[Registers::A as usize];
    let result = val.rotate_right(1);
    set_state_condition_code(state, ConditionCodes::CY, val & 0x01 != 0);
    state.registers[Registers::A as usize] = result;
}

pub fn ral(state: &mut Cpu) {
    let val = state.registers[Registers::A as usize];
    let result = val.rotate_left(1);
    let carry = state.cc[&ConditionCodes::CY] as u8;
    set_state_condition_code(state, ConditionCodes::CY, val & 0x80 != 0);
    state.registers[Registers::A as usize] = result.wrapping_add(carry);
}

pub fn rar(state: &mut Cpu) {
    let val = state.registers[Registers::A as usize];
    let mut result = val.rotate_right(1);
    let carry = state.cc[&ConditionCodes::CY];
    set_state_condition_code(state, ConditionCodes::CY, val & 0x01 != 0);

    if carry {
        result |= 0x80;
    } else {
        result &= 0x7f;
    }
    state.registers[Registers::A as usize] = result;
}

pub fn cma(state: &mut Cpu) {
    state.registers[Registers::A as usize] = !state.registers[Registers::A as usize];
}

pub fn cmc(state: &mut Cpu) {
    let carry = state.cc[&ConditionCodes::CY];
    set_state_condition_code(state, ConditionCodes::CY, !carry);
}

pub fn stc(state: &mut Cpu) {
    set_state_condition_code(state, ConditionCodes::CY, true);
}

pub fn jmp(state: &mut Cpu, operands: [u8; MAX_OPERANDS]) {
    let offset = (operands[1] as u16) << 8 | operands[0] as u16;
    state.pc = offset as usize;
}

pub fn jcc(state: &mut Cpu, condition: ConditionCodes, comp: bool, operands: [u8; MAX_OPERANDS]) {
    if state.cc[&condition] == comp {
        jmp(state, operands);
    }
}

pub fn call (state: &mut Cpu, operands: [u8; MAX_OPERANDS]){
    state.memory[state.sp as usize - 1] = (state.pc >> 8) as u8;
    state.memory[state.sp as usize - 2] = (state.pc & 0xff) as u8;
    state.sp -= 2;
    jmp(state, operands);
}

pub fn ccc (state: &mut Cpu, condition: ConditionCodes, comp: bool, operands: [u8; MAX_OPERANDS]){
    if state.cc[&condition] == comp {
        call(state, operands);
        // TODO: INCREMENT CYCLES HERE
    }
}

pub fn ret (state: &mut Cpu){
    let offset = (state.memory[state.sp as usize] as u16) | ((state.memory[state.sp as usize + 1] as u16) << 8);
    state.sp += 2;
    state.pc = offset as usize;
}

pub fn rcc (state: &mut Cpu, condition: ConditionCodes, comp: bool){
    if state.cc[&condition] == comp {
        ret(state);
        // TODO: INCREMENT CYCLES HERE
    }
}

pub fn rst_n(state: &mut Cpu, n: u8){
    state.memory[state.sp as usize - 1] = (state.pc >> 8) as u8;
    state.memory[state.sp as usize - 2] = (state.pc & 0xff) as u8;
    state.sp -= 2;
    state.pc = (n * 8) as usize;
}

pub fn pchl (state: &mut Cpu){
    let offset = state.get_register_pair(Registers::H, Registers::L);
    state.pc = offset as usize;
}

pub fn push_rp (state: &mut Cpu, src: Registers){
    state.memory[state.sp as usize - 1] = state.registers[src.clone() as usize];
    state.memory[state.sp as usize - 1] = state.registers[src.next() as usize];
    state.sp -= 2;
}

pub fn push_psw (state: &mut Cpu){
    state.memory[state.sp as usize - 1] = state.registers[Registers::A as usize];
    let psw = state_get_sw(state);
    state.memory[state.sp as usize - 2] = psw;
    state.sp -= 2;
}

pub fn pop_rp(state: &mut Cpu, src: Registers){
    state.registers[src.clone().next() as usize] = state.memory[state.sp as usize];
    state.registers[src as usize] = state.memory[state.sp as usize + 1];
    state.sp += 2;
}

pub fn pop_psw(state: &mut Cpu){
    let psw = state.memory[state.sp as usize];
    set_state_condition_code(state, ConditionCodes::CY, (psw & 1) != 0);
    set_state_condition_code(state, ConditionCodes::P, (psw & (1<<2))!=0);
    set_state_condition_code(state, ConditionCodes::AC, (psw & (1<<4))!=0);
    set_state_condition_code(state, ConditionCodes::Z, (psw & (1<<6))!=0);
    set_state_condition_code(state, ConditionCodes::S, (psw & (1<<7))!=0);
    state.registers[Registers::A as usize] = state.memory[state.sp as usize + 1];
    state.sp += 2;
}

pub fn xthl(state: &mut Cpu){
    let val = &mut state.registers[Registers::L as usize];
    let val2 = &mut state.memory[state.sp as usize];
    mem::swap(val, val2);
    let val = &mut state.registers[Registers::H as usize];
    let val2 = &mut state.memory[state.sp as usize  + 1];
    mem::swap(val, val2);
}

pub fn sphl(state: &mut Cpu){
    state.sp = state.get_register_pair(Registers::H, Registers::L);
}



fn state_get_sw(state: &mut Cpu) -> u8{
    let psw = 
    (state.cc[&ConditionCodes::S] as u8) << 7
    | (state.cc[&ConditionCodes::Z] as u8) << 6
    | (state.cc[&ConditionCodes::AC] as u8) << 4
    | (state.cc[&ConditionCodes::P] as u8) << 2
    | (1<<1)
    | (state.cc[&ConditionCodes::CY] as u8);
    return psw;
}

fn update_conditions_add(state: &mut Cpu, val1: u8, val2:u8, carry : bool){
    let car = (carry && state.cc[&ConditionCodes::CY]) as u8;
    let res = val1.wrapping_add(val2).wrapping_add(car);

    set_z_condition(state, res);
    set_p_condition(state, res);
    set_s_condition(state, res);
    set_ac_condition_add(state, val1, val2, car);
    set_cy_condition_add(state, val1, val2, car)
}

fn update_conditions_sub(state: &mut Cpu, val1: u8, val2:u8, carry : bool){
    let car = (carry && state.cc[&ConditionCodes::CY]) as u8;
    let res = val1.wrapping_sub(val2).wrapping_sub(car);

    set_z_condition(state, res);
    set_p_condition(state, res);
    set_s_condition(state, res);
    set_ac_condition_sub(state, val1, val2, car==1);
    set_cy_condition_sub(state, val1, val2, car==1)
}

fn update_conditions_inc(state: &mut Cpu, val: u8){
    let res = val.wrapping_add(1);
    set_z_condition(state, res);
    set_p_condition(state, res);
    set_s_condition(state, res);
    set_ac_condition_add(state, val, 1, 0);
}

fn update_conditions_dcr(state: &mut Cpu, val: u8){
    let res = val.wrapping_sub(1);
    set_z_condition(state, res);
    set_p_condition(state, res);
    set_s_condition(state, res);
    set_ac_condition_add(state, val, 1, 0);
}

fn update_conditions_and(state: &mut Cpu, val1: u8, val2:u8){
    set_z_condition(state, val1 & val2);
    set_p_condition(state, val1 & val2);
    set_s_condition(state, val1 & val2);
    set_state_condition_code(state, ConditionCodes::CY, false);
    set_state_condition_code(state, ConditionCodes::AC, ((val1 | val2) >> 3) & 1 > 0);
}

fn update_conditions_or(state: &mut Cpu, res: u8){
    set_z_condition(state, res);
    set_p_condition(state, res);
    set_s_condition(state, res);
    clear_cy_ac_conditions(state);
}

fn update_conditions_cmp (state: &mut Cpu, val1: u8, val2: u8){
    let res = val1.wrapping_sub(val2);
    set_z_condition(state, res);
    set_p_condition(state, res);
    set_s_condition(state, res);
    set_ac_condition_sub(state, val1, val2, false);
    set_cy_condition_sub(state, val1, val2, false);
}


fn set_state_condition_code(state: &mut Cpu, code: ConditionCodes, val: bool) {
    state.cc.entry(code).and_modify(|v| *v = val).or_insert(val);
}

fn set_z_condition(state: &mut Cpu, val: u8) {
    return set_state_condition_code(state, ConditionCodes::Z, val == 0x00);
}

fn set_s_condition(state: &mut Cpu, val: u8) {
    return set_state_condition_code(state, ConditionCodes::S, val & 0x80 != 0);
}

fn set_p_condition(state: &mut Cpu, val: u8) {
    return set_state_condition_code(state, ConditionCodes::P, val % 2 == 0);
}

fn clear_cy_ac_conditions(state: &mut Cpu) {
    set_state_condition_code(state, ConditionCodes::CY, false);
    set_state_condition_code(state, ConditionCodes::AC, false);
}

fn set_ac_condition_add(state: &mut Cpu, val1: u8, val2: u8, add_carry : u8) {
    let carry = (add_carry == 1 && state.cc[&ConditionCodes::CY]) as u8;
    let ac_val = (val1 & 0xf) + (val2 & 0xf) + carry > 0xf;
    return set_state_condition_code(state, ConditionCodes::AC, ac_val);
}

fn set_cy_condition_add(state: &mut Cpu, val1: u8, val2: u8, add_carry : u8) {
    let carry = (add_carry == 1 && state.cc[&ConditionCodes::CY]) as u8;
    let result = val1.wrapping_add(val2).wrapping_add(add_carry);
    return set_state_condition_code(state, ConditionCodes::CY, result > 0xff);
}

fn set_ac_condition_sub(state: &mut Cpu, val1: u8, val2: u8, sub_carry : bool) {
    let carry = (sub_carry && state.cc[&ConditionCodes::CY]) as u8;
    let ac_val = (val1 & 0xf).wrapping_add(!val2 & 0xf).wrapping_add(!carry);
    return set_state_condition_code(state, ConditionCodes::AC, ac_val>0xf);
}

fn set_cy_condition_sub(state: &mut Cpu, val1: u8, val2: u8, sub_carry : bool) {
    let carry = (sub_carry && state.cc[&ConditionCodes::CY]) as u8;
    let result = val1.wrapping_add(!val2).wrapping_add(!sub_carry as u8) ;
    return set_state_condition_code(state, ConditionCodes::CY, result > 0xff);
}


// fn get_s_condition(val: u8) -> bool {
//     return (val & 0x80) != 0;
// }

// fn get_p_condition(val: u8) -> bool {
//     if val % 2 == 0 {
//         true
//     } else {
//         false
//     }
// }

// fn get_ac_condition_add(val1: u8, val2: u8) -> bool {
//     let lower_nibble_overflow = (val1 & 0xf) + (val2 & 0xf);
//     return lower_nibble_overflow > 0xf;
// }

// fn get_ac_condition_sub(val1: u8, val2: u8) -> bool {
//     let lower_nibble_underflow = (val1 & 0xf) < (val2 & 0xf);
//     return lower_nibble_underflow;
// }
