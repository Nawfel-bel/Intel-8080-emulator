use super::{registers, ConditionCodes, State8080};

pub fn readOpcode(state: &mut State8080) {
    let opcode = state.memory[state.pc];
    match opcode {
        0x00 => NOP(state),
        0x01 => LXI(state, registers::B),
        0x02 => STAX(state, registers::B),
        0x03 => INX(state, registers::B),
        0x04 => INR(state, registers::B),
        0x05 => DCR(state, registers::B),
        0x06 => MVI(state, registers::B),
        0x07 => RLC(state),
        0x09 => DAD(state, registers::B),
        0x0a => LDAX(state, registers::B),
        0x0b => DCX(state, registers::B),
        0x0c => INR(state, registers::C),
        0x0d => DCR(state, registers::C),
        0x0e => MVI(state, registers::C),
        0x0f => RRC(state),
        0x1f => RAR(state),
        0x24 => INR(state, registers::H),
        0x2f => CMA(state),
        0x37 => setStateConditionCode(state, ConditionCodes::cy, 1),
        0x3f => setStateConditionCode(state, ConditionCodes::cy, 1 - state.cc[&ConditionCodes::cy]),
        0xb8 => CMP_R(
            state,
            state.registers[registers::A as usize],
            state.registers[registers::B as usize],
        ),
        0xb9 => CMP_R(
            state,
            state.registers[registers::A as usize],
            state.registers[registers::C as usize],
        ),
        0xba => CMP_R(
            state,
            state.registers[registers::A as usize],
            state.registers[registers::D as usize],
        ),
        0xbb => CMP_R(
            state,
            state.registers[registers::A as usize],
            state.registers[registers::E as usize],
        ),
        0xbc => CMP_R(
            state,
            state.registers[registers::A as usize],
            state.registers[registers::H as usize],
        ),
        0xbd => CMP_R(
            state,
            state.registers[registers::A as usize],
            state.registers[registers::L as usize],
        ),
        0xbe => CMP_R(
            state,
            state.registers[registers::A as usize],
            state.memory[state.getRegisterPair(registers::H, registers::L) as usize],
        ),
        0xbf => CMP_R(
            state,
            state.registers[registers::A as usize],
            state.registers[registers::A as usize],
        ),

        0xc2 => JCC(state, ConditionCodes::z, 0), //JNZ
        0xc3 => JMP(state),
        0xc7 => RST_N(state, 0), // RST 0
        0xc9 => RET(state),
        0xca => JCC(state, ConditionCodes::z, 1), // JZ
        0xcd => CALL(state),
        0xcf => RST_N(state, 1),                   // RST 1
        0xd2 => JCC(state, ConditionCodes::cy, 0), // JNC
        0xd7 => RST_N(state, 2),                   // RST 2
        0xda => JCC(state, ConditionCodes::cy, 1), // JC
        0xdf => RST_N(state, 3),                   // RST 3

        0xe2 => JCC(state, ConditionCodes::p, 0), // jpo
        0xe6 => ANI(state),
        0xe7 => RST_N(state, 4), // RST 4

        0xe9 => PCHL(state),
        0xea => JCC(state, ConditionCodes::p, 1), // jpe
        0xef => RST_N(state, 5),                  // RST 5

        0xf2 => JCC(state, ConditionCodes::s, 1), // jp
        0xf7 => RST_N(state, 6),                  // RST 6
        0xfa => JCC(state, ConditionCodes::s, 0), // jm
        0xfe => CPI(state),
        0xff => RST_N(state, 7), // RST 7

        _ => throwUnimplementedInstructionError(state),
    }
}

fn throwUnimplementedInstructionError(State8080: &mut State8080) {
    panic!(
        "Error: Unimplemented instruction: {:02x} at {}",
        State8080.memory[State8080.pc], State8080.pc
    );
}

// 0x00
fn NOP(state: &State8080) {
    return;
}

