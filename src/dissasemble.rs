fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let file_path = &args[1];
    println!("File path: {}", file_path);
    let mut _file: File = match File::open(&file_path) {
        Ok(file) => file,
        Err(error) => {
            panic!("Error: {}", error);
        }
    };

    let mut buffer = Vec::new();
    _file.read_to_end(&mut buffer);
    let mut pc: usize = 0;
    let mut data: String = String::new();
    while pc < buffer.len() {
        let strr = Dissasemble(&buffer, &mut pc);
        data.push_str(&strr);
    }
    fs::write("./outputCodes", data);
}

fn Dissasemble(buffer: &Vec<u8>, pc: &mut usize) -> String {
    let op = &buffer[*pc];
    let mut opbytes = 1;
    let mut line = String::new();
    match op {
        0x00 => line = "NOP".to_string(),
        0x01 => {
            line = format!("LXI    B,#${:02x}{:02x}", buffer[*pc + 2], buffer[*pc + 1]);
            opbytes = 3;
        }
        0x02 => line = "STAX   B".to_string(),
        0x03 => line = "INX    B".to_string(),
        0x04 => line = "INR    B".to_string(),
        0x05 => line = "DCR    B".to_string(),
        0x06 => {
            line = format!("MVI    B,#${:02x}", buffer[*pc + 1]);
            opbytes = 2;
        }
        0x07 => line = "RLC".to_string(),
        0x08 => line = "NOP".to_string(),
        0x3e => {
            line = format!("MVI    A,#0x{:02x}", buffer[*pc + 1]);
            opbytes = 2;
        }
        0xc3 => {
            line = format!("JMP    ${:02x}{:02x}", buffer[*pc + 2], buffer[*pc + 1]);
            opbytes = 3;
        }
        _ => line = "Unknown opcode".to_string(),
    }
    let line = format!("{:04x} {}\n", pc, line);
    *pc += opbytes;
    return line;
}
