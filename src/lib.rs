#![doc(html_root_url = "https://docs.rs/bevy_hsl_multiplier/0.1.0")]
#![doc = include_str!("../README.md")]
use bevy::{
    asset::load_internal_asset,
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef, SpecializedMeshPipelineError},
    render::{
        mesh::InnerMeshVertexBufferLayout,
        render_resource::{RenderPipelineDescriptor, Shader},
    },
    sprite::{Material2d, Material2dKey, Material2dPlugin},
    utils::Hashed,
};

// $ cargo install uuid-tools && uuid -o simple
pub const HSL_MULTIPLIER_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(0x681280e787334019a3a33a59e7629cea);

pub struct HslMultiplierPlugin;

impl Plugin for HslMultiplierPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            HSL_MULTIPLIER_SHADER_HANDLE,
            "../assets/shaders/hsl_multiplier.wgsl",
            Shader::from_wgsl
        );
        app.add_plugins((
            MaterialPlugin::<HslMultiplierMaterial>::default(),
            Material2dPlugin::<HslMultiplierMaterial>::default(),
        ));
    }
}

impl Material for HslMultiplierMaterial {
    fn fragment_shader() -> ShaderRef {
        HSL_MULTIPLIER_SHADER_HANDLE.into()
    }
}

impl Material2d for HslMultiplierMaterial {
    fn fragment_shader() -> ShaderRef {
        HSL_MULTIPLIER_SHADER_HANDLE.into()
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct HslMultiplierMaterial {
    #[uniform(0)]
    pub hsla_multiplier: Vec4,
    #[texture(1)]
    #[sampler(2)]
    pub color_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,
}

impl Default for HslMultiplierMaterial {
    fn default() -> Self {
        Self {
            hsla_multiplier: Vec4::ONE,
            color_texture: None,
            alpha_mode: AlphaMode::default(),
        }
    }
}
