use crate::{config, palette};
use serde::Deserialize;
use std::{collections::HashMap, fs, io, path::Path};

#[derive(Deserialize)]
struct Artifact {
    colors: Vec<String>,
    normals: Vec<String>,
}

pub(crate) struct Image {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) pixels: Vec<u8>,
}

fn invalid(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}

fn render(
    rows: &[String],
    colors: &[String],
    symbols: &[String],
    section: &str,
) -> io::Result<Image> {
    palette::validate(colors, symbols, section)?;
    let width = rows.first().map_or(0, |row| row.chars().count());
    if width == 0 || rows.is_empty() {
        return Err(invalid(format!("{section} must be a non-empty grid")));
    }
    let mut mapping = HashMap::new();
    for (symbol, color) in symbols.iter().zip(colors) {
        mapping.insert(symbol.chars().next().unwrap(), palette::rgb(color)?);
    }
    let mut pixels = Vec::new();
    for (y, row) in rows.iter().enumerate() {
        if row.chars().count() != width {
            return Err(invalid(format!(
                "{section} row {} must have {width} pixels",
                y + 1
            )));
        }
        for (x, symbol) in row.chars().enumerate() {
            let color = mapping.get(&symbol).ok_or_else(|| {
                invalid(format!(
                    "unmapped character {symbol:?} in {section} at row {}, column {}",
                    y + 1,
                    x + 1
                ))
            })?;
            pixels.extend_from_slice(color);
        }
    }
    Ok(Image {
        width: u32::try_from(width).map_err(|_| invalid("grid width exceeds PNG limits"))?,
        height: u32::try_from(rows.len()).map_err(|_| invalid("grid height exceeds PNG limits"))?,
        pixels,
    })
}

fn encode(image: &Image) -> io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut bytes, image.width, image.height);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().map_err(io::Error::other)?;
        writer
            .write_image_data(&image.pixels)
            .map_err(io::Error::other)?;
        writer.finish().map_err(io::Error::other)?;
    }
    Ok(bytes)
}

pub fn run(path: &Path) -> io::Result<()> {
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .and_then(|name| name.strip_suffix(".pp.json"))
        .filter(|name| !name.is_empty())
        .ok_or_else(|| invalid("artifact filename must be <name>.pp.json"))?;
    let (colors, normals) = load(path)?;
    // Validate and encode both images before changing either output file.
    let color_png = encode(&colors)?;
    let normal_png = encode(&normals)?;
    let color_path = path.with_file_name(format!("{name}.png"));
    let normal_path = path.with_file_name(format!("{name}_normal.png"));
    for (output, bytes) in [(&color_path, color_png), (&normal_path, normal_png)] {
        fs::write(output, bytes).map_err(|error| {
            io::Error::new(
                error.kind(),
                format!("cannot write {}: {error}", output.display()),
            )
        })?;
    }
    println!(
        "Generated {} and {} ({}x{})",
        color_path.display(),
        normal_path.display(),
        colors.width,
        colors.height
    );
    Ok(())
}

// Shared validation and rendering for PNG generation and terminal previews.
pub(crate) fn load(path: &Path) -> io::Result<(Image, Image)> {
    let config: palette::Config = serde_json::from_str(&config::read()?)
        .map_err(|error| invalid(format!("invalid {}: {error}", config::CONFIG_FILE)))?;
    let text = fs::read_to_string(path).map_err(|error| {
        io::Error::new(
            error.kind(),
            format!("cannot read {}: {error}", path.display()),
        )
    })?;
    let artifact: Artifact = serde_json::from_str(&text)
        .map_err(|error| invalid(format!("invalid {}: {error}", path.display())))?;
    let colors = render(
        &artifact.colors,
        &config.palette.colors,
        &config.map.colors,
        "colors",
    )?;
    let normals = render(
        &artifact.normals,
        &config.palette.normals,
        &config.map.normals,
        "normals",
    )?;
    if (colors.width, colors.height) != (normals.width, normals.height) {
        return Err(invalid("colors and normals must have matching dimensions"));
    }
    Ok((colors, normals))
}
