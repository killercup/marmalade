use bevy::{core::FixedTimestep, prelude::*};

#[cfg(feature = "editor")]
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
#[cfg(feature = "editor")]
use bevy_editor_pls::prelude::*;
#[cfg(feature = "dev")]
use bevy_inspector_egui::WorldInspectorPlugin;
mod thingy;
use thingy::Thingy;

fn main() {
    color_eyre::install().unwrap();

    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    #[cfg(feature = "dev")]
    app.add_plugin(WorldInspectorPlugin::new());
    #[cfg(feature = "editor")]
    app.add_plugin(EditorPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin);

    app.add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1. / 60.))
                .with_system(thingy::input)
                .with_system(thingy::u_spin_me),
        )
        .register_type::<Thingy>()
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    commands.insert_resource(ClearColor(Color::DARK_GRAY));
    let offset = 60.;
    for x in 1..500 {
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(
                        (x as f32 / 500f32.sqrt()).floor() * offset - 500f32.sqrt() * offset / 2.0,
                        (x as f32 % 500f32.sqrt()).floor() * offset - 500f32.sqrt() * offset / 2.0,
                        0.0,
                    ),
                    scale: Vec3::new(50.0, 50.0, 0.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::rgb(0.5, 0.5, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Thingy {
                rotation: 0.0,
                position: Vec3::ZERO,
            })
            .insert(Name::new("My block"));
    }
}
