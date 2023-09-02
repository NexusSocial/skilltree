use bevy::{input::mouse::MouseMotion, prelude::*};

#[derive(Component)]
pub struct Flycam {
	/// The direction the camera is looking in
	pub direction: Vec3,
}

#[derive(Resource)]
struct FlycamSettings {
	sensitivity: f32,
}
impl Default for FlycamSettings {
	fn default() -> Self {
		Self { sensitivity: 0.001 }
	}
}

pub struct FlycamPlugin;

impl Plugin for FlycamPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(FlycamSettings::default())
			.add_systems(Update, rotate_camera);
	}
}

fn rotate_camera(
	mut motion_evr: EventReader<MouseMotion>,
	mut query: Query<(&mut Transform, &mut Flycam)>,
	settings: Res<FlycamSettings>,
) {
	let sens = -1. * settings.sensitivity;
	let delta: Vec2 = motion_evr.iter().map(|ev| ev.delta).sum();
	let rotation = Quat::from_euler(EulerRot::YXZ, delta.x * sens, delta.y * sens, 0.);
	for (mut trans, mut fly) in query.iter_mut() {
		fly.direction = rotation.mul_vec3(fly.direction);
		*trans = trans.looking_to(fly.direction, Vec3::Y);
	}
}
