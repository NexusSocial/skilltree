mod render_node;
mod render_systems;

use bevy::prelude::*;
use bevy::render::render_asset::RenderAsset;
use bevy::render::render_asset::RenderAssets;
use bevy::render::renderer::RenderDevice;
use bevy::render::renderer::RenderQueue;
use bevy::render::texture::DefaultImageSampler;
use bevy::render::texture::GpuImage;
use bevy_egui::egui;
use bevy_egui::EguiContexts;
use color_eyre::Result;
use wgpu::util::RenderEncoder;

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
		.add_systems(Update, ui_example_system)
		.add_systems(Update, update_egui_ui);

	let Ok(render_app) = app.get_sub_app_mut(bevy::render::RenderApp) else {
		panic!("The render plugin should have added this subapp");
	};
	render_app.add_systems(ExtractSchedule, crate::render_systems::add_render_node);

	app.run();
	Ok(())
}

#[derive(Component)]
pub struct EguiContext {
	output_texture: Handle<Image>,
	ctx: egui::Context,
	egui_output: egui::FullOutput,
	clipped_primitives: Vec<egui::ClippedPrimitive>,
}

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	mut images: ResMut<Assets<Image>>,
) {
	let egui_thing = {
		let output_texture = Image {
			data: vec![255; 512 * 512 * 4],
			..default()
		};
		let output_texture = images.add(output_texture);

		EguiContext {
			output_texture,
			ctx: egui::Context::default(),
			egui_output: default(),
			clipped_primitives: Vec::new(),
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

fn ui_example_system(mut contexts: EguiContexts) {
	egui::Window::new("Regular 2D screenspace window").show(contexts.ctx_mut(), |ui| {
		ui.label("I am in screenspace!");
	});
}

/// Performs the egui ui draw, updates textures, and generates the primitives.
///
/// Still need to actually perform the render encoder commands
fn update_egui_ui(
	mut q: Query<&mut EguiContext>,
	mut redraw: EventWriter<bevy::window::RequestRedraw>,
) {
	for mut egui_ctx in q.iter_mut() {
		let mut egui_output = egui_ctx.ctx.run(egui::RawInput::default(), |ctx| {
			egui::Window::new("my window").show(ctx, |ui| ui.label("foobar"));
		});
		let shapes = std::mem::take(&mut egui_output.shapes);
		egui_ctx.egui_output = egui_output;
		egui_ctx.clipped_primitives = egui_ctx.ctx.tessellate(shapes);
		redraw.send(bevy::window::RequestRedraw)
	}
}
