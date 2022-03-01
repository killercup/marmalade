use std::f32::consts::PI;

use bevy::{core::FixedTimestep, prelude::*};
use heron::prelude::*;

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
    app.add_plugin(PhysicsPlugin::default());
    app.insert_resource(Gravity::from(Vec2::ZERO));

    #[cfg(feature = "dev")]
    app.add_plugin(WorldInspectorPlugin::new());
    #[cfg(feature = "editor")]
    app.add_plugin(EditorPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin);

    app.add_startup_system(setup);
    app.add_system_set(
        SystemSet::new()
            .with_run_criteria(FixedTimestep::step(1. / 60.))
            .with_system(thingy::input)
            .with_system(thingy::mouse_input)
            .label("movin"),
    );
    app.register_type::<Thingy>();
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    commands.insert_resource(ClearColor(Color::BLACK));

    let scale = 10.;
    let offset = 15.;
    let blox = 10_000_f32;

    for x in 1..(blox as usize) {
        let size = Vec3::new(scale, scale, 0.0);
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(
                        (x as f32 / blox.sqrt()).floor() * offset - blox.sqrt() * offset / 2.0,
                        (x as f32 % blox.sqrt()).floor() * offset - blox.sqrt() * offset / 2.0,
                        0.0,
                    ),
                    scale: size,
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::hsl(360.0 * x as f32 / blox, 0.5, 0.5),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(RigidBody::Dynamic)
            .insert(CollisionShape::Cuboid {
                half_extends: size / 2.,
                border_radius: None,
            })
            .insert(Velocity::from(Vec2::ZERO).with_angular(AxisAngle::new(Vec3::Z, -PI)))
            .insert(PhysicMaterial {
                restitution: 1.,
                density: 5.,
                ..Default::default()
            })
            .insert(Thingy {
                rotation: 0.0,
                position: Vec3::ZERO,
            })
            .insert(Name::new(format!("My block {x}")));
    }
}
