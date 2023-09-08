use bevy::core_pipeline::core_3d;
use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::ecs::prelude::{Bundle, Component, ReflectComponent};

use bevy::math::Mat4;
use bevy::prelude::Camera3d;
use bevy::reflect::{std_traits::ReflectDefault, Reflect};
use bevy::render::view::ColorGrading;
use bevy::render::{
	camera::{Camera, CameraProjection, CameraRenderGraph},
	primitives::Frustum,
	view::VisibleEntities,
};
use bevy::transform::components::{GlobalTransform, Transform};
//  mostly copied from https://github.com/blaind/bevy_openxr/tree/main/crates/bevy_openxr/src/render_graph/camera
use openxr::Fovf;

#[derive(Bundle)]
pub struct XrCameraBundle {
	pub camera: Camera,
	pub camera_render_graph: CameraRenderGraph,
	pub xr_projection: XRProjection,
	pub visible_entities: VisibleEntities,
	pub frustum: Frustum,
	pub transform: Transform,
	pub global_transform: GlobalTransform,
	pub camera_3d: Camera3d,
	pub tonemapping: Tonemapping,
	pub dither: DebandDither,
	pub color_grading: ColorGrading,
}

// NOTE: ideally Perspective and Orthographic defaults can share the same impl, but sadly it breaks rust's type inference
impl Default for XrCameraBundle {
	fn default() -> Self {
		Self {
			camera_render_graph: CameraRenderGraph::new(core_3d::graph::NAME),
			camera: Default::default(),
			xr_projection: Default::default(),
			visible_entities: Default::default(),
			frustum: Default::default(),
			transform: Default::default(),
			global_transform: Default::default(),
			camera_3d: Default::default(),
			tonemapping: Default::default(),
			dither: DebandDither::Enabled,
			color_grading: ColorGrading::default(),
		}
	}
}

#[derive(Debug, Clone, Component, Reflect)]
#[reflect(Component, Default)]
pub struct XRProjection {
	pub near: f32,
	pub far: f32,
	#[reflect(ignore)]
	pub fov: Fovf,
}

impl Default for XRProjection {
	fn default() -> Self {
		Self {
			near: 0.1,
			far: 1000.,
			fov: Default::default(),
		}
	}
}

impl XRProjection {
	pub fn new(near: f32, far: f32, fov: Fovf) -> Self {
		XRProjection { near, far, fov }
	}
}

impl CameraProjection for XRProjection {
	// =============================================================================
	// math code adapted from
	// https://github.com/KhronosGroup/OpenXR-SDK-Source/blob/master/src/common/xr_linear.h
	// Copyright (c) 2017 The Khronos Group Inc.
	// Copyright (c) 2016 Oculus VR, LLC.
	// SPDX-License-Identifier: Apache-2.0
	// =============================================================================
	fn get_projection_matrix(&self) -> Mat4 {
		//  symmetric perspective for debugging
		// let x_fov = (self.fov.angle_left.abs() + self.fov.angle_right.abs());
		// let y_fov = (self.fov.angle_up.abs() + self.fov.angle_down.abs());
		// return Mat4::perspective_infinite_reverse_rh(y_fov, x_fov / y_fov, self.near);

		let fov = self.fov;
		let is_vulkan_api = false; // FIXME wgpu probably abstracts this
		let near_z = self.near;
		let far_z = -1.; //   use infinite proj
				 // let far_z = self.far;

		let tan_angle_left = fov.angle_left.tan();
		let tan_angle_right = fov.angle_right.tan();

		let tan_angle_down = fov.angle_down.tan();
		let tan_angle_up = fov.angle_up.tan();

		let tan_angle_width = tan_angle_right - tan_angle_left;

		// Set to tanAngleDown - tanAngleUp for a clip space with positive Y
		// down (Vulkan). Set to tanAngleUp - tanAngleDown for a clip space with
		// positive Y up (OpenGL / D3D / Metal).
		// const float tanAngleHeight =
		//     graphicsApi == GRAPHICS_VULKAN ? (tanAngleDown - tanAngleUp) : (tanAngleUp - tanAngleDown);
		let tan_angle_height = if is_vulkan_api {
			tan_angle_down - tan_angle_up
		} else {
			tan_angle_up - tan_angle_down
		};

		// Set to nearZ for a [-1,1] Z clip space (OpenGL / OpenGL ES).
		// Set to zero for a [0,1] Z clip space (Vulkan / D3D / Metal).
		// const float offsetZ =
		//     (graphicsApi == GRAPHICS_OPENGL || graphicsApi == GRAPHICS_OPENGL_ES) ? nearZ : 0;
		// FIXME handle enum of graphics apis
		let offset_z = 0.;

		let mut cols: [f32; 16] = [0.0; 16];

		if far_z <= near_z {
			// place the far plane at infinity
			cols[0] = 2. / tan_angle_width;
			cols[4] = 0.;
			cols[8] = (tan_angle_right + tan_angle_left) / tan_angle_width;
			cols[12] = 0.;

			cols[1] = 0.;
			cols[5] = 2. / tan_angle_height;
			cols[9] = (tan_angle_up + tan_angle_down) / tan_angle_height;
			cols[13] = 0.;

			cols[2] = 0.;
			cols[6] = 0.;
			cols[10] = -1.;
			cols[14] = -(near_z + offset_z);

			cols[3] = 0.;
			cols[7] = 0.;
			cols[11] = -1.;
			cols[15] = 0.;

			//  bevy uses the _reverse_ infinite projection
			//  https://dev.theomader.com/depth-precision/
			let z_reversal = Mat4::from_cols_array_2d(&[
				[1f32, 0., 0., 0.],
				[0., 1., 0., 0.],
				[0., 0., -1., 0.],
				[0., 0., 1., 1.],
			]);

			return z_reversal * Mat4::from_cols_array(&cols);
		} else {
			// normal projection
			cols[0] = 2. / tan_angle_width;
			cols[4] = 0.;
			cols[8] = (tan_angle_right + tan_angle_left) / tan_angle_width;
			cols[12] = 0.;

			cols[1] = 0.;
			cols[5] = 2. / tan_angle_height;
			cols[9] = (tan_angle_up + tan_angle_down) / tan_angle_height;
			cols[13] = 0.;

			cols[2] = 0.;
			cols[6] = 0.;
			cols[10] = -(far_z + offset_z) / (far_z - near_z);
			cols[14] = -(far_z * (near_z + offset_z)) / (far_z - near_z);

			cols[3] = 0.;
			cols[7] = 0.;
			cols[11] = -1.;
			cols[15] = 0.;
		}

		Mat4::from_cols_array(&cols)
	}

	fn update(&mut self, _width: f32, _height: f32) {}

	fn far(&self) -> f32 {
		self.far
	}
}
