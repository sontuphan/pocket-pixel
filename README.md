# pocket-pixel

A Rust CLI for text-first pixel art collaboration between humans and agents.

## Development

```sh
cargo run -- --help
cargo run -- --version
cargo run -- init
cargo run -- get config
cargo run -- get palette
cargo run -- discover
cargo run -- generate hero.pp.json
cargo run -- textify reference.png
```

## Install locally

```sh
cargo install --path .
pocket-pixel --help
```

Run `pocket-pixel init` in your art project to create `pocket-pixel.config.json`.
It includes an editable [ENDESGA 32 palette](https://lospec.com/palette-list/endesga-32)
by ENDESGA and a default tile size of 32×32 pixels. Existing configs are never
overwritten.

Run `pocket-pixel get config` to print the current directory's config as JSON.

Run `pocket-pixel get palette` (or `pocket-pixel palette`) to display its colors
and normals with their indices, mapped characters, and hex values. Color swatches
appear in interactive terminals; redirected output is plain text. Set `NO_COLOR`
to disable swatches.

Run `pocket-pixel discover` to print instructions for agents creating and refining
artwork with the installed tool in an art project. It works
without a config and does not write files. To make it discoverable in an art
project, add `Run pocket-pixel discover before working on pixel art.` to that
project's `AGENTS.md` or `CLAUDE.md`.

`palette.colors` lists hex colors, and `map.colors` lists ASCII characters.
The arrays align by position: `map.colors[i]` represents `palette.colors[i]`.
The default mapping
assigns 32 unique printable characters in increasing color luminance, using
roughly smaller marks for dark colors and denser characters for bright colors.
Character density depends on the font. For example, `.` maps to `#181425` and
`@` maps to `#ffffff`. Each character represents one pixel; mappings can be edited.

`palette.normals` lists nine RGB hex colors encoding surface directions,
and `map.normals` lists their symbols
in the same order, using the same index-based mapping as colors:

```text
↖ ↑ ↗
← • →
↙ ↓ ↘
```

Red encodes X (right), green encodes Y (up), and blue encodes Z (toward the
viewer). Each direction component is encoded as `round((component + 1) * 127.5)`.
`•` faces the viewer (`#8080ff`); the other normals tilt 45 degrees toward their
indicated direction, such as `→` (`#da80da`) and `↑` (`#80dada`). Normal canvases
will be separate from color canvases, with one symbol per pixel.

Create an artifact such as `hero.pp.json` with two text grids:

```json
{
  "colors": [".@", "@."],
  "normals": ["•→", "↑↖"]
}
```

Run `pocket-pixel generate hero.pp.json` to write `hero.png` and `hero_normal.png`
beside the artifact. Both grids must be non-empty, rectangular, have matching
dimensions, and use the current config's mapped characters. Unicode symbols
count as one pixel each. Images use the grids' dimensions without scaling and
use RGBA with alpha 0 for transparent pixels and alpha 255 for mapped colors.
Existing output PNGs are replaced on regeneration; the artifact
is preserved. The config is read from the current working directory.

`map.transparent` defines a shared transparent character for both grids. New
configs use a space (`"transparent": " "`). Keep transparent positions identical
in `colors` and `normals`; spaces count toward row widths and must be preserved.
Choose a character not already used in either mapping. `.` remains a dark color.
Older configs without this optional setting keep all mapped pixels opaque; add
`"transparent": " "` under `map` to enable it in an existing project.

Convert an existing image into the text format:

```sh
pocket-pixel textify reference.png
pocket-pixel textify reference.png --output hero.pp.json
```

`textify` supports PNG, JPEG, WebP, GIF (first frame), and BMP. It preserves pixel
dimensions, without resizing or dithering. Each opaque pixel is mapped to the
nearest `palette.colors` entry by squared RGB distance and represented by the
matching `map.colors` character. Equal distances favor the first palette entry.

Alpha values at or below 127 become `map.transparent`; larger values become
fully opaque. Change the cutoff with `--alpha-threshold 0` to keep every pixel
with nonzero alpha opaque. Transparent input requires a configured transparent
character. Both grids receive the same transparency mask.

The normal grid starts flat, using the configured normal color nearest to
`#8080ff`; surface shape is not inferred from the image. Refine normals afterward.
The default output is `<input-name>.pp.json` beside the image. Existing files
are never overwritten, and the source image and config are preserved.
