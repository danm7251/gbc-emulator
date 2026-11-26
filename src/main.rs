use std::process::ExitCode;

use crate::{cartridge::Cartridge, cpu::Cpu};

mod bus;
mod cartridge;
mod cpu;
mod io_registers;
mod trace;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        },
    }
}

/// Runs the emulator:
/// - Parses the path to the ROM image from arguments.
/// - Cycles the CPU infinitely.
fn run() -> anyhow::Result<()> {
    // Argument parsing.
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        anyhow::bail!("Usage: emulator <rom_path>");
    }

    let rom_path = &args[1];

    trace::init_tracing(false);

    // Load ROM image.
    let cartridge = Cartridge::load(rom_path)?;

    #[cfg(debug_assertions)]
    println!("\n{}", &cartridge.rom.header);

    // Initialise and run CPU.
    let mut cpu = Cpu::init(cartridge);

    #[cfg(debug_assertions)]
    println!("Starting CPU...");

    loop {
        cpu.tick()?;

        #[cfg(debug_assertions)]
        println!();
    }
}