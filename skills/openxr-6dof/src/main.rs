//! A simple 3D scene with light shining over a cube sitting on a plane.
mod projection;

use crate::projection::{XRProjection, XrCameraBundle};
use bevy::prelude::Component;
use bevy::render::camera::{CameraProjectionPlugin, Viewport};
use bevy::render::view::{update_frusta, VisibilitySystems};
use bevy::transform::TransformSystem;
use bevy::{prelude::*, render::camera::RenderTarget};
use bevy_openxr::input::XrInput;
use bevy_openxr::resources::{XrFrameState, XrInstance, XrSession, XrViews};
use bevy_openxr::{DefaultXrPlugins, LEFT_XR_TEXTURE_HANDLE, RIGHT_XR_TEXTURE_HANDLE};
use openxr::{ActiveActionSet, FrameState, Path, Posef, Session};
use std::ops::Deref;
use std::sync::MutexGuard;

fn main() {
	color_eyre::install().unwrap();

	info!("Running `openxr-6dof` skill");
	App::new()
		.add_plugins(DefaultXrPlugins)
		.add_plugins(CameraProjectionPlugin::<XRProjection>::default())
		.add_systems(Startup, setup)
		.add_systems(PreUpdate, head_movement)
		.add_systems(PreUpdate, hands)
		.add_systems(
			PostUpdate,
			update_frusta::<XRProjection>
				.after(TransformSystem::TransformPropagate)
				.before(VisibilitySystems::UpdatePerspectiveFrusta),
		)
		.run();
}

#[derive(Component)]
enum CameraType {
	Left,
	Right,
	Middle,
}

/// set up a simple 3D scene
fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	// plane
	commands.spawn(PbrBundle {
		mesh: meshes.add(shape::Plane::from_size(5.0).into()),
		material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
		..default()
	});
	// cube
	commands.spawn(PbrBundle {
		mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
		material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
		transform: Transform::from_xyz(0.0, 0.5, 0.0),
		..default()
	});
	// light
	commands.spawn(PointLightBundle {
		point_light: PointLight {
			intensity: 1500.0,
			shadows_enabled: true,
			..default()
		},
		transform: Transform::from_xyz(4.0, 8.0, 4.0),
		..default()
	});
	// camera
	commands.spawn((
		Camera3dBundle {
			transform: Transform::from_xyz(-2.0, 2.5, 5.0)
				.looking_at(Vec3::ZERO, Vec3::Y),
			..default()
		},
		CameraType::Middle,
	));

	commands.spawn((
		XrCameraBundle {
			transform: Transform::from_xyz(-2.0, 2.5, 5.0)
				.looking_at(Vec3::ZERO, Vec3::Y),
			camera: Camera {
				order: -1,
				target: RenderTarget::TextureView(LEFT_XR_TEXTURE_HANDLE),
				viewport: None,
				..default()
			},
			..default()
		},
		CameraType::Left,
	));
	commands.spawn((
		XrCameraBundle {
			transform: Transform::from_xyz(-2.0, 2.5, 5.0)
				.looking_at(Vec3::ZERO, Vec3::Y),
			camera: Camera {
				order: -1,
				target: RenderTarget::TextureView(RIGHT_XR_TEXTURE_HANDLE),
				viewport: None,
				..default()
			},
			..default()
		},
		CameraType::Right,
	));
}

struct HandState {}

fn hands(
	mut gizmos: Gizmos,
	xr_input: Res<XrInput>,
	session: Res<XrSession>,
	frame_state: Res<XrFrameState>,
) {
	//let pose = xr_input.left_action.create_space(Session::clone(&session), Path, Posef::IDENTITY).unwrap();
	let act = ActiveActionSet::new(&xr_input.action_set);
	session.sync_actions(&[act]).unwrap();
	frame_state.lock().unwrap().map(|a| {
		//let b = pose.locate(&*xr_input.stage, a.predicted_display_time).unwrap();
		let b = xr_input
			.left_space
			.relate(&*xr_input.stage, a.predicted_display_time)
			.unwrap();
		gizmos.rect(
			b.0.pose.position.to_vec3(),
			b.0.pose.orientation.to_quat(),
			Vec2::new(0.05, 0.2),
			Color::YELLOW_GREEN,
		);
		let c = xr_input
			.right_space
			.relate(&*xr_input.stage, a.predicted_display_time)
			.unwrap();
		gizmos.rect(
			c.0.pose.position.to_vec3(),
			c.0.pose.orientation.to_quat(),
			Vec2::new(0.05, 0.2),
			Color::YELLOW_GREEN,
		)
	});
}

fn head_movement(
	views: ResMut<XrViews>,
	mut query: Query<(&mut Transform, &mut Camera, &CameraType, &mut XRProjection)>,
) {
	let views = views.lock().unwrap();
	let mut f = || -> Option<()> {
		let midpoint = (views.get(0)?.pose.position.to_vec3()
			+ views.get(1)?.pose.position.to_vec3())
			/ 2.;
		for (mut t, _, camera_type, _) in query.iter_mut() {
			match camera_type {
				CameraType::Left => {
					t.translation = views.get(0)?.pose.position.to_vec3()
				}
				CameraType::Right => {
					t.translation = views.get(1)?.pose.position.to_vec3()
				}
				CameraType::Middle => {
					t.translation = midpoint;
				}
			}
		}
		let left_rot = views.get(0).unwrap().pose.orientation.to_quat();
		let right_rot = views.get(1).unwrap().pose.orientation.to_quat();
		let mid_rot = if left_rot.dot(right_rot) >= 0. {
			left_rot.slerp(right_rot, 0.5)
		} else {
			right_rot.slerp(left_rot, 0.5)
		};
		for (mut t, _, camera_type, _) in query.iter_mut() {
			match camera_type {
				CameraType::Left => t.rotation = left_rot,
				CameraType::Right => t.rotation = right_rot,
				CameraType::Middle => {
					t.rotation = mid_rot;
				}
			}
		}

		for (mut transform, mut cam, camera_type, mut xr_projection) in query.iter_mut()
		{
			let view_idx = match camera_type {
				CameraType::Left => 0,
				CameraType::Right => 1,
				CameraType::Middle => panic!(),
			};
			let view = views.get(view_idx).unwrap();
			xr_projection.fov = view.fov;

			transform.rotation = view.pose.orientation.to_quat();
			let pos = view.pose.position;
			transform.translation = pos.to_vec3();
		}

		Some(())
	};
	f();
}
pub trait Vec3Conv {
	fn to_vec3(&self) -> Vec3;
}

impl Vec3Conv for openxr::Vector3f {
	fn to_vec3(&self) -> Vec3 {
		Vec3::new(self.x, self.y, self.z)
	}
}
pub trait QuatConv {
	fn to_quat(&self) -> Quat;
}

impl QuatConv for openxr::Quaternionf {
	fn to_quat(&self) -> Quat {
		Quat::from_xyzw(self.x, self.y, self.z, self.w)
	}
}
