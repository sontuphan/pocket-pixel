# How to create readable pixel art

A practical guide to drawing principles and a repeatable pixel-art workflow.

## 1. Define the image before adding pixels

Decide what the subject must communicate, how large it will appear, and which
features make it recognizable. Choose the canvas dimensions, viewing angle,
light direction, and intended outline style before detailing the image.

Use references to understand proportions, construction, materials, and defining
features. Study how related artwork separates important subjects from their
backgrounds. Translate those observations into shapes that fit the available
pixels rather than trying to preserve every detail.

For a small sprite, prioritize a recognizable silhouette and a few identifying
features. Remove details that cannot be read at the intended display size.

Choose resolution according to the subject's essential detail and the project's
established scale. Drawing the same subject at several sizes can reveal which
features survive. Small studies help develop sensitivity to individual pixels;
a larger canvas is useful only when its extra detail serves the image.

For a character, write a short design brief: who they are, what they do, and what
the viewer should feel. Translate that brief into visible choices. A grappler
might have a broad body, planted feet, and arms reaching forward; an agile runner
might have a forward lean and a long, open stride. A familiar subject needs its
specific identifying features, not merely the generic shape of a person.

Study references for the visual associations they carry. Clothing, equipment,
historical designs, and earlier character designs can suggest a role or story.
Choose those associations consciously and combine them to serve this character.

## 2. Choose a restrained palette

An established palette lets you concentrate on drawing while keeping colors
coherent. Choose colors with enough value contrast to separate adjacent shapes
at the final size.

Start each material with three roles:

- **Base:** its main visible color.
- **Shadow:** a darker color that describes surfaces receiving less light.
- **Highlight:** a lighter color for surfaces receiving more light.

Three shades per material are a useful starting point, not a requirement. Use
fewer when the shape remains clear. Add more only when they contribute a visible
change in form, material, or emphasis. Several nearly identical shades often
create noise without improving readability.

Separate **hue** (color family), **saturation** (color intensity), and **value**
(lightness or darkness). A palette provides options, not automatic harmony:
choose a subset whose value relationships support the intended forms.

Perceived brightness is not the same as the numeric value in HSV. Colors with
equal HSV saturation and value can look very different in grayscale because
their hues contribute differently to luminance. Inspect the image in grayscale
to check major light and dark groupings, then check it again in color.

Use contrast to guide attention. Differences in value, hue, shape, and detail
can all create emphasis. Reserve stronger contrasts and concentrated detail for
important features, with quieter regions around them. Saturation can also draw
attention; neither high nor low saturation is inherently wrong. Its effect
depends on the surrounding colors and the image's value structure.

## 3. Build the silhouette and proportions

Start with construction lines or broad, solid shapes. Establish the major masses,
their relative sizes, and the spaces between them. Refine these into a clear
outer contour before spending time on texture or highlights.

Explore several rough versions while changes are inexpensive. Vary head-to-body
ratio, shoulder width, limb length, and stance enough to discover alternatives.
Keep earlier versions alongside new ones so you can compare and recover useful
choices. Avoid polishing the first design simply because it came first.

During concept exploration, temporary colors and a larger sketching area can
help keep decisions flexible. Before refining pixels, translate the chosen
design into the actual output dimensions. Final readability must be solved
within that pixel budget.

The starting method can vary: silhouette blocks, construction lines, or a loose
color sketch can all work. If reducing a larger sketch, redraw the resulting
pixel shapes deliberately; shrinking alone does not preserve expression or
readability. Keep the workflow flexible while evaluating the same fundamentals.

Inspect the silhouette as a single filled shape. Ask whether the pose, object,
and important protrusions are understandable. Correct proportions and overlaps
now; shading will not repair a confusing structure.

Choose an orientation that exposes defining features. A distinctive nose, ear,
weapon, or hairstyle may disappear when placed entirely inside the silhouette.
Use negative space between arms, torso, legs, and props to keep these elements
separate. Recheck the silhouette after changing the pose or adding equipment.

