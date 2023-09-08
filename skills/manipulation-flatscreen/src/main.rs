use bevy::ecs::system::EntityCommands;
use bevy::{pbr::DirectionalLightShadowMap, prelude::*};
use bevy_mod_picking::events::Drag;
use bevy_mod_picking::pointer::PointerButton;
use bevy_mod_picking::prelude::{On, Pointer, RaycastPickCamera, RaycastPickTarget};
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle};
use color_eyre::eyre::Result;
use tracing::info;

const ASSET_FOLDER: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/");

fn main() -> Result<()> {
	// Set up nice error messages
	color_eyre::install()?;

	info!("Running `manipulation-flatscreen` skill");

	App::new()
		.add_plugins(DefaultPlugins.set(AssetPlugin {
			asset_folder: ASSET_FOLDER.to_string(),
			..Default::default()
		}))
		.add_plugins(DefaultPickingPlugins)
		.add_systems(Startup, setup)
		.add_systems(Update, animate_light)
		.run();

	Ok(())
}

pub fn add_events(entity_commands: &mut EntityCommands) {
	entity_commands.insert((
		On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
			let delta_y = drag.delta.y;
			let mut delta_x = drag.delta.x;
			match drag.button {
				PointerButton::Primary => {
					if drag.pointer_location.position.y < transform.translation.y {
						delta_x *= -1.0;
					}

					let axis_of_rotation =
						Vec3::new(delta_y, delta_x, 0.0).normalize_or_zero();

					// Compute the magnitude of rotation. You can scale this to adjust rotation speed
					let rotation_magnitude =
						(delta_x.powi(2) + delta_y.powi(2)).sqrt() * 0.01;

					// Create quaternion from axis-angle representation
					let rotation_quat =
						Quat::from_axis_angle(axis_of_rotation, rotation_magnitude);
					transform.rotation = rotation_quat * transform.rotation;
				}
				PointerButton::Secondary => {
					transform.scale += Vec3::splat((delta_x + delta_y) * 0.01);
				}
				PointerButton::Middle => {
					transform.rotate_local_y((delta_y + delta_x) * 0.01);
				}
			}
		}),
		PickableBundle::default(), // Makes the entity pickable
		RaycastPickTarget::default(),
	));
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
	let mut cube = commands.spawn(PbrBundle {
		mesh: meshes.add(shape::Cube::default().into()),
		material: materials.add(StandardMaterial {
			base_color_texture: Some(tree_img),
			..default()
		}),
		..default()
	});
	add_events(&mut cube);

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
	commands.spawn((
		Camera3dBundle {
			transform: Transform::from_xyz(0.0, 6., 12.0)
				.looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
			..default()
		},
		RaycastPickCamera::default(),
	));
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
