use std::sync::Mutex;

use bevy::{prelude::*, render::render_asset::RenderAssets};

use crate::EguiContext;

pub struct EguiNode {
	pub output_image: Handle<Image>,
	pub entity: Entity,
	pub renderer: Mutex<egui_wgpu::Renderer>,
}

impl bevy::render::render_graph::Node for EguiNode {
	// TODO: When should I use this instead of an Extract render system?
	fn update(&mut self, _world: &mut World) {}

	fn run(
		&self,
		_graph: &mut bevy::render::render_graph::RenderGraphContext,
		render_context: &mut bevy::render::renderer::RenderContext,
		world: &World,
	) -> Result<(), bevy::render::render_graph::NodeRunError> {
		let device = render_context.render_device().clone();
		let device = device.wgpu_device();
		let queue = world
			.get_resource::<bevy::render::renderer::RenderQueue>()
			.unwrap();
		let encoder = render_context.command_encoder();
		let gpu_images = world.get_resource::<RenderAssets<Image>>().unwrap();
		let output_gpu_image = gpu_images
			.get(&self.output_image)
			.expect("Should have been a `GpuImage` that corresponds to the `Image`");
		let screen_descriptor = egui_wgpu::renderer::ScreenDescriptor {
			pixels_per_point: 1.0,
			size_in_pixels: [
				output_gpu_image.texture.size().width,
				output_gpu_image.texture.size().height,
			],
		};

		let egui_ctx = world
			.get::<EguiContext>(self.entity)
			.expect("The entity should be valid");

		let mut renderer = self.renderer.lock().unwrap();
		// TODO: Handle textures to delete
		for (tid, delta) in egui_ctx.egui_output.textures_delta.set.iter() {
			renderer.update_texture(device, queue, *tid, delta);
		}

		renderer.update_buffers(
			device,
			queue,
			encoder,
			&egui_ctx.clipped_primitives,
			&screen_descriptor,
		);

		let mut egui_render_pass =
			encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
				label: Some("Egui Render Pass"),
				color_attachments: &[Some(wgpu::RenderPassColorAttachment {
					view: &output_gpu_image.texture_view,
					resolve_target: None,
					ops: wgpu::Operations {
						load: wgpu::LoadOp::Clear(wgpu::Color {
							r: 0.1,
							g: 0.2,
							b: 0.3,
							a: 1.0,
						}),
						store: true,
					},
				})],
				depth_stencil_attachment: None,
			});

		renderer.render(
			&mut egui_render_pass,
			&egui_ctx.clipped_primitives,
			&screen_descriptor,
		);
		//
		// drop(egui_render_pass);
		// let commands = encoder.finish();
		// tuple.1.submit([commands]);
		// error!("After submit");
		// // output.present();
		// todo!()
		Ok(())
	}
}
