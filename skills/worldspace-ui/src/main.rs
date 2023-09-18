mod render_node;
mod render_systems;

use bevy::prelude::*;
use bevy_egui::egui;
use bevy_egui::EguiContexts;
use color_eyre::Result;

fn main() -> Result<()> {
	color_eyre::install()?;
	let mut app = App::new();
	app.add_plugins(DefaultPlugins)
		.add_plugins(bevy_flycam::PlayerPlugin)
		.add_plugins(bevy_egui::EguiPlugin)
		.insert_resource(AmbientLight {
			color: Color::WHITE,
			brightness: 1.,
		})
		.add_systems(Startup, setup)
		.add_systems(Update, screenspace_ui);

	let Ok(render_app) = app.get_sub_app_mut(bevy::render::RenderApp) else {
		panic!("The render plugin should have added this subapp");
	};
	render_app.add_systems(ExtractSchedule, crate::render_systems::add_render_node);

	app.run();
	Ok(())
}

#[derive(Component, Clone)]
pub struct EguiContext {
	output_texture: Handle<Image>,
	ctx: egui::Context,
}

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	mut images: ResMut<Assets<Image>>,
) {
	let egui_thing = {
		let size = wgpu::Extent3d {
			width: 256,
			height: 256,
			depth_or_array_layers: 1,
		};
		let mut output_texture = Image {
			data: vec![255; (size.width * size.height * 4) as usize],
			..default()
		};
		output_texture.texture_descriptor.usage |=
			wgpu::TextureUsages::RENDER_ATTACHMENT;
		output_texture.texture_descriptor.size = size;
		let output_texture = images.add(output_texture);

		EguiContext {
			output_texture,
			ctx: egui::Context::default(),
		}
	};

	commands.spawn((
		PbrBundle {
			mesh: meshes.add(shape::Cube::default().into()),
			material: materials.add(StandardMaterial {
				base_color: Color::WHITE,
				base_color_texture: Some(Handle::clone(&egui_thing.output_texture)),
				..default()
			}),
			..default()
		},
		egui_thing,
	));
}

fn screenspace_ui(mut contexts: EguiContexts) {
	egui::Window::new("Screenspace Window").show(contexts.ctx_mut(), |ui| {
		ui.label("I am rendering to the screen!");
	});
}
