use bevy::ecs::system::RunSystemOnce;
use bevy::pbr::DirectionalLightShadowMap;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{EguiPlugin, EguiContext};
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use bevy_inspector_egui::bevy_inspector::hierarchy::SelectedEntities;
use color_eyre::eyre::bail;
use std::f32::consts::*;
use color_eyre::Result;

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
			..Default::default()
		}))
		.add_plugins(EguiPlugin)
		.add_plugins(DefaultInspectorConfigPlugin)
		.add_systems(Startup, setup)
		.add_systems(Update, animate_light_direction)
		.add_systems(Update, animate_morphable)
		.add_systems(PostUpdate, inspector_ui)


		.run();

	Ok(())
}

#[derive(Component)]
struct MorphableTarget {
	
}
impl MorphableTarget {
	fn findTarget(target_name: &str, entity: Entity, world: &mut World) -> Result<Self> {
		let tg_name_cp = target_name.to_owned();
	
		
		world.run_system_once(move | children: Query<&Children>, names: Query<&Name> | -> Result<()> {
			let mut target_entity = None;
			for child in children.iter_descendants(entity) {
				if let Ok(name) = names.get(child) {
					if tg_name_cp == name.as_str() {
						target_entity.replace(child);
					}
				}
			}
			let found_entity = match target_entity {
				Some(e) => e,
				None => bail!("Could not find the target entity {tg_name_cp}"),
			};

			// found_entity.

			Ok(())
		})?;

	

		
		todo!() 
	}
}


fn setup(
	mut commands: Commands,
	assets: Res<AssetServer>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	world: &mut World,
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

	let avatar_entity = commands.spawn((
		SceneBundle {
			scene:  assets.load("malek.gltf#Scene0"),
			transform: Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(
				Quat::from_euler(EulerRot::XYZ, 0.0, 180.0_f32.to_radians(), 0.0),
			),
			..default()
		},
	)).id();


	MorphableTarget::findTarget("Face", avatar_entity, world);

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

fn animate_morphable(
    morphable_entities_q: Query<(Entity, &MorphableTarget)>,
    mut morph_weights_q: Query<&mut MorphWeights>,
    children: Query<&Children>,
    names: Query<&Name>,
	time: Res<Time>,
) {
    for (entity, _thing) in morphable_entities_q.iter() {
        let mut face = None;

        for child in children.iter_descendants(entity) {
            if let Ok(name) = names.get(child) {
                if name.as_str() == "Face" {
                    face.replace(child);
                }
            }
        }

        let face = match face {
            Some(e) => e,
            // keep returning until the model fully loads in and we have found the face
            // this is massively inefficient.
            None => continue,
        };

		let mut morph_wheight = morph_weights_q.get_mut(face).unwrap();

		// Accessing morph 13, that is the blink morph target
		morph_wheight.weights_mut()[13] = (f32::sin(time.elapsed_seconds() * 10.0) + 1.) / 2.;
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
