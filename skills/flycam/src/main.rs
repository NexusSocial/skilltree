use bevy::log::info;
use bevy::prelude::*;
use color_eyre::Result;
use flycam::{Flycam, FlycamPlugin};

fn main() -> Result<()> {
	// Better error reporting
	color_eyre::install()?;
	info!("Running `flycam` skill");

	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins(FlycamPlugin)
		.insert_resource(AmbientLight {
			color: Color::WHITE,
			brightness: 1.0 / 5.0f32,
		})
		.add_systems(Startup, setup)
		.run();

	Ok(())
}

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let cam_pos = Vec3::new(-0.5, 1.5, 2.5);
	let cam_target = Vec3::new(0., 1., 0.);
	commands
		.spawn(SpatialBundle::default())
		.with_children(|parent| {
			parent.spawn((
				Camera3dBundle {
					transform: Transform::from_translation(cam_pos),
					projection: bevy::render::camera::Projection::Perspective(
						PerspectiveProjection {
							fov: std::f32::consts::FRAC_PI_4,
							aspect_ratio: 1.0,
							near: 0.1,
							far: 100.0,
						},
					),
					..default()
				},
				Flycam {
					direction: cam_target - cam_pos,
				},
			));
		});

	commands.spawn(PbrBundle {
		mesh: meshes.add(Mesh::from(shape::Plane {
			size: 5.0,
			subdivisions: 0,
		})),
		material: materials.add(StandardMaterial {
			base_color: Color::WHITE,
			..default()
		}),
		..default()
	});
}
