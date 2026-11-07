use crate::config::CONFIG_FILE;
use serde::Deserialize;
use std::{
    env, fs,
    io::{self, IsTerminal},
};

#[derive(Deserialize)]
struct Config {
    palette: Palette,
    map: Mapping,
}

#[derive(Deserialize)]
struct Palette {
    name: String,
    colors: Vec<String>,
    #[serde(default)]
    normals: Vec<String>,
}

#[derive(Deserialize)]
struct Mapping {
    colors: Vec<String>,
    #[serde(default)]
    normals: Vec<String>,
}

fn invalid(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}

fn rgb(hex: &str) -> io::Result<[u8; 3]> {
    let digits = hex.strip_prefix('#').unwrap_or("");
    if digits.len() != 6 || !digits.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(invalid(format!("invalid color {hex:?}; expected #RRGGBB")));
    }
    Ok([
        u8::from_str_radix(&digits[0..2], 16).unwrap(),
        u8::from_str_radix(&digits[2..4], 16).unwrap(),
        u8::from_str_radix(&digits[4..6], 16).unwrap(),
    ])
}

fn validate(colors: &[String], symbols: &[String], section: &str) -> io::Result<()> {
    if colors.len() != symbols.len() {
        return Err(invalid(format!(
            "palette.{section} and map.{section} must have equal lengths"
        )));
    }
    for (index, (color, symbol)) in colors.iter().zip(symbols).enumerate() {
        rgb(color)?;
        if symbol.chars().count() != 1 || symbol.chars().any(char::is_control) {
            return Err(invalid(format!(
                "map.{section}[{index}] must be one printable character"
            )));
        }
    }
    Ok(())
}

fn print_section(title: &str, colors: &[String], symbols: &[String], colored: bool) {
    println!("\n{title} ({})", colors.len());
    println!("Index  Char  Color");
    for (index, (hex, symbol)) in colors.iter().zip(symbols).enumerate() {
        print!("{index:>5}  {symbol:^4}  {hex}");
        if colored {
            let [r, g, b] = rgb(hex).expect("colors validated before printing");
            print!("  \x1b[48;2;{r};{g};{b}m    \x1b[0m");
        }
        println!();
    }
}

pub fn show() -> io::Result<()> {
    let text = fs::read_to_string(CONFIG_FILE).map_err(|error| {
        let hint = if error.kind() == io::ErrorKind::NotFound {
            "; run 'pocket-pixel init' first"
        } else {
            ""
        };
        io::Error::new(
            error.kind(),
            format!("cannot read {CONFIG_FILE}: {error}{hint}"),
        )
    })?;
    let config: Config = serde_json::from_str(&text)
        .map_err(|error| invalid(format!("invalid {CONFIG_FILE}: {error}")))?;
    validate(&config.palette.colors, &config.map.colors, "colors")?;
    validate(&config.palette.normals, &config.map.normals, "normals")?;
    let colored = io::stdout().is_terminal()
        && env::var_os("NO_COLOR").is_none()
        && env::var("TERM").as_deref() != Ok("dumb");
    println!("Palette: {}", config.palette.name.escape_debug());
    print_section(
        "Colors",
        &config.palette.colors,
        &config.map.colors,
        colored,
    );
    if !config.palette.normals.is_empty() {
        print_section(
            "Normals",
            &config.palette.normals,
            &config.map.normals,
            colored,
        );
    }
    Ok(())
}