// 0x01
fn LXI(state: &mut State8080, dest: registers) {
    let result = (state.memory[state.pc + 2] as u16) << 8 | state.memory[state.pc + 1] as u16;
    state.setRegisterPair(dest.clone(), dest.next(), result);
    state.pc += 2;
}

// 0x02
fn STAX(state: &mut State8080, dest: registers) {
    let offset = state.getRegisterPair(dest.clone(), dest.next());
    state.memory[offset as usize] = state.registers[registers::A as usize];
}

// 0x03
fn INX(state: &mut State8080, dest: registers) {
    let mut result = state.getRegisterPair(dest.clone(), dest.clone().next());
    result += 1;
    state.registers[dest.clone() as usize] = (result >> 8) as u8;
    state.registers[dest.clone().next() as usize] = (result & 0xff) as u8;
}

fn JMP(state: &mut State8080) {
    let offset = (state.memory[state.pc + 2] as u16) << 8 | state.memory[state.pc + 1] as u16;
    state.pc = offset as usize;
}

fn CALL(state: &mut State8080) {
    let ret = state.pc + 2;
    state.memory[state.sp as usize - 1] = ((ret >> 8) & 0xff) as u8;
    state.memory[state.sp as usize - 2] = (ret & 0xff) as u8;
    state.sp -= 2;
    JMP(state)
}

fn RET(state: &mut State8080) {
    let offset = (state.memory[state.sp as usize] as u16)
        | (state.memory[state.sp as usize + 1] as u16) << 8;
    state.sp += 2;
    state.pc = offset as usize;
}

fn PCHL(state: &mut State8080) {
    state.pc = state.getRegisterPair(registers::H, registers::L) as usize;
}

fn RST_N(state: &mut State8080, n: u8) {
    state.memory[state.sp as usize - 1] = (state.pc >> 8) as u8;
    state.memory[state.sp as usize - 2] = (state.pc & 0xff) as u8;
    state.sp -= 2;
    state.pc = (n * 8) as usize; //TODO: Check if this is correct
}

fn JCC(state: &mut State8080, condition: ConditionCodes, comp: u8) {
    if state.cc[&condition] == comp {
        JMP(state);
    } else {
        state.pc += 2;
    }
}

fn ANI(state: &mut State8080) {
    let a = state.registers[registers::A as usize];
    let result = a & state.memory[state.pc + 1];
    setStateConditionCode(state, ConditionCodes::z, getZCondition(result));
    setStateConditionCode(state, ConditionCodes::s, getSCondition(result));
    setStateConditionCode(state, ConditionCodes::p, getPCondition(result));
    setStateConditionCode(state, ConditionCodes::ac, getACCondition(result));
    setStateConditionCode(state, ConditionCodes::cy, 0);
    state.registers[registers::A as usize] = result;
    state.pc += 1;
}

// 0x04 ,0x0c
fn INR(state: &mut State8080, dest: registers) {
    let dest_index = dest as usize;
    let result: u16 = state.registers[dest_index] as u16 + 1;
    setStateConditionCode(state, ConditionCodes::z, getZCondition(result as u8));
    setStateConditionCode(state, ConditionCodes::s, getSCondition(result as u8));
    setStateConditionCode(state, ConditionCodes::p, getPCondition(result as u8));
    setStateConditionCode(state, ConditionCodes::ac, getACCondition(result as u8));
    state.registers[dest_index] = (result & 0xff) as u8;
}

fn CMA(state: &mut State8080) {
    state.registers[registers::A as usize] = !state.registers[registers::A as usize];
}

//0x05 , 0x0d
fn DCR(state: &mut State8080, dest: registers) {
    let dest_index = dest as usize;
    let result = state.registers[dest_index] - 1;
    setStateConditionCode(state, ConditionCodes::z, getZCondition(result));
    setStateConditionCode(state, ConditionCodes::s, getSCondition(result));
    setStateConditionCode(state, ConditionCodes::p, getPCondition(result));
    setStateConditionCode(state, ConditionCodes::ac, getACCondition(result));
    state.registers[dest_index] = (result & 0xff) as u8;
}

