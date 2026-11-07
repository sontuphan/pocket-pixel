use crate::generate::{self, Image};
use std::{
    io::{self, Write},
    path::Path,
};

fn row(output: &mut impl Write, image: &Image, y: usize) -> io::Result<()> {
    let width = image.width as usize;
    for x in 0..width {
        let top = (y * width + x) * 3;
        let [r, g, b] = image.pixels[top..top + 3] else {
            unreachable!()
        };
        write!(output, "\x1b[38;2;{r};{g};{b}m")?;
        if y + 1 < image.height as usize {
            let bottom = ((y + 1) * width + x) * 3;
            let [r, g, b] = image.pixels[bottom..bottom + 3] else {
                unreachable!()
            };
            write!(output, "\x1b[48;2;{r};{g};{b}m")?;
        } else {
            // Leave the missing lower pixel in the terminal's default background.
            write!(output, "\x1b[49m")?;
        }
        write!(output, "▀")?;
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
