# Create pixel art with pocket-pixel

Use pocket-pixel to collaborate with the user on sprites and tilesets. Deliver
editable text artwork using the project's palette and character mappings.

## Prepare the art project

Work in the user's artwork directory. Run `pocket-pixel get config` to read
`pocket-pixel.config.json`; if it is missing, run `pocket-pixel init` to create it.
Run `pocket-pixel get palette` to
inspect the current colors, normal-map colors, and mapped characters.

Read existing artwork, references, and the user's art direction. Use the
configured palette and tile dimensions. Ask about missing artistic requirements
only when they materially affect the result; otherwise choose a reasonable
starting point and describe it briefly.

## Draw the artwork

Create a text grid with one character per pixel and one line per pixel row.
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

Save the grid in the art project using its existing file conventions, or a
plain text file if none exist. Keep labels, coordinate guides, and explanations
outside the grid. Show the artwork to the user and state the file path and pixel
dimensions. Check row lengths and character mappings before sharing.

## Add normal maps when needed

Use a separate text grid with the same dimensions and pixel positions as the
color grid. Represent `palette.normals[i]` with `map.normals[i]`.

Normal colors encode surface directions: red is X (right), green is Y (up),
and blue is Z (toward the viewer). In the default mapping, `•` faces the viewer;
arrows tilt 45 degrees toward the indicated direction. Describe the object's
surface shape independently of its painted colors and shadows.

## Refine together

Apply the user's feedback to the existing artwork. Make targeted changes,
preserve unrelated details, and keep related tiles stylistically consistent.
Show the revised grid and briefly explain the changes. Keep palette and mapping
arrays aligned if the user requests configuration changes.

## Available tool support

- `pocket-pixel init`: create the default config without overwriting an existing one.
- `pocket-pixel get config`: print the current config as JSON.
- `pocket-pixel get palette`: display the configured colors and mappings.
- `pocket-pixel palette`: shortcut for `pocket-pixel get palette`.
- `pocket-pixel discover`: read these art collaboration instructions.
- `pocket-pixel --help`: inspect available commands.

Create and revise text grids with your file-editing tools. The CLI currently
supports configuration and palette inspection; image rendering, export, and
canvas validation are not yet available. Present text artwork as text artwork.
