# S2 · Core-limits GPU spike

**Can kill Part II §7.**

## What was built

A throwaway `wgpu` binary in `joinn/spikes/s2-gpu/` that:

1. Requests an adapter and prints the limits that Part II §7 cares about (`max_bind_groups`, `max_storage_buffers_per_shader_stage`, vertex-stage storage).
2. Attempts a vertex shader that reads a storage buffer of instance records (position, size, id).
3. Draws instanced SDF rounded rects (procedural, no vertex buffer of meshes) and an integer ID target with async readback.

The spike is excluded from the workspace. It is not a crate anyone else depends on.

## Spec finding (device-independent)

WebGPU **core** allows storage buffers in the vertex stage. WebGPU **compatibility mode** does not: `STORAGE` is not a valid vertex-stage buffer binding in compatibility.

If the weakest device, or the browser target, is compatibility-only, per-instance data moves into vertex buffers. The upward wrap does not change. That is the whole point of running this before Phase 6.

## Runtime finding

Recorded when the binary is actually run on a device:

| Item | Value |
|---|---|
| Adapter | *(fill on run)* |
| `max_bind_groups` | core minimum 4 |
| `max_storage_buffers_per_shader_stage` | core minimum 8 |
| Vertex-stage storage buffers | core: yes · compatibility: no |
| 10k SDF rects, interactive | *(fill on run)* |
| ID readback vs CPU sweep | *(fill on run)* |

Until a weak device is in the test set, this spike has **not** killed Part II §7 and has **not** cleared it. The spec fork is the finding that matters: plan for vertex-buffer fallback; do not assume vertex-stage storage on every target.

## Kill criterion

Not fired on the spec. Compatibility-mode unavailability is the expected fork, already named in the roadmap, not a surprise that would collapse D5.
