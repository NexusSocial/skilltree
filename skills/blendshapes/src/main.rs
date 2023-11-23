use bevy::pbr::DirectionalLightShadowMap;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{EguiContext, EguiPlugin};
use bevy_inspector_egui::bevy_inspector::hierarchy::SelectedEntities;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use color_eyre::eyre::{bail, ensure, eyre, WrapErr};
use color_eyre::Result;
use std::f32::consts::*;

const ASSET_FOLDER: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/");
fn main() -> Result<()> {
	// Set up nice error messages
	color_eyre::install()?;

	// This will take like 5 seconds to load the gltf/vrm model, don't worry, just wait.
	App::new()
		.insert_resource(AmbientLight {
			color: Color::WHITE,
			brightness: 1.0 / 5.0f32,
		})
		.insert_resource(DirectionalLightShadowMap { size: 4096 })
		.add_plugins(DefaultPlugins.set(AssetPlugin {
			file_path: ASSET_FOLDER.to_string(),
			..default()
		}))
		.add_plugins(EguiPlugin)
		.add_plugins(DefaultInspectorConfigPlugin)
		.add_systems(Startup, setup)
		.add_systems(Update, on_avatar_load)
		.insert_resource(PendingAvatars::default())
		.add_systems(Update, mark_morph_targets::<Blink>.map(log_on_err))
		.add_event::<MarkMorphTargetEvent>()
		.add_systems(Update, animate_light_direction)
		.add_systems(Update, animate_blink)
		.add_systems(PostUpdate, inspector_ui)
		.run();

	Ok(())
}

fn panic_on_err(result: Result<()>) {
	result.unwrap()
}

fn log_on_err(result: Result<()>) {
	if let Err(err) = result {
		error!("{:?}", err);
	}
}

