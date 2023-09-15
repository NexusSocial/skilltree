use bevy::render::Extract;
use bevy::{prelude::*, render::render_graph::RenderGraph};

use crate::render_node::EguiNode;
use crate::EguiContext;

pub fn add_render_node(
	q: Extract<Query<(Entity, &EguiContext)>>,
	textures: Extract<Res<Assets<Image>>>,
	mut render_graph: ResMut<RenderGraph>,
	device: Res<bevy::render::renderer::RenderDevice>,
) {
	error!("add_render_node");
	for (entity, egui_ctx) in q.iter() {
		let output_texture_format = textures
			.get(&egui_ctx.output_texture)
			.expect("Should have found the matching `Image`")
			.texture_descriptor
			.format;
		let renderer = egui_wgpu::Renderer::new(
			device.wgpu_device(),
			output_texture_format,
			None,
			1,
		);
		let new_node = EguiNode {
			output_image: Handle::clone(&egui_ctx.output_texture),
			renderer: renderer.into(),
			entity,
		};
		let node_label = "egui-texture";
		render_graph.add_node(node_label, new_node);
		render_graph
			.add_node_edge(bevy::render::main_graph::node::CAMERA_DRIVER, node_label);
	}
}
