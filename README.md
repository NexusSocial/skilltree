# The Skilltree

A collection of experiments and demos for building social VR with bevy and wgpu. 

## Repo Structure

Social VR requires many things - on top of all the regular 3D game programming,
it has additional requirements unique to social VR. Rome wasn't built in a day,
and neither can a Social VR game.

The repository consistes of "skills" which represent a single isolated example
for achieving a specifc technical goal - like doing IK, or loading a VRM. These
aree located in the `skills` folder. These try to do exactly 1 thing.

There are also `demos`, which integrate multiple of these skills into a larger
unified tech demo.

Eventually when the skilltree is fleshed out enough, we will create a new
monorepo specifically for building a full fledged game.

### Skills

* 🚧 [Basic textured cube](skills/cube)

### Demos

* Nothing yet

## License

Unless otherwise specified, all code in this repository is dual-licensed under
either:

- MIT License ([LICENSE-MIT](LICENSE-MIT))
- BSD 2-Clause Plus Patent License ([LICENSE-BSD](LICENSE-BSD))

at your option. This means you can select the license you prefer!

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be dual licensed as above, without any
additional terms or conditions.
