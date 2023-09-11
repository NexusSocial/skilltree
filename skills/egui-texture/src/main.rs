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
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins(bevy_flycam::PlayerPlugin)
		.add_plugins(bevy_egui::EguiPlugin)
		.insert_resource(AmbientLight {
			color: Color::WHITE,
			brightness: 1.,
		})
		.add_systems(Startup, setup)
		.add_systems(Update, ui_example_system)
		.add_systems(Update, update_egui_ui)
		.run();

	Ok(())
}

#[derive(Component)]
pub struct EguiContext {
	output_texture: Handle<Image>,
	renderer: egui_wgpu::Renderer,
	ctx: egui::Context,
	primitives: Vec<egui::ClippedPrimitive>,
}

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	mut images: ResMut<Assets<Image>>,
	device: Res<RenderDevice>,
) {
	let egui_thing = {
		let output_texture = Image {
			data: vec![255; 512 * 512 * 4],
			..default()
		};
		let output_texture_format = output_texture.texture_descriptor.format;
		let output_texture = images.add(output_texture);
		let renderer = egui_wgpu::Renderer::new(
			device.wgpu_device(),
			output_texture_format,
			None,
			1,
		);

		EguiContext {
			output_texture,
			renderer,
			ctx: egui::Context::default(),
			primitives: Vec::new(),
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
	device: Res<RenderDevice>,
	queue: Res<RenderQueue>,
	mut q: Query<&mut EguiContext>,
	mut redraw: EventWriter<bevy::window::RequestRedraw>,
) {
	for mut egui_ctx in q.iter_mut() {
		let egui_output = egui_ctx.ctx.run(egui::RawInput::default(), |ctx| {
			egui::Window::new("my window").show(ctx, |ui| ui.label("foobar"));
		});
		// TODO: Handle textures to delete
		for (tid, delta) in egui_output.textures_delta.set {
			egui_ctx.renderer.update_texture(
				device.wgpu_device(),
				queue.as_ref(),
				tid,
				&delta,
			);
		}
		egui_ctx.primitives = egui_ctx.ctx.tessellate(egui_output.shapes);
		redraw.send(bevy::window::RequestRedraw)
	}
}

fn render_egui_wgpu() {

	// let output_image = images
	// 	.get(&egui_thing.output_texture)
	// 	.expect("Should get output texture from handle");
	// let screen_descriptor = egui_wgpu::renderer::ScreenDescriptor {
	// 	pixels_per_point: 1.0,
	// 	size_in_pixels: [
	// 		output_image.texture_descriptor.size.width,
	// 		output_image.texture_descriptor.size.height,
	// 	],
	// };
	//
	// let mut encoder =
	// 	tuple
	// 		.0
	// 		.create_command_encoder(&wgpu::CommandEncoderDescriptor {
	// 			label: Some("Egui Render Encoder"),
	// 		});
	// egui_thing.renderer.update_buffers(
	// 	tuple.0.wgpu_device(),
	// 	tuple.1.as_ref(),
	// 	&mut encoder,
	// 	&egui_primitives,
	// 	&screen_descriptor,
	// );
	//
	// let mut egui_render_pass =
	// 	encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
	// 		label: Some("Egui Render Pass"),
	// 		color_attachments: &[Some(wgpu::RenderPassColorAttachment {
	// 			view: &output_gpu_image.texture_view,
	// 			resolve_target: None,
	// 			ops: wgpu::Operations {
	// 				load: wgpu::LoadOp::Clear(wgpu::Color {
	// 					r: 0.1,
	// 					g: 0.2,
	// 					b: 0.3,
	// 					a: 1.0,
	// 				}),
	// 				store: true,
	// 			},
	// 		})],
	// 		depth_stencil_attachment: None,
	// 	});
	//
	// egui_thing.renderer.render(
	// 	&mut egui_render_pass,
	// 	&egui_primitives,
	// 	&screen_descriptor,
	// );
	//
	// drop(egui_render_pass);
	// let commands = encoder.finish();
	// tuple.1.submit([commands]);
	// error!("After submit");
	// // output.present();
}
