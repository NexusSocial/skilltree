# The Skilltree

A collection of experiments and demos for building social VR with bevy and wgpu. 

Rome wasn't built in a day - likewise a Rust social VR game will take some time
and ecosystem maturity to build. This repo can be used to track Rust and bevy's
"skilltree" in social VR and provide a single place to provide clear working
examples and demos for all the things that are necessary for social VR.

Eventually when the skilltree is fleshed out enough, we will create a new
monorepo specifically for building a full fledged game.

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

### Skill List

- 📋 [Single textured 3d cube](skills/cube)
- 📋 Flycam camera controller
- 📋 Entity inspector
- 📋 Loading a GLTF model
- 📋 Loading a GLTF model (standard shader)
- 📋 Animating a skinned mesh with IK and FK
- 📋 MToon shader in WGSL + bevy
- 📋 Side by side render to texture
- 📋 Spatial audio
- ❌ OpenXR render to headset (0dof)
- ❌ OpenXR 6dof headset and controller tracking
- ❌ OpenXR Vive trackers (`XR_HTCX_vive_tracker_interaction`)
- Feel free to add to this list!

### Demo List

- No demos planned until we have the skills fleshed out more.

## License

Unless otherwise specified, all code in this repository is dual-licensed under
either:

- MIT License ([LICENSE-MIT](LICENSE-MIT))
- BSD 2-Clause Plus Patent License ([LICENSE-BSD](LICENSE-BSD))

at your option. This means you can select the license you prefer!

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be dual licensed as above, without any
additional terms or conditions.
