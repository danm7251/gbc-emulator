use std::{path::Path, process::ExitCode};

use crate::rom::RomImage;

mod rom;

const VALIDATE_ROM_FILE: bool = true;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {e}");
            ExitCode::FAILURE
        },
    }
}

fn run() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        anyhow::bail!("Usage: emulator <rom_path>");
    }

    let rom_path = &args[1];

    if VALIDATE_ROM_FILE {
        validate_path(rom_path)?;
    }

    let rom = RomImage::load(rom_path)?;

    print!("{}", rom.header);

    Ok(())
}

fn validate_path(_path: impl AsRef<Path>) -> anyhow::Result<()> {
    Ok(())
}