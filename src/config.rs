use std::{
    fs::OpenOptions,
    io::{self, Write},
};

pub const CONFIG_FILE: &str = "pocket-pixel.config.json";
const DEFAULT_CONFIG: &str = include_str!("default.config.json");

pub fn init() -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(CONFIG_FILE)
        .map_err(|error| {
            if error.kind() == io::ErrorKind::AlreadyExists {
                io::Error::new(
                    error.kind(),
                    format!("{CONFIG_FILE} already exists; left unchanged"),
                )
            } else {
                io::Error::new(
                    error.kind(),
                    format!("cannot create {CONFIG_FILE}: {error}"),
                )
            }
        })?;
    file.write_all(DEFAULT_CONFIG.as_bytes())?;
    println!("Created {CONFIG_FILE} (EDG32 palette, 32x32 tiles)");
    Ok(())
}
