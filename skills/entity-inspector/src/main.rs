use bevy::{pbr::DirectionalLightShadowMap, prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::{DefaultInspectorConfigPlugin, bevy_inspector::{hierarchy::SelectedEntities}};
use color_eyre::eyre::Result;
use tracing::info;
use bevy_egui::{EguiPlugin, EguiContext};

const ASSET_FOLDER: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/");

fn main() -> Result<()> {
	// Set up nice error messages
	color_eyre::install()?;

	info!("Running `entity-inspector` skill");

	App::new()
		.add_plugins(DefaultPlugins.set(AssetPlugin {
			file_path: ASSET_FOLDER.to_string(),
			..Default::default()
		}))
		.add_plugins(EguiPlugin)
		.add_plugins(DefaultInspectorConfigPlugin)
		.add_systems(Startup, setup)
		.add_systems(Update, animate_light)
		.add_systems(PostUpdate, inspector_ui)
		.run();

	Ok(())
}

fn setup(
	assets: Res<AssetServer>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	mut commands: Commands,
) {
	info!("Running setup system");

	// Load assets
	let tree_img: Handle<Image> = assets.load("tree.png");

	// Build cube
	commands.spawn(PbrBundle {
		mesh: meshes.add(shape::Cube::default().into()),
		material: materials.add(StandardMaterial {
			base_color_texture: Some(tree_img),
			..default()
		}),
		..default()
	});

	// Build the rest of the scene
	commands.spawn(DirectionalLightBundle {
		directional_light: DirectionalLight {
			shadows_enabled: true,
			illuminance: 10000.,
			..default()
		},
		transform: Transform::from_xyz(8.0, 16.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
		..default()
	});
	commands.insert_resource(DirectionalLightShadowMap { size: 4096 });
	commands.spawn(Camera3dBundle {
		transform: Transform::from_xyz(0.0, 6., 12.0)
			.looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
		..default()
	});
	commands.spawn(PbrBundle {
		mesh: meshes.add(
			shape::Plane {
				size: 10.,
				subdivisions: 4,
			}
			.into(),
		),
		material: materials.add(Color::MIDNIGHT_BLUE.into()),
		transform: Transform::from_xyz(0., 0., 0.),
		..default()
	});
}

fn animate_light(mut query: Query<&mut Transform, With<DirectionalLight>>) {
	for mut t in query.iter_mut() {
		let t: &mut Transform = &mut t;
		t.rotate_y(0.01);
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
						bevy_inspector_egui::bevy_inspector::ui_for_entity(world, entity, ui);
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