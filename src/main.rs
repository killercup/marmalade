use bevy::{input::mouse::MouseWheel, prelude::*};
use bevy_mod_picking::*;
use heron::prelude::*;

#[cfg(feature = "editor")]
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
#[cfg(feature = "editor")]
use bevy_editor_pls::prelude::*;
#[cfg(feature = "dev")]
use bevy_inspector_egui::WorldInspectorPlugin;

mod tile;
use tile::Tile;
mod killscreen;
mod map_actions;
mod map_generator;
mod minesweeper;
mod params;
mod stages;

const BLOCK_SIZE: f32 = 20.;
const BLOCK_OFFSET: f32 = 35.;
const MAP_ROWS: usize = 24;
const MAP_COLUMNS: usize = 24;
const BOMB_COUNT: usize = 64;

fn main() {
    color_eyre::install().unwrap();

    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    app.add_plugin(PhysicsPlugin::default());
    app.insert_resource(Gravity::from(Vec2::ZERO));

    app.add_plugins(DefaultPickingPlugins);
    app.insert_resource(PickingPluginsState {
        enable_picking: true,
        enable_highlighting: true,
        enable_interacting: true,
        update_debug_cursor: false,
        print_debug_events: false,
    });

    #[cfg(feature = "dev")]
    app.add_plugin(WorldInspectorPlugin::new());
    #[cfg(feature = "editor")]
    app.add_plugin(EditorPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin);

    app.add_startup_system(setup);
    app.add_startup_system(map_actions::create_map);
    app.add_system_set(
        SystemSet::new()
            .label(SystemSets::GameplayControls)
            .with_system(zoom)
            .with_system(minesweeper::click_on_tile)
            .with_system(map_actions::trigger_set_map)
            .with_system(stages::trigger_endgame)
            .with_system(stages::trigger_reset)
            .with_system(map_actions::toggle_hint),
    );
    app.add_system_set(
        SystemSet::new()
            .label(SystemSets::Map)
            .with_system(map_actions::set_map),
    );
    app.add_system_set(
        SystemSet::new()
            .label(SystemSets::Movements)
            .with_system(tile::mouse_input)
            .with_system(tile::go_home),
    );
    app.add_system_set(
        SystemSet::new()
            .label(SystemSets::Reactions)
            .after(SystemSets::Movements)
            .after(SystemSets::Map)
            .with_system(minesweeper::clear)
            .with_system(killscreen::draw)
            .with_system(minesweeper::go_nuclear_if_fast)
            .with_system(minesweeper::go_nuclear),
    );

    app.add_event::<map_actions::SetMapEvent>();
    app.add_event::<minesweeper::BoomEvent>();
    app.add_event::<minesweeper::ClearTileEvent>();
    app.add_event::<killscreen::GameOverEvent>();

    app.register_type::<Tile>();
    app.run();
}

#[derive(SystemLabel, Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum SystemSets {
    GameplayControls,
    Map,
    Movements,
    Reactions,
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
        .insert_bundle(PickingCameraBundle::default())
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