Make the pose communicate personality or action. Use the relationship between
head, shoulders, torso, and feet to establish balance and a clear directional
gesture. A neutral front-facing stance is useful when intended, but should not
be the automatic choice for every character. Exaggerate a gesture if it helps
the action read, while preserving the subject's identity.

Once the single-color silhouette works, add a second tone to separate internal
forms and establish depth. Push those two tones as far as they remain useful
before adding a third. Treat new tones as solutions to specific readability
problems, rather than a reason to decorate every surface.

## 4. Clean the pixel edges

Pixel edges should have deliberate rhythms. For a straight diagonal, repeat a
consistent run of pixels. For a curve, change the run lengths gradually as its
slope changes. For example, a shallow curve might progress through runs of
`4, 3, 2, 1` pixels rather than jumping unpredictably between lengths.

Place the endpoints of a difficult curve first, then connect them with runs that
transition smoothly. Match opposite sides when the form is intended to be
symmetrical; preserve asymmetry when the pose or perspective calls for it.

Remove accidental extra corner pixels that make thin contours look bulky or
create unwanted right-angle bumps. These are often called *doubles*. Keep them
when they intentionally describe a corner, reinforce a structure, or belong to
the chosen style. Consistency matters more than eliminating every thick edge.

Uneven steps are often called *jaggies*. Remove them when they accidentally
interrupt a smooth contour, but retain deliberate angular changes that describe
an eyebrow, crease, or rough material. Judge an edge by its intended shape,
rather than treating every irregularity as a defect.

## 5. Fill the main color regions

Assign flat base colors to the major materials and parts. Check that neighboring
regions remain distinguishable before introducing shadows.

Keep the large shapes readable. A tiny patch of a different color should explain
something specific, such as an eye, a fastening, or a material boundary. Avoid
scattering individual pixels merely to make an area look detailed.

Think in **clusters**: connected groups of pixels that describe a coherent
plane, feature, or patch of light. Refine their shapes and relationships before
adding individual marks. An isolated or *orphan* pixel can be useful for an eye,
sparkle, or fine texture, but should have a clear role among the larger clusters.

Identify a few interior shapes that carry identity: an eye arrangement, a helmet
opening, a garment shape, or a recognizable prop. Reserve contrast and space for
them. Small accessories should reinforce the role, story, orientation, or pose;
remove them if they obscure a more important shape.

Give adjacent materials enough separation. Similar-colored skin, clothing, and
equipment can merge even when their outlines are correct. Adjust their value
relationships or shading treatment before adding extra details. Strong, compact
highlights and abrupt contrast can suggest polished metal; broader, quieter
transitions can suggest matte surfaces. Color alone does not establish material.

## 6. Shade according to the light and form

Choose a clear light direction. Identify which surfaces face the light, which
turn away, and where one part blocks light from reaching another.

Apply shadows first and highlights afterward. Shape those regions around the
object's volume: a flat plane, rounded cheek, cylinder, and sharp corner should
not all receive the same shading pattern.

Distinguish the shadow types before simplifying them:

- **Form shadow:** the object's own surface turning away from the light.
- **Cast shadow:** light blocked by an object and falling onto another surface.
- **Contact or occlusion shadow:** darkening in tight contacts or crevices where
  less surrounding light reaches the surface.
- **Bounce light:** illumination reflected from nearby surfaces into a shadow.

The boundary between the lit and shadowed portions is the *terminator*.
At small resolutions, select only the cues needed to describe the form; a
sprite does not need a separate shade for every lighting phenomenon. Keep
bounce light subordinate to the main lighting unless the scene calls for a
strong secondary source. The environment and the number, color, and strength of
light sources determine the actual relationships.

Avoid **pillow shading**: dark bands wrapped evenly around the outline with a
bright center, regardless of the light direction. Shadows should follow the
form and lighting rather than simply tracing the perimeter.

Keep lighting coherent across related assets. In a three-quarter view, both the
top and front may be visible, but their relative brightness depends on the
chosen light direction.

Keep the viewing angle and projection coherent too. Related objects should show
top and side planes using compatible foreshortening. They do not need identical
amounts of visible top surface; their dimensions and orientation also matter.

## 7. Use hue shifts deliberately

