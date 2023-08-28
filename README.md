# The Skilltree

A collection of experiments and demos for building social VR with bevy and wgpu. 

Rome wasn't built in a day - likewise a Rust social VR game will take some time
and ecosystem maturity to build. This repo can be used to track Rust and bevy's
"skilltree" in social VR. We will provide a collection of example and demo code
that will serve to demonstrate what is and isn't possible right now in the
ecosystem.

Eventually when the skilltree is fleshed out enough, we will take this exporatory
work and translate it into a full fledged game.

## Repo Structure

- **Skills**: Single isolated example of a specific technical feat - like doing
  IK, or loading a VRM. See [`skills`](/skills).
- **Demos**: Integrates multiple skills into a larger cohesive tech demo. See
  [`demos`](/demos).

Both demos and skills are presented as binary crates that you can actually run
and try out yourself.

## Project Status

Legend:
- ✅ = Finished
- 🚧 = Active WIP
- 💩 = Outdated/Broken
- 📋 = Planned
- ❌ = Blocked

Feel free to add to the following lists:

### Skill List

- 📋 [Single textured 3d cube](skills/cube)
- 📋 Flycam camera controller
- 📋 First person camera controller (for desktop users)
- 📋 Entity inspector
- 📋 Loading a GLTF model (standard shader)
- 📋 Loading a VRM model (standard shader)
- 📋 Animating a skinned mesh with IK and FK
- 📋 MToon shader implemented in WGSL + bevy
- 📋 Side by side render to texture
- 📋 Spatial audio
- 📋 Animating a skinned mesh with blendshapes/morph targets
- 📋 Non-deformable physics
- 📋 Jiggle physics/Dynamic bones
- 📋 Cloth or hair physics
- 📋 Hot reloading of shaders
- 📋 Hot reloading of skinned meshes
- 📋 Hot reloading of scene
- 📋 Dynamic scaling of resolution
- 📋 Touch controls for flat in-game surfaces
- 📋 Dynamic foveated rendering
- 📋 Get tracked positions from SlimeVR/SolarXR
- ❌ OpenXR render to headset (0dof)
- ❌ OpenXR 6dof headset and controller tracking
- ❌ OpenXR Vive trackers (`XR_HTCX_vive_tracker_interaction`)

Note: The OpenXR stuff should be unblocked soon, there are community
crates being developed to add openxr to bevy right now. See also

### Demo List

- 📋 Comparison of different bevy networking libraries for shared state
- 📋 Create a VOIP API, abstract it over transport, and compare latency
  of different web-compatible transports (probably WebRTC, WebTransport, WebSocket)
- 📋 Auto atlasing, shader registry, and shader merging across all avatars in the
  world




## License

Unless otherwise specified, all code in this repository is dual-licensed under
either:

- MIT License ([LICENSE-MIT](LICENSE-MIT))
- BSD 2-Clause Plus Patent License ([LICENSE-BSD](LICENSE-BSD))

at your option. This means you can select the license you prefer!

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be dual licensed as above, without any
additional terms or conditions.
