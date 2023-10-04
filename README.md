# The Skilltree

A collection of experiments and demos for building social VR with bevy and wgpu. 

Rome wasn't built in a day - likewise a Rust social VR game will take some time
and ecosystem maturity to build. This repo can be used to track Rust and bevy's
"skilltree" in social VR. We will provide a collection of example and demo code
that will serve to demonstrate what is and isn't possible right now in the
ecosystem.

Eventually when the skilltree is fleshed out enough, we will take this exploratory
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

Additionally, **skills marked with 🫵 are easy for beginners**. 
**skills marked 🥺 are pretty hard** 👉👈

### Skills
These are technical features are needed for a Social VR game. They are written as tiny demos of a single *thing*.

#### General
The skills in this section are general and often can be accomplished by piecing together existing
plugins/example code on the internet.

- 🫵 📋 Flycam camera controller
- 🫵 📋 Entity inspector (`bevy_inspector_egui`)
- 🫵 ✅ [Single textured 3d cube](skills/cube)
- ✅ [Import a GLTF/VRM model (standard shader)](skills/ik)
- ✅ [Animate a skinned mesh with IK and FK](skills/ik)
- 📋 First person camera controller (for desktop users)
- ❌ Hot reload scene (no lag spike) (blocked on bevy supporting hot reload for assets)
- ❌ Hot reload avatar and shaders (no lag spike) (blocked, see above)

#### UI/UX Design
The skills in this section are related to playing with UI/UX designs. VR is typically not required for working on these.

- 🫵 📋 Laser pointer visuals (you can work on this without VR)
- 🫵 🚧 World-space gizmos for resizing, rotating, and moving objects and viewing bounding box 
- 📋 Friends list menu
- 📋 Avatar list menu
- 📋 Active game worlds
- 📋 Invite notification and Invite accept
- ✅ [Rotating and resize flatscreen](skills/manipulation-flatscreen)
- 🥺 📋 Input scheme useful for social VR games that abstracts over mouse & keyboard, gamepad, VR Controllers,
  and hand tracking (likely a superset of `bevy_mod_picking`).

#### Needs VR
The skills in this section are vr-specific and distinct from a regular flatscreen game. You will likely
need VR to work on these

- ✅ [OpenXR 6dof headset and controller tracking](skills/openxr-6dof)
- 📋 Plugin to animate transform using 6dof data from OpenXR Vive trackers (`XR_HTCX_vive_tracker_interaction`)
- 📋 Plugin to animate transform using 6dof data from SlimeVR/SolarXR
- 📋 Add VR controllers as input method to `bevy_mod_picking`

#### Rendering
The skills in this section are all related to 3D rendering and go a bit deeper than just
using an existing plugin. VR is not necessary.

- 🫵 📋 Custom vertex and fragment shader in WGSL + bevy
- 📋 Dynamic foveated rendering
- 📋 Dynamic scaling of resolution
- 🥺 🚧 [World-space UI](skills/worldspace-ui) (with egui)
- 🥺 🚧 [Mirror](skills/xr-ik-mirror)
- 🥺 📋 Plugin for MToon shaders implemented in WGSL + bevy


#### Math, Physics, Animation
The skills in this section are all focused on math heavy parts of 3D game engines. VR is not necessary.

- 🫵 📋 Non-deformable collision physics
- 🚧 [VR Inverse Kinematics](skills/xr-ik-mirror)
- 📋 Jiggle physics/Dynamic bones
- 📋 Animate blendshapes/morph targets on a skinned mesh
- 🥺 ❌ Cloth or hair physics (blocked on a physics engine that does this)

#### Audio
- 🫵 📋 Spatial audio (bevy already supports this, just show how to use it)

### Demo List

- 📋 Comparison of different bevy networking libraries for shared state
- 📋 Create a VOIP API, abstract it over transport, and compare latency
  of different web-compatible transports (probably WebRTC, WebTransport, WebSocket)
- 📋 Explore auto atlasing, shader merging, shader registry (is it possible?)
- [Leapmotion blocks] physics demo with interactions

## First time setup

There are a few options to get set up. We suggest Option 1.
Once you've done this first time setup, you can go read the README of any of
the skills/demos that interest you for instructions on how to run them.

### Option 1 (recommended): Regular rust

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

## Contributions

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Unless otherwise specified, all code in this repository is dual-licensed under
either:

- MIT License ([LICENSE-MIT](LICENSE-MIT))
- BSD 2-Clause Plus Patent License ([LICENSE-BSD](LICENSE-BSD))

at your option. This means you can select the license you prefer!

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be dual licensed as above, without any
additional terms or conditions.

[Leapmotion blocks]: https://www.youtube.com/watch?v=oZ_53T2jBGg&pp=ygURbGVhcG1vdGlvbiBibG9ja3M%3D
