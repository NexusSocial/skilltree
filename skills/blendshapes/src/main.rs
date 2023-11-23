use bevy::prelude::*;

/// The blendshape/morph target corresponding to a blink is at this index
const BLINK_MORPH_IDX: usize = 13;
const FACE_ENTITY_NAME: &str = "Face";
const ASSET_FOLDER: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/");

fn main() {
	// This will take like 5 seconds to load the gltf/vrm model, don't worry, just wait.
	App::new()
		.insert_resource(AmbientLight {
			color: Color::WHITE,
			brightness: 1.0 / 5.0f32,
		})
		.add_plugins(DefaultPlugins.set(AssetPlugin {
			file_path: ASSET_FOLDER.to_string(),
			..default()
		}))
		.add_systems(Startup, setup)
		.add_systems(Update, animate_blink)
		.run()
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
	commands
		.spawn(SpatialBundle::default())
		.with_children(|parent| {
			parent.spawn(Camera3dBundle {
				transform: Transform::from_xyz(0.0, 1.45, 0.5)
					.looking_at(Vec3::new(0.0, 1.45, 0.0), Vec3::Y),
				projection: bevy::render::camera::Projection::Perspective(
					PerspectiveProjection {
						fov: std::f32::consts::FRAC_PI_4,
						aspect_ratio: 1.0,
						near: 0.1,
						far: 100.0,
					},
				),
				..default()
			});
		});

	commands.spawn(SceneBundle {
		scene: assets.load("malek.gltf#Scene0"),
		transform: Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_euler(
			EulerRot::XYZ,
			0.0,
			180.0_f32.to_radians(),
			0.0,
		)),
		..default()
	});
}

fn animate_blink(mut morphs: Query<(&Name, &mut MorphWeights)>, time: Res<Time>) {
	let Some((_, mut weights)) = morphs
		.iter_mut()
		.find(|(name, _)| name.as_str() == FACE_ENTITY_NAME)
	else {
		return;
	};
	weights.weights_mut()[BLINK_MORPH_IDX] =
		(f32::sin(time.elapsed_seconds() * 10.0) + 1.) / 2.;
}
