use crate::generate::{self, Image};
use std::{
    io::{self, Write},
    path::Path,
};

fn pixel(image: &Image, x: usize, y: usize) -> Option<[u8; 3]> {
    if y >= image.height as usize {
        return None;
    }
    let offset = (y * image.width as usize + x) * 4;
    let pixel = &image.pixels[offset..offset + 4];
    (pixel[3] != 0).then(|| [pixel[0], pixel[1], pixel[2]])
}

fn row(output: &mut impl Write, image: &Image, y: usize) -> io::Result<()> {
    for x in 0..image.width as usize {
        match (pixel(image, x, y), pixel(image, x, y + 1)) {
            (Some([r, g, b]), Some([br, bg, bb])) => {
                write!(output, "\x1b[38;2;{r};{g};{b}m\x1b[48;2;{br};{bg};{bb}m▀")?;
            }
            (Some([r, g, b]), None) => {
                write!(output, "\x1b[38;2;{r};{g};{b}m\x1b[49m▀")?;
            }
            (None, Some([r, g, b])) => {
                write!(output, "\x1b[38;2;{r};{g};{b}m\x1b[49m▄")?;
            }
            (None, None) => write!(output, "\x1b[0m ")?,
        }
    }
    write!(output, "\x1b[0m")
}

pub fn run(path: &Path) -> io::Result<()> {
    let (colors, normals) = generate::load(path)?;
    let mut output = io::BufWriter::new(io::stdout().lock());
    writeln!(
        output,
        "Color image + normal map ({}x{})",
        colors.width, colors.height
    )?;
    for y in (0..colors.height as usize).step_by(2) {
        row(&mut output, &colors, y)?;
        write!(output, "    ")?;
        row(&mut output, &normals, y)?;
        writeln!(output)?;
    }
    output.flush()
}
