# Workspace sketch

A window for AJ to look at. It is a spike. It is not JoInn's storage, and nothing under `crates/` uses it.

From the repository root:

```
cargo run --manifest-path joinn/spikes/s9-workspace/Cargo.toml -p workspace-app
```

## Keys

- Click a cell in the palette, or drag it onto the canvas.
- Double-click a name to rename it. The panel on the right edits the name, the value, the unit, and the formula.
- Drag a cell to move it. Or select it, press M, and click the group it should move into.
- Select a cell, press L, and click another cell to draw a grey link. A formula draws its own green line, with an arrow.
- The wheel zooms toward the pointer. Drag empty canvas to pan.
- Double-click a group to go inside it. Esc goes up one. Home frames the whole worksheet.
- Delete removes the selected cell.
- The slider along the bottom moves through time. Left and right arrows step one entry while the slider has focus.
- In the past, the worksheet is read-only. Return to now, or Restore this moment.
- Open example loads Simple beam.
