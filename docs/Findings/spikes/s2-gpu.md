# S2 · Core-limits GPU spike

**Can kill Part II §7.** **Run 28 Sep 2026 on AJ's desktop. Not fired.**

## What was built

The first version of this file described a binary that was never committed. It was written on 28 Sep 2026 and lives in `joinn/spikes/s2-gpu/`. It has its own `[workspace]`, so the JoInn workspace never builds it, lints it or depends on it. `cargo run --release` there writes `RESULTS.md` beside it. That file is the raw evidence for everything below.

For every adapter the machine offers, including the software one, the binary:

1. Records the adapter's own limits and whether it can read storage buffers in the vertex stage.
2. Requests a device with **exactly the WebGPU core limits** (`Limits::defaults()`). A strong desktop is held to what the weakest core device must offer.
3. Binds Part II §7.2's layout: 4 bind groups (tick, universe, genome, pass), with 7 storage buffers visible to the vertex stage (body, cell, port, link, incidence, shape instances, style).
4. Draws 10,000 and 100,000 instanced SDF rounded rects at 1920×1080, two ways:
   - **S**: instance → cell → body, walked in the vertex shader from the storage tables (§7 as written).
   - **V**: the fallback. The same rects are resolved on the CPU and fed as a per-instance vertex buffer, with no storage in the vertex stage.
5. Writes a color target and an R32Uint ID target, reads the ID target back asynchronously, and checks 10,000 random points against a CPU pick of the same tables.
6. Compares the S and V ID images pixel for pixel.

Frame time is headless: encode + submit + wait for the GPU, with no window and no vsync. It is the median of 100 frames after 10 warm-up frames. Interactive means a median under 16.7 ms.

## Runtime finding (Windows, AJ's desktop)

| Adapter | Vertex-stage storage | Core-limits device | 10k S | 10k V | 100k S | 100k V | Pick sweep | S = V images | 1-texel readback |
|---|---|---|---|---|---|---|---|---|---|
| RTX 2080 · Vulkan | yes | granted | 0.68 ms | 0.68 ms | 3.94 ms | 3.93 ms | 0 disagree | identical | 0.10 ms |
| RTX 2080 · DX12 | yes | granted | 0.59 ms | 0.61 ms | 3.99 ms | 3.95 ms | 0 disagree | identical | 0.16 ms |
| Microsoft Basic Render Driver (WARP, CPU) · DX12 | yes | granted | **18.65 ms** | 12.25 ms | 140.56 ms | 78.67 ms | 0 disagree | identical | 0.18 ms |

Earlier cross-check (Linux, cloud, 28 Sep): llvmpipe (CPU) · Vulkan: 10k S 59.92 ms, V 52.64 ms; 100k S 449.80 ms, V 433.77 ms. The pick sweep and the S = V images gave the same results.

The pick sweep was 9,983 agree, 0 disagree and 17 on an edge at 10k, and 9,944 / 0 / 56 at 100k, on every adapter. "On an edge" means within 0.01 px of a shape boundary, too close to call in floating point. Those points are counted, not judged.

## What it means

- **§7.2 fits core limits.** Four bind groups and seven vertex-stage storage buffers were granted and validated on every adapter.
- **Picking is exact.** GPU IDs matched the CPU pick on every non-edge point, on three adapters and two backends. A single-texel readback costs 0.1–0.2 ms, so the roadmap's "CPU pick immediate, GPU pick confirming" is cheap.
- **The fallback is a real second engine.** Path V drew the same ID image as path S on every adapter. If a target lacks vertex-stage storage, the upward wrap does not change.
- **A real GPU has room to spare.** The RTX 2080 draws 100k rects in about 4 ms at core limits. Walking the tables costs nothing measurable there (S ≈ V).
- **A software renderer is the stand-in for weak, and it is marginal.** WARP runs path S at 18.65 ms, just over the 16.7 ms line, and path V at 12.25 ms. On a CPU renderer, walking instance → cell → body costs about 1.5× at 10k and 1.8× at 100k. That extra cost does not show on a GPU.

## The trade-off this spike exposes (carry into Phase 6)

Path V is only fast because its rects were resolved once, before timing. In a live universe, any change to a body's chart (every zoom or pan tick, Part II §9) changes the screen position of every cell in that body. Under path V that means rewriting every instance row of the body, which is exactly the failure named in risk #7 (the delta protocol degenerating into a rebuild). Path S rewrites one body row. So:

- **S is the design.** Its per-frame cost buys a delta protocol that stays small.
- **V is the fallback** for targets without vertex-stage storage, and it pays in upload volume instead of shader time. Phase 6 should instrument rows rewritten per tick on both paths, not only frame time.

## What this run did not settle

- **V31 is not met.** No real weak device was in the test set. The desktop has one discrete GPU and no integrated GPU, and WARP is a simulation of weakness, not a weak device. Carry forward: the first real weak device (old laptop, Chromebook, phone browser) re-runs this binary unchanged.
- **No adapter lacked vertex-stage storage,** so the compatibility-mode fork was exercised only as a design path (V), not observed on hardware that forces it.
- **The browser leg was not run.** The spike is native only.
- **No window, no present.** Frame times exclude presentation and vsync.

## Kill criterion

**Not fired.** Part II §7 fits WebGPU core limits and picks exactly. The vertex-buffer fallback works and agrees pixel for pixel, so losing vertex-stage storage would cost upload volume, not a redesign.
