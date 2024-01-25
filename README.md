# bevy_hsl_multiplier
![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![CI](https://github.com/shanecelis/bevy_hsl_multiplier/actions/workflows/rust.yml/badge.svg)](https://github.com/shanecelis/bevy_hsl_multiplier/actions)
  [![crates-io](https://img.shields.io/crates/v/bevy_hsl_multiplier.svg)](https://crates.io/crates/bevy_hsl_multiplier)
  [![api-docs](https://docs.rs/bevy_hsl_multiplier/badge.svg)](https://docs.rs/bevy_hsl_multiplier)

This crate provides a shader that multiplies a texture's color in HSL color space; it can be applied to 2D
and 3D objects on the [bevy game engine](https://bevyengine.org).

![hsl multiplier example](https://github.com/shanecelis/bevy_terminal_shader/assets/54390/05308e0a-439f-4ae8-9aa2-07144222aa3e)

# Install

Not actually available on crates.io yet.

``` sh
# cargo add bevy_hsl_multiplier
```

But you can get it from the repo directly.

``` sh
cargo add --git https://github.com/shanecelis/bevy_hsl_multiplier.git
```

# Usage

## Add plugin to app
```compile
use bevy::prelude::*;
fn main() {
    App::new()
        .add_plugins(bevy_hsl_multiplier::HslMultiplierPlugin)
        .run()
}
```

## Add settings to camera

```compile
use bevy::prelude::*;

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
```

# Example

Run the "quad" example like so:

``` sh
cargo run --example quad
```

This will show a large quad like the one shown at the beginning of this README.

``` sh
cargo run --example cube
```

This will show a rotating cube with the shader as its surfaces.

# License

This crate is licensed under the MIT License or the Apache License 2.0 or CC0 License.

# Acknowlegments

* [Example rust crab image](https://blog.devgenius.io/creating-an-api-with-rust-clean-architecture-axum-and-surrealdb-2a95b1b72e0f) from [Vitor Lacerda](https://medium.com/@vitorlacerdafaria7).

* Prompted by [PrinceOfBorgo](https://www.reddit.com/user/PrinceOfBorgo/)'s [question](https://www.reddit.com/r/bevy/comments/19dn10x/how_to_edit_colors_of_a_texture_in_a_spritebundle/) on reddit.

* Most code copied wholesale from my other crate the [bevy_terminal_shader](https://github.com/shanecelis/bevy_terminal_shader).
