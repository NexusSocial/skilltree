use bevy::{pbr::DirectionalLightShadowMap, prelude::*};
use color_eyre::eyre::Result;
use tracing::info;

const ASSET_FOLDER: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/");

fn main() -> Result<()> {
	// Set up nice error messages
	color_eyre::install()?;

	info!("Running `cube` skill");

	App::new()
		.add_plugins(DefaultPlugins.set(AssetPlugin {
			asset_folder: ASSET_FOLDER.to_string(),
			..Default::default()
		}))
		.add_systems(Startup, setup)
		.add_systems(Update, animate_light)
		.run();

	Ok(())
}

fn setup(
	assets: Res<AssetServer>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	mut commands: Commands,
) {
	info!("Running setup system");

	// Load assets
	let tree_img: Handle<Image> = assets.load("tree.png");

	// Build cube
	commands.spawn(PbrBundle {
		mesh: meshes.add(shape::Cube::default().into()),
		material: materials.add(StandardMaterial {
			base_color_texture: Some(tree_img),
			..default()
		}),
		..default()
	});

	// Build the rest of the scene
	commands.spawn(DirectionalLightBundle {
		directional_light: DirectionalLight {
			shadows_enabled: true,
			illuminance: 10000.,
			..default()
		},
		transform: Transform::from_xyz(8.0, 16.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
		..default()
	});
	commands.insert_resource(DirectionalLightShadowMap { size: 4096 });
	commands.spawn(Camera3dBundle {
		transform: Transform::from_xyz(0.0, 6., 12.0)
			.looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
		..default()
	});
	commands.spawn(PbrBundle {
		mesh: meshes.add(
			shape::Plane {
				size: 10.,
				subdivisions: 4,
			}
			.into(),
		),
		material: materials.add(Color::MIDNIGHT_BLUE.into()),
		transform: Transform::from_xyz(0., 0., 0.),
		..default()
	});
}

fn animate_light(mut query: Query<&mut Transform, With<DirectionalLight>>) {
	for mut t in query.iter_mut() {
		let t: &mut Transform = &mut t;
		t.rotate_y(0.01);
	}
}
