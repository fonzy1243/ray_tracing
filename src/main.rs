use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use std::ops::Range;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, geostar_movement)
        .run();
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("geostar.png"),
            transform: Transform::from_xyz(100., 0., 50.).with_scale(Vec3::new(0.5, 0.5, 1.)),
            ..default()
        },
        Direction::Up,
    ));
}

fn geostar_movement(time: Res<Time>, mut spire_position: Query<(&mut Direction, &mut Transform)>) {
    let mut angle;

    for (mut geostar, mut transform) in &mut spire_position {
        match *geostar {
            Direction::Up => {
                transform.translation.x += 350. * time.delta_seconds();
                transform.translation.y += 150. * time.delta_seconds();
                angle = 7f32;
            }
            Direction::Down => {
                transform.translation.x -= 350. * time.delta_seconds();
                transform.translation.y -= 150. * time.delta_seconds();
                angle = -7f32;
            }
        }

        if transform.translation.y > 200. {
            *geostar = Direction::Down;
        } else if transform.translation.y < -200. {
            *geostar = Direction::Up;
        }

        transform.rotate(Quat::from_rotation_z(angle * time.delta_seconds()));
    }
}
