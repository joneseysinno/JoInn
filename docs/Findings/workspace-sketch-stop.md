# Workspace sketch

```
cargo run --manifest-path joinn/spikes/s9-workspace/Cargo.toml -p workspace-app
```

1. Drag a cell from the palette onto the canvas, name it, and give it a value. Clicking the palette adds a cell too.
2. Drag cells inside other cells, as deep as you like. Select a cell, press M, and click a group when a drag is awkward.
3. Zoom with the wheel from the whole worksheet down into one cell and back. The trail at the top says where you are. Home frames the top. Double-click a group to go in. Esc goes up one.
4. Type a formula into a Result (`w * L^2 / 8`) and see the answer, with a line from each cell it uses.
5. Drag the time slider back and watch the worksheet un-build, step by step. Left and right arrows step while the slider has focus.
6. Open example, Simple beam: 24 ft, 1.2 kip/ft. Reactions 14.4 kip, moment 86.4 kip·ft, and Check says pass.

## Cuts and fallbacks

- Band crossfades were left out.
- A palette cell can be clicked to add it, as well as dragged.
- Press M, then click a group, to move a cell when a drag does not land.
- A new worksheet starts as a free group named Worksheet. Simple beam is a stack, as its table says.

## Snags

- The sketch plan was already committed, so the first step only updated the two pointer files.
- Zooming back out through a group leaves the root at a different scale from the one you started at. The picture does not jump. The enter and leave sizes are different on purpose, so the view does not flicker.

## Tests

11 tests passed in `joinn/spikes/s9-workspace`.
