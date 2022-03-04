use bevy::{core::FixedTimestep, input::mouse::MouseWheel, prelude::*};
use heron::prelude::*;

#[cfg(feature = "editor")]
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
#[cfg(feature = "editor")]
use bevy_editor_pls::prelude::*;
#[cfg(feature = "dev")]
use bevy_inspector_egui::WorldInspectorPlugin;

mod tile;
use map::Map;
use tile::{ForceParams, Tile, TileKind};
mod map;

const BLOCK_SIZE: f32 = 20.;
const BLOCK_OFFSET: f32 = 35.;
const MAP_ROWS: usize = 24;
const MAP_COLUMNS: usize = 24;

fn main() {
    color_eyre::install().unwrap();

    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugin(PhysicsPlugin::default());
    app.insert_resource(Gravity::from(Vec2::ZERO));

    #[cfg(feature = "dev")]
    app.add_plugin(WorldInspectorPlugin::new());
    #[cfg(feature = "editor")]
    app.add_plugin(EditorPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin);

    app.add_startup_system(setup);
    app.add_startup_system(create_map);
    app.add_system_set(
        SystemSet::new()
            .with_run_criteria(FixedTimestep::step(1. / 60.))
            .with_system(zoom)
            .with_system(set_map)
            .with_system(tile::input)
            .with_system(tile::mouse_input)
            .with_system(tile::go_home),
    );
    app.register_type::<Tile>();
    app.run();
}

#[derive(Debug, Clone, Copy)]
enum Stage {
    NewGame,
    MapSet,
    KillScreen,
}

#[derive(Component)]
struct MainCamera;

fn setup(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands.insert_resource(ClearColor(Color::BLACK));

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 0.0, 900.0),
            perspective_projection: PerspectiveProjection {
                far: 9000.,
                near: 0.0001,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(MainCamera);
    commands.insert_resource(AmbientLight {
        color: Color::ANTIQUE_WHITE,
        brightness: 0.95,
    });
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(5.0, 5.0, 500.0),
        point_light: PointLight {
            intensity: 1.0,
            range: 250.,
            color: Color::WHITE,
            shadow_depth_bias: 0.0,
            shadow_normal_bias: 0.0,
            shadows_enabled: true,
            ..Default::default()
        },
        ..Default::default()
    });
}

fn create_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Cube { size: BLOCK_SIZE }));
    let green_tile = materials.add(StandardMaterial {
        base_color: Color::hsl(125., 0.5, 0.5),
        ..Default::default()
    });
    commands.insert_resource(Stage::NewGame);
    commands.insert_resource(ForceParams::regular());

    let map = Map::new(MAP_ROWS, MAP_COLUMNS);
    let blox = (MAP_ROWS * MAP_COLUMNS) as f32;

    for (x, kind) in map.map.iter().enumerate() {
        let size = Vec3::new(BLOCK_SIZE, BLOCK_SIZE, 0.0);
        let original_position = Vec3::new(
            (x as f32 / blox.sqrt()).floor() * BLOCK_OFFSET - blox.sqrt() * BLOCK_OFFSET / 2.0,
            (x as f32 % blox.sqrt()).floor() * BLOCK_OFFSET - blox.sqrt() * BLOCK_OFFSET / 2.0,
            0.0,
        );

        commands
            .spawn_bundle(PbrBundle {
                mesh: mesh.clone(),
                material: green_tile.clone(),
                transform: Transform::from_translation(original_position),
                ..Default::default()
            })
            .insert(RigidBody::Dynamic)
            .insert(CollisionShape::Cuboid {
                half_extends: size / 2.,
                border_radius: None,
            })
            .insert(Velocity::from(Vec2::ZERO))
            .insert(PhysicMaterial {
                restitution: 0.9,
                friction: 0.2,
                density: 1.,
            })
            .insert(Damping::from_linear(0.5).with_angular(0.2))
            .insert(Tile {
                original_position,
                kind: *kind,
                index_in_map: x,
            })
            .insert(Name::new(format!("My block {x}")));
    }

    commands.insert_resource(map);
}

fn set_map(
    keys: Res<Input<KeyCode>>,
    mut stage: ResMut<Stage>,
    mut map: ResMut<Map>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&mut Tile, &mut Handle<StandardMaterial>)>,
) {
    if !keys.just_pressed(KeyCode::X) {
        return;
    }
    if !matches!(*stage, Stage::NewGame) {
        return;
    }

    map.set_bombs(16);

    let red_tile = materials.add(StandardMaterial {
        base_color: Color::hsl(15., 0.5, 0.5),
        ..Default::default()
    });
    let orange_tile = materials.add(StandardMaterial {
        base_color: Color::hsl(55., 0.5, 0.5),
        ..Default::default()
    });

    for (mut tile, mut material) in query.iter_mut() {
        tile.kind = map.map[tile.index_in_map];
        match tile.kind {
            TileKind::Boom => {
                *material = red_tile.clone();
            }
            TileKind::Danger(_) => {
                *material = orange_tile.clone();
            }
            TileKind::Fine => {
                // same old green
            }
        }
    }

    *stage = Stage::MapSet;
    info!("Game set");
}

fn zoom(
    mut scroll_evr: EventReader<MouseWheel>,
    mut query: Query<(&mut Transform,), With<MainCamera>>,
) {
    use bevy::input::mouse::MouseScrollUnit;

    let mut camera = if let Ok((camera,)) = query.get_single_mut() {
        camera
    } else {
        info!("no camera");
        return;
    };

    for ev in scroll_evr.iter() {
        let y = match ev.unit {
            MouseScrollUnit::Line => ev.y * 40.,
            MouseScrollUnit::Pixel => ev.y,
        };

        camera.translation.z += y;
    }
}
