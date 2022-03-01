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
            .with_system(thingy::go_home),
    );
    app.register_type::<Thingy>();
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands.insert_resource(ClearColor(Color::BLACK));

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 0.0, 700.0),
        ..Default::default()
    });
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

    let scale = 10.;
    let offset = 15.;
    let blox = 10_000_f32;

    let mesh = meshes.add(Mesh::from(shape::Cube { size: scale }));

    for x in 1..(blox as usize) {
        let size = Vec3::new(scale, scale, 0.0);
        let original_position = Vec3::new(
            (x as f32 / blox.sqrt()).floor() * offset - blox.sqrt() * offset / 2.0,
            (x as f32 % blox.sqrt()).floor() * offset - blox.sqrt() * offset / 2.0,
            0.0,
        );
        let material = materials.add(StandardMaterial {
            base_color: Color::hsl(360.0 * x as f32 / blox, 0.5, 0.5),
            ..Default::default()
        });

        commands
            .spawn_bundle(PbrBundle {
                mesh: mesh.clone(),
                material: material.clone(),
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
                density: 5.,
            })
            .insert(Thingy { original_position })
            .insert(Name::new(format!("My block {x}")));
    }
}
