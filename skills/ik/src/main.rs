use bevy::pbr::DirectionalLightShadowMap;
use bevy::prelude::*;
use bevy_mod_inverse_kinematics::{IkConstraint, InverseKinematicsPlugin};
use std::f32::consts::*;

fn main() {
	// This will take like 5 seconds to load the gltf/vrm model, don't worry, just wait.
	App::new()
		.insert_resource(AmbientLight {
			color: Color::WHITE,
			brightness: 1.0 / 5.0f32,
		})
		.insert_resource(DirectionalLightShadowMap { size: 4096 })
		.add_plugins(DefaultPlugins)
		.add_plugins(InverseKinematicsPlugin)
		.add_systems(Startup, setup)
		.add_systems(Update, animate_light_direction)
		.add_systems(Update, (setup_ik, manually_target))
		.run();
}

#[derive(Component)]
struct Thing;

#[derive(Component)]
pub struct ManuallyTarget(Vec4);

fn setup(
	mut commands: Commands,
	assets: Res<AssetServer>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	commands
		.spawn(SpatialBundle::default())
		.with_children(|parent| {
			parent.spawn(Camera3dBundle {
				transform: Transform::from_xyz(-0.5, 1.5, 2.5)
					.looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
				projection: bevy::render::camera::Projection::Perspective(
					PerspectiveProjection {
						fov: std::f32::consts::FRAC_PI_4,
						aspect_ratio: 1.0,
						near: 0.1,
						far: 100.0,
					},
				),
				..default()
			});
		});

	commands.spawn(DirectionalLightBundle {
		directional_light: DirectionalLight {
			color: Color::WHITE,
			illuminance: 10000.0,
			shadows_enabled: true,
			..default()
		},
		transform: Transform::from_xyz(-8.0, 8.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
		..default()
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

	commands.spawn((
		SceneBundle {
			scene: assets.load("malek.gltf#Scene0"),
			transform: Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(
				Quat::from_euler(EulerRot::XYZ, 0.0, 180.0_f32.to_radians(), 0.0),
			),
			..default()
		},
		Thing,
	));
}

fn animate_light_direction(
	time: Res<Time>,
	mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
	for mut transform in &mut query {
		transform.rotation = Quat::from_euler(
			EulerRot::ZYX,
			0.0,
			time.elapsed_seconds() * PI / 5.0,
			-FRAC_PI_4,
		);
	}
}

fn setup_ik(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	added_query: Query<(Entity, &Thing)>,
	children: Query<&Children>,
	names: Query<&Name>,
) {
	for (entity, _thing) in added_query.iter() {
		let mut right_hand = None;
		// Try to get the entity for the right hand joint.
		for child in children.iter_descendants(entity) {
			if let Ok(name) = names.get(child) {
				if name.as_str() == "J_Bip_R_Hand" {
					right_hand.replace(child);
					commands.entity(entity).remove::<Thing>();
				}
			}
		}
		let right_hand = match right_hand {
			Some(e) => e,
			// keep returning until the model fully loads in and we have found the right hand
			// this is massively inefficient.
			None => return,
		};
		let target = commands
			.spawn((
				PbrBundle {
					transform: Transform::from_xyz(0.3, 0.8, 0.2),
					mesh: meshes.add(Mesh::from(shape::UVSphere {
						radius: 0.05,
						sectors: 7,
						stacks: 7,
					})),
					material: materials.add(StandardMaterial {
						base_color: Color::RED,
						..default()
					}),
					..default()
				},
				ManuallyTarget(Vec4::new(0.0, 0.0, 1.0, 0.3)),
			))
			.id();

		let pole_target = commands
			.spawn(PbrBundle {
				transform: Transform::from_xyz(-1.0, 0.4, -0.2),
				mesh: meshes.add(Mesh::from(shape::UVSphere {
					radius: 0.05,
					sectors: 7,
					stacks: 7,
				})),
				material: materials.add(StandardMaterial {
					base_color: Color::GREEN,
					..default()
				}),
				..default()
			})
			.id();

		// Add an IK constraint to the right hand, using the targets that were created earlier.
		commands.entity(right_hand).insert(IkConstraint {
			chain_length: 3,
			iterations: 20,
			target,
			pole_target: Some(pole_target),
			pole_angle: -std::f32::consts::FRAC_PI_2,
			enabled: true,
		});
	}
}

fn manually_target(
	camera_query: Query<(&Camera, &GlobalTransform)>,
	mut target_query: Query<(&ManuallyTarget, &mut Transform)>,
	mut cursor: EventReader<CursorMoved>,
) {
	let (camera, transform) = camera_query.single();

	if let Some(event) = cursor.iter().last() {
		let view = transform.compute_matrix();
		let viewport_rect = camera.logical_viewport_rect().unwrap();
		let viewport_size = viewport_rect.size();
		let adj_cursor_pos =
			event.position - Vec2::new(viewport_rect.min.x, viewport_rect.min.y);

		let projection = camera.projection_matrix();
		let far_ndc = projection.project_point3(Vec3::NEG_Z).z;
		let near_ndc = projection.project_point3(Vec3::Z).z;
		let cursor_ndc =
			((adj_cursor_pos / viewport_size) * 2.0 - Vec2::ONE) * Vec2::new(1.0, -1.0);
		let ndc_to_world: Mat4 = view * projection.inverse();
		let near = ndc_to_world.project_point3(cursor_ndc.extend(near_ndc));
		let far = ndc_to_world.project_point3(cursor_ndc.extend(far_ndc));
		let ray_direction = far - near;

		for (&ManuallyTarget(plane), mut transform) in target_query.iter_mut() {
			let normal = plane.truncate();
			let d = plane.w;
			let denom = normal.dot(ray_direction);
			if denom.abs() > 0.0001 {
				let t = (normal * d - near).dot(normal) / denom;
				transform.translation = near + ray_direction * t;
			}
		}
	}
}
