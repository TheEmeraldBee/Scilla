/*
Project Name
roe-engine

---

Project Goals:
Primary focus on multithreadding, Create a threaded game system with a single derive macro!
Lightweight But Powerful:
    - Keep large features behind flags.
    - Octree-based Voxel Rendering (Main Goal)
    - Raytraced Realtime Lighting

---

Project Layout:
This crate contains examples, the crate prelude, and
misc tests that don't belong in any specific feature

- Main Crate
    - voxel crate
        - Contains all definitions of voxels, and the octree implementations.
    - window crate
        - Contains abstractions from winit to a custom app trait built for paralelism.
    - render crate
        - Contains Renderer definitions, and abstractions from wgpu to improve rendering
        - Also Contains all default shaders that will render voxels using raytracing
        - Probably Split this crate eventually, as it will probably be the biggest one.
    - world crate
        - Contains world and all of it's contained methods to quickly generate voxel worlds.
    - thread crate
        - Contains threaded definitions to improve world access and use.
*/
