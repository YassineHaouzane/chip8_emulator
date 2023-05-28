mod constants;
mod renderer;
mod vm;
use std::env;
use vm::*;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    VM::run_rom(&args[1])
}