A shadow does not have to be the base hue made darker, nor a highlight the same
hue made lighter. Changing hue along a shading ramp can produce richer results.

Possible starting points include a warm red base with orange highlights and
purplish shadows, or green with yellowish highlights and bluish shadows. Treat
these as examples, not universal formulas. Lighting, material, palette, and mood
should determine the actual choices.

Warm and cool are relative descriptions: a neutral or reddish color can appear
warmer or cooler depending on its neighbors. Judge each color in context.
Colored illumination and reflected light can also alter an object's apparent
color, so do not assume its local material color must stay unchanged everywhere.

Preserve a readable value progression even when the hue changes. Attractive
colors will not explain form if the light and shadow regions are too similar.

## 8. Choose outlines that support readability

An outline can be black, a darker material color, a shared accent hue, or absent.
It can also become lighter on the lit side and darker on the shadowed side.
Choose its thickness and treatment deliberately.

Selective or broken outlines can omit or soften portions of the contour to
explain light and form. Test them against the intended background so the subject
does not disappear. Keep the treatment intentional across related assets.

For related assets, establish a consistent visual hierarchy. Important subjects
or interactive objects may use stronger outlines while scenery uses softer ones.
Such differences should guide attention rather than appear accidental.

Build a shared design language for a cast: compatible proportions, eye shapes,
hand and foot treatment, outline rules, and shading. Within that language, vary
silhouette, pose, clothing, and props so characters remain distinguishable.
Consistency should make them feel related without making them interchangeable.

Without outlines, separation must come from clear shapes and sufficient value
or color contrast against the intended background. Check the subject in context,
not only against an empty canvas.

## 9. Add optional finishing techniques sparingly

**Dithering** interleaves two colors in a controlled pattern to suggest an
intermediate tone or texture. Use it to support a material or break up a large
transition. At small sizes, excessive dithering can obscure the main shapes.

**Manual anti-aliasing** places selected intermediate colors near stepped edges
to soften their apparent transition. Use it selectively where it improves a
curve; covering every edge can make the image blurry. Consider the background,
since an edge color chosen for one background may stand out against another.

Watch for **banding**: adjacent rows or clusters of similar shape lining up in
parallel strips, often when shading or smoothing follows an outline too closely.
This can expose the pixel grid and flatten the form. Reshape the shadow or
intermediate-color clusters to describe the surface instead of tracing every
contour. Too much anti-aliasing can also change the silhouette by adding visible
corners; smoothing should preserve the intended shape.

Neither technique is required. Add it only after the silhouette, color regions,
and shading already work.

Test facial details individually at the final size. One eye highlight or a small
change to the brows can alter expression substantially. Keep the version that
supports the intended character, rather than assuming more detail is better.

For animated sprites, consider the cost of each accessory and tiny detail across
poses and frames. Favor shapes that stay readable in motion and can be redrawn
consistently. A decorative feature should justify the extra animation work.

## 10. Review at the intended display size

Alternate between close inspection and the size at which the artwork will
actually be viewed. Enlarged pixels help with placement, but the final-size
image determines whether the artwork succeeds.

Consider the intended display conditions when studying references. Some older
art relied on display softness or blending that is absent in sharp modern
previews. Judge how an image reads in its intended presentation, and avoid
copying a pixel pattern without understanding that context. Do not assume every
older sprite was designed for the same display effect.

Review in this order:

1. **Recognition:** do the silhouette and key interior shapes identify this subject?
2. **Pose and structure:** do the gesture, balance, proportions, and perspective agree?
3. **Lighting:** do shadows and highlights describe one coherent light setup?
4. **Focus and separation:** do value, saturation, and detail emphasize the right shapes?
5. **Edges and clusters:** are contours deliberate, with no distracting jaggies or banding?
6. **Economy:** can unnecessary shades, texture, or isolated details be removed?
7. **Consistency:** does the image fit the related artwork while remaining distinctive?
8. **Motion:** if animated, will defining shapes and details survive changes of pose?

Fix the largest problem first. Refine small details only after the larger
structure reads correctly. Practice with manageable subjects, study what each
revision improves, and build a process you can enjoy repeating.

