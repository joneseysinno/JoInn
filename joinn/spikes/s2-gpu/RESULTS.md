# S2 run results

Written by `cargo run --release` in `joinn/spikes/s2-gpu` (windows on x86_64).

Target 1920×1080, headless (no window, no vsync). Frame time = encode + submit + wait for the GPU: median and 95th percentile of 100 frames after 10 warm-up frames. Interactive means a median under 16.7 ms.

## NVIDIA GeForce RTX 2080 · Vulkan · DiscreteGpu

Driver: NVIDIA 591.86

| Adapter reports | Value | Core minimum |
|---|---|---|
| `max_bind_groups` | 8 | 4 |
| `max_storage_buffers_per_shader_stage` | 524288 | 8 |
| `max_storage_buffer_binding_size` | 2147483644 | 134217728 |
| `max_vertex_buffers` | 16 | 8 |
| Storage buffers in the vertex stage | yes | core: yes · compatibility: no |
| WebGPU compliant (wgpu downlevel check) | true | |

Device requested with exactly the WebGPU core limits (`Limits::defaults()`): granted.

| Instances | Path | Frame median | Frame p95 | Interactive | Pick sweep (10000 points): agree / disagree / on an edge | S and V ID images |
|---|---|---|---|---|---|---|
| 10000 | S · storage, walked in the vertex shader | 0.68 ms | 0.77 ms | yes | 9983 / 0 / 17 |  |
| 10000 | V · vertex-buffer fallback | 0.68 ms | 0.93 ms | yes | 9983 / 0 / 17 | identical |
| 100000 | S · storage, walked in the vertex shader | 3.94 ms | 4.12 ms | yes | 9944 / 0 / 56 |  |
| 100000 | V · vertex-buffer fallback | 3.93 ms | 4.07 ms | yes | 9944 / 0 / 56 | identical |

Single-pixel ID readback (copy one texel, map, wait), median of 50: **0.10 ms**

## NVIDIA GeForce RTX 2080 · Dx12 · DiscreteGpu

Driver: 32.0.15.9186 

| Adapter reports | Value | Core minimum |
|---|---|---|
| `max_bind_groups` | 8 | 4 |
| `max_storage_buffers_per_shader_stage` | 262144 | 8 |
| `max_storage_buffer_binding_size` | 2147483644 | 134217728 |
| `max_vertex_buffers` | 16 | 8 |
| Storage buffers in the vertex stage | yes | core: yes · compatibility: no |
| WebGPU compliant (wgpu downlevel check) | true | |

Device requested with exactly the WebGPU core limits (`Limits::defaults()`): granted.

| Instances | Path | Frame median | Frame p95 | Interactive | Pick sweep (10000 points): agree / disagree / on an edge | S and V ID images |
|---|---|---|---|---|---|---|
| 10000 | S · storage, walked in the vertex shader | 0.59 ms | 0.65 ms | yes | 9983 / 0 / 17 |  |
| 10000 | V · vertex-buffer fallback | 0.61 ms | 0.77 ms | yes | 9983 / 0 / 17 | identical |
| 100000 | S · storage, walked in the vertex shader | 3.99 ms | 4.32 ms | yes | 9944 / 0 / 56 |  |
| 100000 | V · vertex-buffer fallback | 3.95 ms | 4.19 ms | yes | 9944 / 0 / 56 | identical |

Single-pixel ID readback (copy one texel, map, wait), median of 50: **0.16 ms**

## Microsoft Basic Render Driver · Dx12 · Cpu

Driver: 10.0.26100.9278 

| Adapter reports | Value | Core minimum |
|---|---|---|
| `max_bind_groups` | 8 | 4 |
| `max_storage_buffers_per_shader_stage` | 262144 | 8 |
| `max_storage_buffer_binding_size` | 2147483644 | 134217728 |
| `max_vertex_buffers` | 16 | 8 |
| Storage buffers in the vertex stage | yes | core: yes · compatibility: no |
| WebGPU compliant (wgpu downlevel check) | true | |

Device requested with exactly the WebGPU core limits (`Limits::defaults()`): granted.

| Instances | Path | Frame median | Frame p95 | Interactive | Pick sweep (10000 points): agree / disagree / on an edge | S and V ID images |
|---|---|---|---|---|---|---|
| 10000 | S · storage, walked in the vertex shader | 18.65 ms | 20.79 ms | **no** | 9983 / 0 / 17 |  |
| 10000 | V · vertex-buffer fallback | 12.25 ms | 13.07 ms | yes | 9983 / 0 / 17 | identical |
| 100000 | S · storage, walked in the vertex shader | 140.56 ms | 143.34 ms | **no** | 9944 / 0 / 56 |  |
| 100000 | V · vertex-buffer fallback | 78.67 ms | 81.40 ms | **no** | 9944 / 0 / 56 | identical |

Single-pixel ID readback (copy one texel, map, wait), median of 50: **0.18 ms**

## Summary

- Microsoft Basic Render Driver · Dx12 · Cpu: 10000, S · storage, walked in the vertex shader: not interactive (18.65 ms)
