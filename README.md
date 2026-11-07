# pocket-pixel

A Rust CLI for text-first pixel art collaboration between humans and agents.

## Development

```sh
cargo run -- --help
cargo run -- --version
cargo run -- init
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

Canvas editing, validation, and rendering will be added next.
