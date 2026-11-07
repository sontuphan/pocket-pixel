# Create pixel art with pocket-pixel

Use pocket-pixel to collaborate with the user on sprites and tilesets. Deliver
an editable `<name>.pp.json` artifact containing a text-based color image and
normal map, then generate `<name>.png` and `<name>_normal.png` from it.

## Prepare the art project

Work in the user's artwork directory. Run `pocket-pixel get config` to read
`pocket-pixel.config.json`; if it is missing, run `pocket-pixel init` to create it.
Run `pocket-pixel get palette` to
inspect the current colors, normal-map colors, and mapped characters.

Read existing artwork, references, and the user's art direction. Use the
configured palette and tile dimensions. Ask about missing artistic requirements
only when they materially affect the result; otherwise choose a reasonable
starting point and describe it briefly.

## Create the artifact

Create `<name>.pp.json` in the art project. Store both grids as arrays of row
strings in a single JSON object:

```json
{
  "colors": [
    ".@@.",
    "@@@@",
    "@@@@",
    ".@@."
  ],
  "normals": [
    "↖↑↑↗",
    "←••→",
    "←••→",
    "↙↓↓↘"
  ]
}
```

This is a 4×4 structure example using the default mappings. Draw the real
artifact at the project's intended dimensions and use its current mappings.

Use one character per pixel and one array entry per pixel row.
For a single tile, use `tileset.tileWidth` columns and `tileset.tileHeight` rows.
For a larger composition, use the agreed dimensions and tile layout.

Select colors from `palette.colors`. Represent color at index `i` with the
character at `map.colors[i]`. The arrays match by position; their indices are
not ASCII codes. Read the hex values to understand hue and brightness rather
than relying on the apparent density of a character.

Start with the silhouette and broad color regions, then refine proportions,
lighting, outlines, and details. Follow the style of related project artwork.
Use only mapped characters. `.` is a mapped color, not transparency; the current
config does not define transparent pixels.

Keep labels, coordinate guides, and explanations outside the row strings.
Check that the artifact is valid JSON, row lengths match, and every character
has a configured mapping. Count Unicode characters, not UTF-8 bytes.
Show the text artwork to the user and state the artifact path and pixel dimensions.

## Draw the normal map

Draw the `normals` grid in the same artifact, with the same dimensions and pixel
positions as `colors`. Represent `palette.normals[i]` with `map.normals[i]`.

Normal colors encode surface directions: red is X (right), green is Y (up),
and blue is Z (toward the viewer). In the default mapping, `•` faces the viewer;
arrows tilt 45 degrees toward the indicated direction. Describe the object's
surface shape independently of its painted colors and shadows.

## Generate the images

After drawing both grids, run:

```sh
pocket-pixel generate <name>.pp.json
```

It produces `<name>.png` for the color image and `<name>_normal.png` for
the normal map alongside the artifact. Keep `<name>.pp.json` as the editable
source. The command reads the config from the current directory, validates both
grids, and writes one pixel per character. Outputs use the grids' dimensions;
they are not resized to the configured tile size. Existing PNG outputs are
replaced when regenerating. Inspect the generated images and report their paths.

## Refine together

Apply the user's feedback to the existing artwork. Make targeted changes,
preserve unrelated details, and keep related tiles stylistically consistent.
Edit the artifact first, then regenerate both images.
Show the revision and briefly explain the changes. Keep palette and mapping
arrays aligned if the user requests configuration changes.

## Available tool support

- `pocket-pixel init`: create the default config without overwriting an existing one.
- `pocket-pixel get config`: print the current config as JSON.
- `pocket-pixel get palette`: display the configured colors and mappings.
- `pocket-pixel palette`: shortcut for `pocket-pixel get palette`.
- `pocket-pixel discover`: read these art collaboration instructions.
- `pocket-pixel generate <name>.pp.json`: validate and render both grids to PNGs.
- `pocket-pixel --help`: inspect available commands.

Create and revise the JSON artifact with your file-editing tools, then use
`generate` to validate and render it. Unmapped characters, invalid hex colors,
duplicate mappings, uneven rows, and mismatched grid dimensions must be fixed
before generation succeeds. PNGs are opaque RGB images; transparency is not yet
supported.
