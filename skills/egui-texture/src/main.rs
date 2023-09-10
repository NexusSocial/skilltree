use bevy::prelude::*;
use bevy_egui::egui;
use bevy_egui::EguiContexts;
use color_eyre::Result;

fn main() -> Result<()> {
	color_eyre::install()?;
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins(bevy_flycam::PlayerPlugin)
		.add_plugins(bevy_egui::EguiPlugin)
		.insert_resource(AmbientLight {
			color: Color::WHITE,
			brightness: 1. / 4.,
		})
		.add_systems(Startup, setup)
		.add_systems(Update, ui_example_system)
		.run();

	Ok(())
}

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	// Build cube
	commands.spawn(PbrBundle {
		mesh: meshes.add(shape::Cube::default().into()),
		material: materials.add(StandardMaterial {
			base_color: Color::GREEN,
			..default()
		}),
		..default()
	});
}

fn ui_example_system(mut contexts: EguiContexts) {
	egui::Window::new("Regular 2D screenspace window").show(contexts.ctx_mut(), |ui| {
		ui.label("I am in screenspace!");
	});
}