// 0x06 , 0x0e
fn MVI(state: &mut State8080, dest: registers) {
    let dest_index = dest as usize;
    state.registers[dest_index] = state.memory[state.pc + 1];
    state.pc += 1;
}

// 0x07
fn RLC(state: &mut State8080) {
    let a = state.registers[registers::A as usize];
    let result = a.rotate_left(1);
    setStateConditionCode(state, ConditionCodes::cy, result & 1);
    state.registers[registers::A as usize] = (result & 0xff) as u8;
}

//0x0f
fn RRC(state: &mut State8080) {
    let a = state.registers[registers::A as usize];
    let result = a.rotate_right(1);
    setStateConditionCode(state, ConditionCodes::cy, (result >> 7) & 1);
    state.registers[registers::A as usize] = (result & 0xff) as u8;
}

fn CMP_R(state: &mut State8080, a: u8, b: u8) {
    let result = a - b;
    setStateConditionCode(state, ConditionCodes::z, getZCondition(result));
    setStateConditionCode(state, ConditionCodes::s, getSCondition(result));
    setStateConditionCode(state, ConditionCodes::p, getPCondition(result));
    setStateConditionCode(state, ConditionCodes::ac, getACCondition(result));
    setStateConditionCode(state, ConditionCodes::cy, (a < b).into());
}

fn CPI(state: &mut State8080) {
    let a = state.registers[registers::A as usize];
    let b = state.memory[state.pc + 1];
    let result = a - b;
    setStateConditionCode(state, ConditionCodes::z, getZCondition(result));
    setStateConditionCode(state, ConditionCodes::s, getSCondition(result));
    setStateConditionCode(state, ConditionCodes::p, getPCondition(result));
    setStateConditionCode(state, ConditionCodes::ac, getACCondition(result));
    setStateConditionCode(state, ConditionCodes::cy, (a < b).into());
    state.pc += 1;
}

fn RAR(state: &mut State8080) {
    let a = state.registers[registers::A as usize];
    setStateConditionCode(state, ConditionCodes::cy, a & 1);
    let result = (a >> 1) | (a & 0x80);
    state.registers[registers::A as usize] = (result & 0xff) as u8;
}

//0x09
fn DAD(state: &mut State8080, src: registers) {
    let result = state.getRegisterPair(registers::H, registers::L) as u32
        + state.getRegisterPair(src.clone(), src.next()) as u32;
    setStateConditionCode(state, ConditionCodes::cy, (result > 0xffff).into());
    state.setRegisterPair(registers::H, registers::L, result as u16);
}

// 0x0a
fn LDAX(state: &mut State8080, src: registers) {
    let offset = state.getRegisterPair(src.clone(), src.next());
    state.registers[registers::A as usize] = state.memory[offset as usize];
}

// 0x0b
fn DCX(state: &mut State8080, dest: registers) {
    let mut result = state.getRegisterPair(dest.clone(), dest.clone().next());
    result -= 1;
    state.setRegisterPair(dest.clone(), dest.next(), result);
}

fn setStateConditionCode(state: &mut State8080, code: ConditionCodes, val: u8) {
    state.cc.entry(code).and_modify(|v| *v = val).or_insert(val);
}

fn getZCondition(val: u8) -> u8 {
    return ((val & 0xff) == 0).into();
}

fn getSCondition(val: u8) -> u8 {
    return ((val & 0x80) != 0).into();
}

fn getPCondition(val: u8) -> u8 {
    if val % 2 == 0 {
        1
    } else {
        0
    }
}

fn getACCondition(val: u8) -> u8 {
    return ((val & 0xf) > 0xf).into();
}
