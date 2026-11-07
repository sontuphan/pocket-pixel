# How to draw pixel-art characters

Build a character from clear proportions and simple volumes, then develop its
identity through pose, hair, clothing, and color. Establish one readable base
sprite before expanding it into action poses and animation.

## 1. Define the character and its purpose

A character sprite is an image used to represent a character in a game. It may
depict a player, ally, enemy, or another actor. Related drawings show actions
such as standing, running, jumping, or attacking.

Write a short brief before drawing:

- What personality or role should the character communicate?
- Which features must remain recognizable across poses?
- What viewing angle and style fit the project?
- Which actions will the character need to perform?

Choose a few defining features. A strong design should remain identifiable when
reduced to a simple sketch; it should not depend on many tiny decorations.

Rank those features before drawing. Make the head, gaze direction, and overall
body extent clear first, then prioritize the hair, clothing, or prop most
important to identity. If a minor accessory cannot fit without weakening a
stronger feature, simplify or omit it. Recognizable emotion can matter more
than realistic anatomical proportions at a small resolution.

## 2. Set the scale and proportion plan

Canvas dimensions such as 32×32, 48×48, and 64×64 are useful reference points,
not mandatory character sizes. Canvas size and occupied character size are
different: leave space for hair, extended limbs, equipment, and movement.

When working in a tiled world, compare the character with the environment's
tile size, doorways, furniture, and other characters. A character may occupy
roughly one tile in width while extending across several tiles in height, but
the project's visual scale should determine that relationship.

Distinguish visible sprite bounds from gameplay collision bounds. Hair, tails,
and weapons may extend beyond the body without being solid collision surfaces.
Keep the body's apparent position and contact with walls or ground understandable,
and agree on how decorative extensions behave in play. The collision shape does
not need to match every visible pixel, but unexplained differences can feel wrong.

Plan the relative heights and widths of the head, torso, hips, and legs before
detailing them. A large head produces a different impression from a small head
on a long body. Treat proportions as deliberate design choices.

Check the full silhouette against the available height. Construction volumes
overlap at the neck, waist, and joints, so adding their separate heights does
not necessarily give the final character height. Keep the chosen proportions
consistent when drawing other poses.

Use body-shape changes to support the individual design: shoulder width, waist
taper, hip width, and limb proportions can all affect the impression. These are
stylistic choices, not required formulas for a character's gender or personality.

## 3. Construct the body from simple volumes

Use a few understandable forms to establish both anatomy and orientation:

| Body region | Starting form | What it helps establish |
| --- | --- | --- |
| Head | Sphere or rounded block | Tilt, facing direction, facial placement |
| Torso | Box | Shoulder plane, chest, front and side surfaces |
| Hips | Tapered box or truncated pyramid | Pelvis orientation and leg attachments |
| Legs | Cylinders or tapered blocks | Thigh, knee, calf, and limb direction |
| Arms | Gesture lines, then simple volumes | Reach, elbow position, and thickness |

Simple volumes are one useful approach, especially when a bare skeleton does
not make the body's thickness clear. Gesture lines can also work; use the
construction method that lets you understand and revise the pose.

Orient the forms together. If the view shows the top of the torso, the head,
hips, and limbs should agree with that camera angle. Preserve the difference
between the near and far sides rather than forcing mirrored screen shapes.

## 4. Establish a readable base pose

Begin with a pose that makes the body easy to understand. It can be relaxed,
alert, or ready for action; it does not have to be perfectly upright or
symmetrical.

Position the head, shoulders, hips, and feet as a connected structure. Add a
small lean or turn if it supports the character. Leave enough negative space
between limbs and the torso to distinguish them at the final display size.

For a grounded pose, consider where the feet support the body. The projected
center of mass should plausibly relate to the support area unless the character
is deliberately moving, falling, or recovering balance. Balance does not require
equal visible mass on both sides of a vertical line.

Resolve the silhouette and gesture before adding hair strands or clothing trim.

Temporary, deliberately nonfinal colors can help keep early decisions focused
on shape. Add a second tone when it is needed to separate an arm, torso, or prop.
Do not wait for perfect colors before testing the construction. Revisit the pose
and check it at its intended display size throughout the drawing process.

## 5. Refine anatomy and facial placement

