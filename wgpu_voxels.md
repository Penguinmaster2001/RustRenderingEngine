### The Tech Stack

- **wgpu:** Graphics API.
- **winit:** Window creation and event loop.
- **glam:** Linear algebra (math).
- **rayon:** Data parallelism (multithreading).
- **bytemuck:** Casting raw bytes for buffer uploads.
- **log / env_logger:** Logging.

---

## Phase 1: Workspace Setup & The Modern Graphics Pipeline

**What you will learn:**

- **Rust Workspaces:** Managing multiple crates in one project.
- **WGPU Concepts:** Adapters, Devices, Queues, Surfaces, and Swapchains.
- **The Pipeline:** Unlike OpenGL, you cannot change render state on the fly. You will learn to build **Pipeline State Objects (PSOs)**.

### Implementation Steps

1.  **Initialize Workspace:**
    Create a root `Cargo.toml`. Define the following members:
    - `voxel-engine` (Binary, the entry point).
    - `voxel-render` (Library, handles wgpu interaction).

2.  **The Render Context (`voxel-render`):**
    Create a struct `RenderState`.
    - Initialize `winit` window.
    - Request a `wgpu::Instance` and `wgpu::Surface`.
    - Request an `Adapter` (physical GPU) and `Device`/`Queue` (logical connection).
    - Configure the **Swapchain** (handling VSync and format).

3.  **The Event Loop (`voxel-engine`):**
    - Set up the `winit` event loop.
    - Create a clean separate of logic: `update()` and `render()`.
    - **Goal:** Get a window to open and clear to a specific color (e.g., Cornflower Blue).

---

## Phase 2: Math, Camera, and Uniforms

**What you will learn:**

- **Uniform Buffers:** How to send global data (MVP matrices) to shaders.
- **Bind Groups:** The wgpu equivalent of "Texture Slots" and "UBO bindings," but strictly validated.
- **Coordinate Systems:** Moving from OpenGL's right-handed (Y-up) to wgpu’s coordinate system (Z is 0 to 1, unlike GL's -1 to 1).

### Implementation Steps

1.  **Create Crate: `voxel-core`:**
    - Add `glam` as a dependency.
    - Create a `Camera` struct with position, yaw, and pitch.
    - Implement a method to generate the **View-Projection Matrix**.

2.  **Shader Implementation:**
    - Write a basic WGSL shader (`shader.wgsl`). wgpu uses WGSL, not GLSL.
    - Define a struct for the `CameraUniform` in both Rust and WGSL. **Important:** Learn about `#[repr(C)]` and padding/alignment requirements in Rust.

3.  **The Binding Layout:**
    - In `voxel-render`, create a `BindGroupLayout` (describes the inputs to shaders).
    - Create a `Buffer` for the camera uniform.
    - Create a `BindGroup` (actual data linked to the layout).

4.  **Integration:**
    - Update the camera position based on keyboard input in `voxel-engine`.
    - Write the new matrix to the buffer every frame.

---

## Phase 3: Voxel Data & The Chunk System

**What you will learn:**

- **Memory Layout:** How to store voxel data efficiently (flat arrays vs. vectors).
- **Rust Traits:** Defining common behavior for blocks.
- **Coordinate Mapping:** Converting (x, y, z) to array indices.

### Implementation Steps

1.  **Create Crate: `voxel-world`:**
    - Define a `BlockID` (u8 or u16).
    - Define a constant `CHUNK_SIZE` (e.g., 32x32x32).

2.  **The Chunk Struct:**
    - Implement `Chunk`. Instead of `Vec<Block>`, use `Box<[Block; CHUNK_SIZE^3]>`. This keeps chunks on the heap but avoids vector resizing overhead.
    - Implement helper functions: `get_block(x,y,z)` and `set_block(x,y,z)`.

3.  **World Management:**
    - Create a `World` struct containing a `HashMap<(i32, i32, i32), Chunk>`.
    - Implement simple noise generation (use the `noise-rs` crate) to fill chunks with terrain data.

---

## Phase 4: Basic Meshing (CPU Side)

**What you will learn:**

- **Vertex Formats:** Defining memory layouts manually (`wgpu::VertexBufferLayout`).
- **Face Culling:** The logic of only drawing sides of a block touching air.
- **Index Buffers:** Reducing vertex count by reusing vertices.

### Implementation Steps

1.  **Create Crate: `voxel-mesh`:**
    - Define a `Vertex` struct: `[position: vec3, tex_coords: vec2, normal: vec3]`.
    - Implement `bytemuck::Pod` and `Zeroable` for the vertex to safely cast it to bytes for GPU upload.

2.  **Naive Meshing:**
    - Iterate through every block in a `Chunk`.
    - For every block, generate 6 faces (2 triangles each).
    - _Result:_ A very heavy mesh.

