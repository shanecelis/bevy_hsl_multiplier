//! Shows the terminal material rendered on a quad.

use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::{PresentMode, WindowTheme},
};
use bevy_hsl_multiplier::{HslMultiplierMaterial, HslMultiplierPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "I am a window!".into(),
                    resolution: (1024., 1024.).into(),
                    present_mode: PresentMode::AutoVsync,
                    // Tells wasm to resize the window according to the available canvas
                    fit_canvas_to_parent: true,
                    // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    // This will spawn an invisible window
                    // The window will be made visible in the make_visible() system after 3 frames.
                    // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                    ..default()
                }),
                ..default()
            }),
            HslMultiplierPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, update_multiplier)
        .run();
}

/// Setup a quad and camera.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<HslMultiplierMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(1024., 1024.)).into())
            .into(),
        material: materials.add(HslMultiplierMaterial {
            hsla_multiplier: Vec4::new(1.0, 1.0, 1.0, 1.0),
            color_texture: Some(asset_server.load("rust_crab.png")),
            alpha_mode: AlphaMode::Opaque,
        }),
        ..default()
    });
}

fn update_multiplier(
    q: Query<&mut Handle<HslMultiplierMaterial>>,
    mut materials: ResMut<Assets<HslMultiplierMaterial>>,
    time: Res<Time>,
) {
    let h = q.single();
    let m = materials.get_mut(h).unwrap();
    let t = time.elapsed_seconds() / 2.0;
    m.hsla_multiplier.y = t.sin().abs();
}