Carve the construction forms into a body instead of leaving every limb as a
uniform tube. Suggest a broader thigh, narrower knee, fuller calf, and tapering
ankle. Apply similar changes at the shoulder, elbow, wrist, and waist.

Use anatomy and pose references to check these transitions. At small sizes,
describe the important changes with a few connected pixel clusters rather than
trying to draw every muscle.

Establish the contour with an outline if that suits the style. Keep temporary
construction guides separate from final artwork. A facial centerline and eye
guide can help align features on the head's volume. In a turned view, the far
eye may be narrower or partly hidden; correct placement is not the same as
mirrored spacing on the canvas.

Treat the face as a small system of interacting shapes. The eye opening, pupil,
brow, and nearby accessories together establish gaze and expression. In a turned
head, place them on the head's volume and keep the looking direction legible.

At resolutions that support visible eye whites, a pupil surrounded by white can
suggest surprise, while one touching an eyelid may suggest a more directed or
relaxed gaze. These are visual cues, not fixed rules: lid shape, brow angle, pupil
size, and context can change the reading. For one-pixel eyes, test alternate
positions rather than attempting details the resolution cannot support.

Glasses and brows may share the same few pixels. Adjust their angle and contrast
carefully so the frames communicate the intended attitude without erasing the
eyes. Compare a few versions at final size; a single pixel can change expression.

If an essential expression or accessory will not read, test changing the head's
width by one pixel or shifting the facial layout. Judge the resulting sprite at
its display size rather than protecting the original construction unnecessarily.
Keep the overall proportions intentional and compare alternatives before settling.

## 6. Design hair as a recognizable shape

Block in the overall hair mass before individual locks. Consider its attachment
to the head, its direction, and how it changes the silhouette.

A distinctive fringe, ponytail, or other large shape can communicate personality
and help the character remain recognizable across poses. Make that shape serve
the design rather than adding arbitrary complexity.

Choose the head angle to reveal important hair shapes when the pose permits it.
A low ponytail or a controlled flyaway can imply grooming habits, movement, or
personality. Use a few purposeful departures from the main hair mass instead of
many stray pixels that read as accidental noise.

Check whether the feature will remain readable and manageable in motion.
Long or elaborate hair needs consistent shape, volume, and attachment across
frames, so weigh its contribution against the animation work it introduces.

## 7. Fit clothing around the body

For close-fitting garments, use the established body volume as the starting
boundary and change the material or color regions. Add only the thickness,
seams, or silhouette changes that matter at this scale.

For loose clothing, extend the silhouette beyond the body. Show where fabric
hangs, flares, or overlaps. A shaped hem can suggest flow, but folds should
follow attachment points, gravity, and the pose rather than becoming random
wavy edges.

Recheck the underlying form after dressing the character. A skirt, cape, or wide
sleeve can erase the waist, arm separation, or direction of the legs. Recover
needed structure through the garment's contour, overlap, and shadow groups;
do not add anatomy lines that would be hidden beneath the fabric.

Small contrast decisions can explain a garment. A lighter cuff beside a darker
hand shadow, followed by a short darker boundary, may identify the sleeve opening.
A few pixels forming a neckline can communicate a collar. Place these accents
where they clarify construction rather than repeating bright trim everywhere.

Place accessories near the intended focal region when they help emphasize it.
Keep the underlying body understandable, and avoid allowing clothing or props
to conceal the pose's defining gaps.

## 8. Separate values before exploring hues

Start with a limited value arrangement so adjacent parts remain distinct. A
monochromatic study can help: use lighter and darker shades within one color
family to establish the important separations.

Place a dark accessory against a lighter garment when it needs emphasis. If
their values become too similar, they may merge even when their hues differ.
Reserve stronger contrast for the face and other identifying features.

Compare a few palette variations without changing the structure. Shift the
dominant hue to test different moods, or replace selected regions with an accent
from another color family. Preserve the readable value relationships while
experimenting. Recheck perceived brightness whenever hues change.

## 9. Refine details without competing with the character

Treat contrast as an attention budget. Decide where the viewer should look
first, then compare that region with the strongest dark-light boundaries
elsewhere. Dark eyes can attract attention, but a darker, larger garment accent
may compete with them. Soften an unimportant region's contrast instead of always
making the focal region more extreme.