3.  **Face Culling (Optimization 1):**
    - Update the loop: Check the neighbor block.
    - If the neighbor is solid, **do not** generate geometry for that face.
    - **Note:** You will need to handle chunk boundaries (checking neighbors in adjacent chunks).

4.  **Render the Mesh:**
    - Pass the generated `Vec<Vertex>` and `Vec<u32>` (indices) to `voxel-render`.
    - Create buffers: `BufferUsage::VERTEX` and `BufferUsage::INDEX`.
    - Issue the draw call: `render_pass.draw_indexed(...)`.

---

## Phase 5: Texturing & Depth Buffers

**What you will learn:**

- **Texture Arrays:** Storing all block textures in a single array texture to avoid switching bind groups.
- **Depth Stencil State:** Ensuring close objects cover far objects.
- **Z-Fighting mitigation:** Precision issues.

### Implementation Steps

1.  **Texture Loading:**
    - Load an image using the `image` crate.
    - Create a `wgpu::Texture` with multiple layers (Texture Array). layer 0 = dirt, layer 1 = grass, etc.

2.  **Sampler & Bind Group:**
    - Create a `Sampler` (nearest neighbor for that retro voxel look).
    - Update the `BindGroup` in `voxel-render` to include the texture view and sampler.

3.  **Depth Buffer:**
    - Create a depth texture (Format: `Depth32Float`).
    - Attach it to the `RenderPass` in the `depth_stencil_attachment` field.
    - _Observation:_ Cubes now look like cubes, not weirdly overlapping shapes.

---

## Phase 6: Multithreading (The Engine Core)

**What you will learn:**

- **Async/Await vs Thread Pools:** When to use which.
- **Arc & Mutex/RwLock:** Shared memory safety.
- **Channels:** Communicating between threads without locking the render loop.

### Implementation Steps

1.  **Architecture Change:**
    - Mesh generation is slow. If done in the main loop, the game freezes while loading chunks.
    - We need a **Task System**.

2.  **The Thread Pool (`voxel-world`):**
    - Integrate `rayon`.
    - Create a struct `ChunkTask`.
    - Use `crossbeam_channel` or `flume`.

3.  **The Flow:**
    - **Main Thread:** "I need chunk at (0,0,0)." -> Send request to channel.
    - **Worker Thread:** Receives request -> Generates Noise -> Generates Mesh -> Sends `Mesh` data back to Main Thread via result channel.
    - **Main Thread (Update Loop):** Check result channel. If mesh acts, upload to GPU.

4.  **Synchronization:**
    - Wrap the `World` data in `Arc<RwLock<World>>` if multiple threads need to read block data to generate meshes (for neighbor checking).

---

## Phase 7: Advanced Optimization (Greedy Meshing)

**What you will learn:**

- **Algorithmic Optimization:** Reducing VRAM usage and Vertex Shader load.
- **Data Compression:** merging adjacent faces.

### Implementation Steps

1.  **Refine `voxel-mesh`:**
    - Implement **Greedy Meshing**.
    - Instead of 1 block = 1 face, merge adjacent matching blocks into one large rectangle.
    - _Result:_ Massive reduction in triangle count (often 80%+ reduction for flat terrain).

---

## Phase 8: Polish & Infinite World

**What you will learn:**

- **Frustum Culling:** Don't process chunks behind the player.
- **Dynamic Loading/Unloading:** Managing memory.

### Implementation Steps

1.  **Chunk Loading Radius:**
    - In `voxel-engine`, track player position.
    - Calculate which chunk coordinates are within radius $R$.
    - Queue load requests for new chunks.
    - Queue drop requests for far chunks (free memory and destroy GPU buffers).

2.  **Frustum Culling:**
    - Extract the frustum planes from the Camera's View-Projection matrix.
    - Check AABB (Axis Aligned Bounding Box) of chunks against planes.
    - Filter out chunks from the `render()` list if they aren't visible.

---

## Final Project Structure Overview

```text
/voxel_workspace
  ├── Cargo.toml
  ├── /voxel-engine (bin)
  │    └── Main loop, Input handling, Glue code
  ├── /voxel-core (lib)
  │    └── Camera, Transform, Math utils
  ├── /voxel-world (lib)
  │    └── Chunk storage, Generation logic, Multithreading logic
  ├── /voxel-mesh (lib)
  │    └── Vertex definitions, Greedy meshing algorithms
  └── /voxel-render (lib)
       └── wgpu instance, pipelines, buffer management, texture loading
```

### Key Differences from OpenGL (Mental Check)

1.  **No Global State:** You must pass the `Device` and `Queue` into every function that needs to modify GPU memory.
2.  **Promises:** In wgpu, when you ask to map a buffer to read it back, it is async.
3.  **Validation:** wgpu crashes with helpful messages if your shader bindings don't match your Rust struct layout exactly.
