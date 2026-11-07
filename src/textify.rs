use crate::{config, generate::Artifact, palette};
use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::{self, Write},
    path::Path,
};

fn invalid(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}

fn entries(
    colors: &[String],
    symbols: &[String],
    section: &str,
) -> io::Result<Vec<([u8; 3], char)>> {
    palette::validate(colors, symbols, section)?;
    if colors.is_empty() {
        return Err(invalid(format!("palette.{section} must not be empty")));
    }
    colors
        .iter()
        .zip(symbols)
        .map(|(color, symbol)| Ok((palette::rgb(color)?, symbol.chars().next().unwrap())))
        .collect()
}

fn nearest(rgb: [u8; 3], entries: &[([u8; 3], char)]) -> char {
    entries
        .iter()
        .min_by_key(|(color, _)| {
            rgb.iter()
                .zip(color)
                .map(|(&a, &b)| {
                    let delta = i32::from(a) - i32::from(b);
                    (delta * delta) as u32
                })
                .sum::<u32>()
        })
        .expect("palette checked as non-empty")
        .1
}

pub fn run(input: &Path, output: Option<&Path>, alpha_threshold: u8) -> io::Result<()> {
    let default_output = input.with_extension("pp.json");
    let output = output.unwrap_or(&default_output);
    if !output
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.ends_with(".pp.json") && name.len() > ".pp.json".len())
    {
        return Err(invalid("output filename must be <name>.pp.json"));
    }
    let config: palette::Config = serde_json::from_str(&config::read()?)
        .map_err(|error| invalid(format!("invalid {}: {error}", config::CONFIG_FILE)))?;
    let transparent = config.map.transparency()?;
    let colors = entries(&config.palette.colors, &config.map.colors, "colors")?;
    let normals = entries(&config.palette.normals, &config.map.normals, "normals")?;
    let flat = nearest([128, 128, 255], &normals);
    let image = image::ImageReader::open(input)
        .and_then(|reader| reader.with_guessed_format())
        .map_err(|error| {
            io::Error::new(
                error.kind(),
                format!("cannot read {}: {error}", input.display()),
            )
        })?
        .decode()
        .map_err(|error| invalid(format!("cannot decode {}: {error}", input.display())))?
        .to_rgba8();
    if image.width() == 0 || image.height() == 0 {
        return Err(invalid("input image must have non-zero dimensions"));
    }
    let mut artifact = Artifact {
        colors: Vec::new(),
        normals: Vec::new(),
    };
    let mut cache = HashMap::new();
    let mut partial_alpha = false;
    for y in 0..image.height() {
        let mut color_row = String::new();
        let mut normal_row = String::new();
        for x in 0..image.width() {
            let [r, g, b, a] = image.get_pixel(x, y).0;
            partial_alpha |= a != 0 && a != 255;
            if a <= alpha_threshold {
                let symbol = transparent.ok_or_else(|| invalid(
                    "image contains transparent pixels; configure map.transparent before textifying"
                ))?;
                color_row.push(symbol);
                normal_row.push(symbol);
            } else {
                color_row.push(
                    *cache
                        .entry([r, g, b])
                        .or_insert_with(|| nearest([r, g, b], &colors)),
                );
                normal_row.push(flat);
            }
        }
        artifact.colors.push(color_row);
        artifact.normals.push(normal_row);
    }
    let mut json = serde_json::to_string_pretty(&artifact).map_err(io::Error::other)?;
    json.push('\n');
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output)
        .map_err(|error| {
            let message = if error.kind() == io::ErrorKind::AlreadyExists {
                format!(
                    "{} already exists; choose another --output path",
                    output.display()
                )
            } else {
                format!("cannot create {}: {error}", output.display())
            };
            io::Error::new(error.kind(), message)
        })?;
    file.write_all(json.as_bytes())?;
    println!(
        "Textified {} as {} ({}x{}); initialized flat normals",
        input.display(),
        output.display(),
        image.width(),
        image.height()
    );
    if partial_alpha {
        eprintln!(
            "Note: partial alpha was converted to binary transparency using threshold {alpha_threshold}."
        );
    }
    Ok(())
}
