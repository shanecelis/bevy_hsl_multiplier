//! Shows the terminal material rendered on a cube.

use bevy::prelude::*;

use bevy_hsl_multiplier::{HslMultiplierMaterial, HslMultiplierPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HslMultiplierPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, update_multiplier)
        .run();
}

/// Set up a simple 3D scene.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<HslMultiplierMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // cube
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: materials.add(HslMultiplierMaterial {
            hsla_multiplier: Vec4::new(1.0, 1.0, 1.0, 1.0),
            color_texture: Some(asset_server.load("rust_crab.png")),
            alpha_mode: AlphaMode::Opaque,
        }),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn update_multiplier(q: Query<&mut Handle<HslMultiplierMaterial>>,
                     mut materials: ResMut<Assets<HslMultiplierMaterial>>,
                     time: Res<Time>) {
    let h = q.single();
    let m = materials.get_mut(h).unwrap();
    let t = time.elapsed_seconds() / 2.0;
    m.hsla_multiplier.y = t.sin().abs();
}
