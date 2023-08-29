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

- 🚧 [Single textured 3d cube](skills/cube)
- 📋 Flycam camera controller
- 📋 First person camera controller (for desktop users)
- 📋 Entity inspector (`bevy_inspector_egui`)
- 📋 Custom vertex and fragment shader in WGSL + bevy
- 📋 Render a GLTF model (standard shader)
- 📋 Render a VRM model (standard shader)
- 📋 Animate a skinned mesh with IK and FK
- 📋 Prove that custom vertex and fragment shaders on skinned meshes is possible
- 📋 MToon shader implemented in WGSL + bevy
- 📋 Render to side-by-side texture
- 📋 Display side-by-side textures as OpenVR overlay or stereokit window.
  This is probably laggy, but allows use of VR before OpenXR is ready in bevy.
- 📋 Spatial audio
- 📋 Animate a skinned mesh with blendshapes/morph targets
- 📋 Non-deformable physics
- 📋 Jiggle physics/Dynamic bones
- 📋 Cloth or hair physics
- 📋 Hot reload shaders (no lag spike)
- 📋 Hot reload skinned meshes (no lag spike)
- 📋 Hot reload scene (no lag spike)
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
- 📋 Explore auto atlasing, shader merging, shader registry (is it possible?)

## First time setup

There are a few options to get set up. We suggest Option 1.
Once you've done this first time setup, you can go read the README of any of
the skills/demos that interest you for instructions on how to run them.

### Option 1 (recommended): regular rust

- Install [rustup](https://rustup.rs)
- Install [bevy's dependencies](https://bevyengine.org/learn/book/getting-started/setup/#install-os-dependencies)
- Install [git lfs](https://git-lfs.com/) and run `git lfs install && git lfs pull`

### Option 2 (Linux and Mac only!): Nix package manager 

- Have the [nix](https://nixos.org/download) package manager installed 
- [Enable flakes](https://nixos.wiki/wiki/Flakes#Permanent)
- `nix profile install nixpkgs#direnv`
- [hook direnv into your shell](https://direnv.net/docs/hook.html)
- `cp .envrc.example .envrc` - feel free to customize this
- `direnv allow`
- `git lfs install && git lfs pull`

## License

Unless otherwise specified, all code in this repository is dual-licensed under
either:

- MIT License ([LICENSE-MIT](LICENSE-MIT))
- BSD 2-Clause Plus Patent License ([LICENSE-BSD](LICENSE-BSD))

at your option. This means you can select the license you prefer!

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be dual licensed as above, without any
additional terms or conditions.
