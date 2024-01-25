//! Shows the terminal material rendered on a quad.

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_terminal_shader::{TerminalMaterial, TerminalShaderPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalShaderPlugin))
        .add_systems(Startup, setup)
        .run();
}

/// Setup a quad and camera.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<TerminalMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(1300., 800.)).into())
            .into(),
        material: materials.add(TerminalMaterial::green()),
        ..default()
    });
}
