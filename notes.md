# WGPU Quick Reference Cheat Sheet

## 1. The Core Hardware (Setup)
*   **Instance:** The entry point. Used only to find your physical GPU.
*   **Adapter:** The physical hardware (e.g., your RTX 3050). 
*   **Device:** Your logical connection to the GPU. **Use this to create things** (Buffers, Textures, Pipelines).
*   **Queue:** The execution line. **Use this to send things** (Data, Commands) to the GPU.

## 2. Memory (The Data)
### **Buffer**
A raw, contiguous block of bytes in GPU memory. Types (`wgpu::BufferUsages`):
*   `UNIFORM`: Small, fast, read-only in shader. Good for single structs (e.g., `Camera`). Max size usually 64KB.
*   `STORAGE`: Large, read/write in shader, supports dynamic array lengths (`array<T>`). Good for lists (e.g., `Spheres`).
*   `VERTEX`: Holds 3D model points (X, Y, Z, UV, Normals).
*   `INDEX`: Holds the order to connect vertices into triangles.
*   `COPY_DST` / `COPY_SRC`: Required if you want to copy data TO or FROM this buffer.
*   `MAP_READ` / `MAP_WRITE`: Required to read/write the buffer directly with the CPU.

### **Texture**
A 1D, 2D, or 3D grid of formatted pixels. 
*   **TextureView:** A "lens" to look at a texture. Shaders and BindGroups require Views, not raw Textures.
*   **Sampler:** Tells the GPU *how* to read a texture (e.g., pixelated vs. blurry when zoomed in).

## 3. The Blueprints (The "Layouts")
These are definitions. They hold **zero data**. You create them once at startup.
*   **Shader Module:** Your loaded WGSL code.
*   **BindGroupLayout:** A blueprint for a single `@group`. Tells the GPU: *"Expect a Texture at binding 0 and a Uniform Buffer at binding 1."*
*   **PipelineLayout:** A collection of BindGroupLayouts. The ultimate blueprint defining *everything* the shader will take as input.

## 4. The Connections (The "Actual Data")
*   **BindGroup:** The actual bridge. It locks specific Buffers and TextureViews into a BindGroupLayout. 
    *   *Rule:* If you delete or recreate a Buffer/Texture, you **MUST** recreate the BindGroup pointing to it.

## 5. The State (The "Pipelines")
Pre-compiled GPU states. Creating them is slow; using them is fast. They lock in the Shader and the PipelineLayout.
*   **ComputePipeline:** Used for raw math and writing to Storage Buffers/Textures (e.g., Raytracing). 
*   **RenderPipeline:** Used for rasterization. Draws triangles to the screen. Locks in vertex layouts, color formats, and depth-testing rules.

## 6. Execution (The Action)
### **CommandEncoder**
A CPU-side "notepad". You write down commands here (`encoder.begin_compute_pass()`, `copy_texture_to_buffer()`). **Nothing executes yet.**

### **Passes**
Specific sections in the CommandEncoder where you bind your pipelines and data.
*   **ComputePass:** You call `set_pipeline()`, `set_bind_group()`, and finally `dispatch_workgroups(x, y, z)`.
*   **RenderPass:** You call `set_pipeline()`, `set_bind_group()`, `set_vertex_buffer()`, and finally `draw()`.

### **Shader Execution Timing**
Shaders **DO NOT** execute when you call `dispatch` or `draw`.
1.  You write commands to the `CommandEncoder`.
2.  You finish the notepad: `let command_buffer = encoder.finish()`.
3.  You hand the notepad to the queue: `queue.submit([command_buffer])`.
4.  **NOW** the GPU executes the shaders asynchronously.

---

## The TL;DR Flowchart

1. **Setup:** `Instance` -> `Adapter` -> `Device` + `Queue`.
2. **Define:** `Shader` + `BindGroupLayout` = `Pipeline`.
3. **Allocate:** `Device` creates `Buffer` and `Texture`.
4. **Bridge:** Map `Buffer`/`Texture` to `BindGroupLayout` by creating a `BindGroup`.
5. **Record:** 
   * `Encoder` opens `Pass`.
   * Set `Pipeline`.
   * Set `BindGroup`.
   * Call `Draw` or `Dispatch`.
6. **Execute:** `Encoder.finish()` -> `Queue.submit()`.