fn setup(
	mut commands: Commands,
	assets: Res<AssetServer>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	mut pending_avatars: ResMut<PendingAvatars>,
) {
	commands
		.spawn(SpatialBundle::default())
		.with_children(|parent| {
			parent.spawn(Camera3dBundle {
				transform: Transform::from_xyz(0.0, 1.45, 0.5)
					.looking_at(Vec3::new(0.0, 1.45, 0.0), Vec3::Y),
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

	let scene = assets.load("malek.gltf#Scene0");
	let avatar_entity = commands
		.spawn(SceneBundle {
			scene: scene.clone(),
			transform: Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(
				Quat::from_euler(EulerRot::XYZ, 0.0, 180.0_f32.to_radians(), 0.0),
			),
			..default()
		})
		.id();
	pending_avatars.0.insert(
		scene.id(),
		MarkMorphTargetEvent {
			avatar: avatar_entity,
			child_name: "Face".into(),
			weight_name: "Blink",
		},
	);
}

/// Because asset loading happens asynchronously over multiple update frames, we can't
/// immediately send events. Therefore, we store these events in this resource
/// and defer their sending until we detect that the avatar is loaded.
#[derive(Resource, Debug, Default)]
struct PendingAvatars(bevy::utils::HashMap<AssetId<Scene>, MarkMorphTargetEvent>);

/// Sends a [`MarkMorphTargetEvent`] when the avatar's `Scene` has loaded.
fn on_avatar_load(
	mut asset_event: EventReader<AssetEvent<Scene>>,
	mut mark_morph_targets_writer: EventWriter<MarkMorphTargetEvent>,
	mut pending_avatars: ResMut<PendingAvatars>,
) {
	for evt in asset_event.read() {
		match evt {
			AssetEvent::Removed { id } => {
				pending_avatars.0.remove(&*id);
			}
			AssetEvent::LoadedWithDependencies { id } => {
				let Some(pending) = pending_avatars.0.remove(id) else {
					continue;
				};
				info!("Sending morph mark event");
				mark_morph_targets_writer.send(pending);
			}
			_ => continue,
		}
	}
}

/// Events for the [`mark_morph_targets`] system.
#[derive(Debug, Event)]
struct MarkMorphTargetEvent {
	pub avatar: Entity,
	pub child_name: Name,
	pub weight_name: &'static str,
}

/// Marks information about a morph target.
/// `C` allows giving the target a particular type, to make queries easy, as well
/// as adding any additional desired metadata.
#[derive(Debug, Component)]
struct MorphTarget<C> {
	pub weight_idx: usize,
	#[allow(unused)]
	pub target_info: C,
}

/// A blink morph target. Used inside [`MorphTarget`].
struct Blink;

/// The particular entity to mark will be filtered for based on `input`.
/// Attaches component [`MorphTarget<C>`] to this entity.
fn mark_morph_targets<C>(
	mut inputs: EventReader<MarkMorphTargetEvent>,
	children: Query<&Children>,
	morph_weights: Query<&MorphWeights>,
	names: Query<&Name>,
	meshes: Res<Assets<Mesh>>,
	mut cmds: Commands,
) -> Result<()> {
	for input in inputs.read() {
		info!("Marking morphs");
		mark_morph_target::<C>(
			input,
			&children,
			&morph_weights,
			&names,
			&meshes,
			&mut cmds,
		)
		.wrap_err_with(|| format!("Failed to mark morph target for {input:?}"))?;
	}
	Ok(())
}

/// The single item version of [`mark_morph_targets`], except we also return the
/// Entity that was marked.
fn mark_morph_target<C>(
	input: &MarkMorphTargetEvent,
	children: &Query<&Children>,
	morph_weights: &Query<&MorphWeights>,
	names: &Query<&Name>,
	meshes: &Res<Assets<Mesh>>,
	cmds: &mut Commands,
) -> Result<Entity> {
	let MarkMorphTargetEvent {
		avatar,
		child_name,
		weight_name,
	} = input;
	let mut num_children = 0;
	let matching_child = children.iter_descendants(*avatar).find(|child| {
		num_children += 1;
		names.get(*child) == Ok(child_name)
	});
	ensure!(
		num_children > 0,
		"No children of avatar entity {avatar:?} could be found"
	);
	let Some(matching_child) = matching_child else {
		bail!("No child named `{child_name}` found");
	};
	// TODO: Can we combine the morph weights query with the children query?
	let morph_weights = morph_weights.get(matching_child).unwrap();
	let first_mesh_handle = morph_weights.first_mesh().expect("expected a mesh");
	let first_mesh = meshes
		.get(first_mesh_handle)
		.expect("expected asset referenced by morph weights to exist");
	let (idx, _) = first_mesh
		.morph_target_names()
		.ok_or_else(|| {
			eyre!("failed to get morph target names for mesh {first_mesh_handle:?}")
		})?
		.iter()
		.enumerate()
		.find(|(_idx, name)| *name == weight_name)
		.expect("expected morph target with name {weight_name}");

	cmds.entity(matching_child).insert(MorphTarget {
		weight_idx: idx,
		target_info: Blink,
	});
	Ok(matching_child)
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

fn animate_blink(
	mut morphs: Query<(&mut MorphWeights, &MorphTarget<Blink>)>,
	time: Res<Time>,
) {
	for (mut weights, target) in morphs.iter_mut() {
		// Accessing morph 13, that is the blink morph target
		weights.weights_mut()[target.weight_idx] =
			(f32::sin(time.elapsed_seconds() * 10.0) + 1.) / 2.;
	}
}

fn inspector_ui(world: &mut World, mut selected_entities: Local<SelectedEntities>) {
	let mut egui_context = world
		.query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
		.single(world)
		.clone();

	egui::SidePanel::left("hierarchy")
		.default_width(200.0)
		.show(egui_context.get_mut(), |ui| {
			egui::ScrollArea::vertical().show(ui, |ui| {
				ui.heading("Hierarchy");

				bevy_inspector_egui::bevy_inspector::hierarchy::hierarchy_ui(
					world,
					ui,
					&mut selected_entities,
				);

				ui.label("Press escape to toggle UI");
				ui.allocate_space(ui.available_size());
			});
		});

	egui::SidePanel::right("inspector")
		.default_width(250.0)
		.show(egui_context.get_mut(), |ui| {
			egui::ScrollArea::vertical().show(ui, |ui| {
				ui.heading("Inspector");

				match selected_entities.as_slice() {
					&[entity] => {
						bevy_inspector_egui::bevy_inspector::ui_for_entity(
							world, entity, ui,
						);
					}
					entities => {
						bevy_inspector_egui::bevy_inspector::ui_for_entities_shared_components(
							world, entities, ui,
						);
					}
				}

				ui.allocate_space(ui.available_size());
			});
		});
}