Use a few perceptual principles to communicate more with fewer pixels:

- **Closure:** aligned fragments can imply a continuing seam, contour, or texture
  without drawing its entire length. Keep enough consistent direction and context
  for the missing portion to be understood.
- **Proximity:** nearby marks tend to group together. Place detail pixels so they
  belong clearly to an eye, strap, or garment rather than an unrelated feature.
- **Common region:** a shared color area can group details into one object, such
  as markings contained within a backpack or a sleeve.

Suggest secondary textures with a few characteristic marks instead of tracing
every strand, seam, or repeated pattern. Completing all those dark lines can
make the region noisier and more prominent than intended. Remove marks that do
not help the viewer identify the material, structure, or character.

Let edge treatment describe form. A sharp boundary between differently lit
planes can suggest a corner; a broader transition through a few tones can
suggest curvature. Choose the transition according to the shape and lighting,
not merely the desired amount of detail. Rounded forms do not require a smooth
gradient or many extra shades at a small resolution.

A narrow, bright highlight can suggest a reflective edge or small bevel when
the light, viewing angle, and material support it. Use the brightest accents on
small, selected areas rather than outlining every edge with shine. A bright
edge is not automatically a specular reflection.

Check ambiguous props by their silhouette and attachment. A backpack may need
to project behind the torso to read differently from a vest. A held object may
need a clearer tip or a short contrasting boundary where the fingers overlap
it. Establish the larger identifying shape before adding surface decoration.

Use selective internal boundaries too: a short shadow between hair and face,
or between a hand and its prop, may explain an overlap without enclosing every
part in a continuous dark outline. Let those boundaries stop where the viewer
can infer their continuation.

## 10. Develop key action poses

Use the base character as a reference for proportions, costume, and defining
features. Reconstruct the body's volumes for each action instead of merely
moving isolated parts of the original sprite.

- **Idle:** establish a clear resting or ready stance that can anchor a loop.
- **Run:** communicate direction through the torso's lean, bent knees, and
  coordinated arm motion. The amount of lean depends on speed and personality;
  a fixed 45-degree angle is not required.
- **Jump or fall:** decide which phase of the motion is depicted. Use leg
  positions, head direction, and arms to communicate that phase and intention.
- **Wall slide:** establish contact with the wall and make the body's turn,
  supporting limbs, and downward motion understandable.
- **Crouch:** lower the hips and bend the supporting joints while keeping the
  body plausibly supported and the limbs readable.

Maintain a consistent camera angle. A character can turn within the scene, but
its body parts should agree about that turn. Keep defining hair and clothing
features recognizable without forcing them into the same screen position.

These are key-pose studies, not complete animations. They establish the action's
look before timing and intermediate frames are added.

## 11. Review the set before animating

Compare poses at their intended display size. Check the head-to-body ratio,
limb thickness, costume shapes, facial identity, and distribution of detail.
Look for accidental changes in volume or scale between drawings.

Test the sprite against light, dark, and representative scene backgrounds.
Check the face, exposed limbs, hair, and prop tips for disappearing edges or
unwanted emphasis. Adjust local contrast or selective outlines as needed while
keeping the chosen style coherent. Character and environment colors should be
developed together rather than judged only against an empty canvas.

Check recognition in every frame, not only in the original pose. Preserve a
consistent visual shorthand for recurring features such as a shoulder edge,
collar opening, hair attachment, or glasses. Their shapes can change with the
view, but their placement should continue to explain the same character.

Use consistent frame dimensions and a deliberate alignment reference so the
character does not drift unintentionally. A planted foot can serve as a useful
reference during ground contact; airborne poses need their intended trajectory.

A sprite sheet arranges multiple drawings into a larger image. Frames may be
grouped by action and playback order, but their timing and interpretation must
also be defined for the intended use. One pose per action is a useful foundation,
not a finished idle, run, or jump cycle.

Choose a sustainable detail level before committing to the full animation set.
Consider whether the same features can be drawn consistently across all actions,
directions, and other characters. Simplify redundant shades and repeated trim
when they add production work without strengthening recognition.

Refine the largest structural problem first. Repeatedly drawing the same
character in different poses is a practical way to learn its construction and
prepare for animation.
