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
cargo run -- preview hero.pp.json
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
are opaque RGB. Existing output PNGs are replaced on regeneration; the artifact
is preserved. The config is read from the current working directory.

Run `pocket-pixel preview hero.pp.json` to display the color image on the left
and the normal map on the right directly from the artifact, without writing PNGs.
It uses ANSI truecolor and Unicode half blocks to display two pixel rows per
terminal row. Use a truecolor terminal wide enough for twice the image width plus
four columns. Redirected preview output also contains ANSI escape sequences.
