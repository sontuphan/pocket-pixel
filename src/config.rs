use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
};

pub const CONFIG_FILE: &str = "pocket-pixel.config.json";
const DEFAULT_CONFIG: &str = include_str!("assets/default.config.json");

pub fn read() -> io::Result<String> {
    fs::read_to_string(CONFIG_FILE).map_err(|error| {
        let hint = if error.kind() == io::ErrorKind::NotFound {
            "; run 'pocket-pixel init' first"
        } else {
            ""
        };
        io::Error::new(
            error.kind(),
            format!("cannot read {CONFIG_FILE}: {error}{hint}"),
        )
    })
}

pub fn show() -> io::Result<()> {
    let value: serde_json::Value = serde_json::from_str(&read()?).map_err(|error| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("invalid {CONFIG_FILE}: {error}"),
        )
    })?;
    println!("{value:#}");
    Ok(())
}

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
