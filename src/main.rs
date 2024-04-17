use std::{env, fs, str, vec};
use std::{fs::File, io::Read};

mod cpu;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut state = cpu::State8080::new();

    let args = std::env::args().collect::<Vec<String>>();
    let file_path = &args[1];
    println!("reading file path: {}", file_path);
    let mut _file: File = match File::open(&file_path) {
        Ok(file) => file,
        Err(error) => {
            panic!("Error: {}", error);
        }
    };

    let mut buffer = Vec::new();
    _file.read_to_end(&mut buffer);
    state.loadROM(&buffer);
    state.read();
}
