#import bevy_pbr::forward_io::VertexOutput

@group(1) @binding(0) var<uniform> hsla_multiplier: vec4<f32>;
@group(1) @binding(1) var material_color_texture: texture_2d<f32>;
@group(1) @binding(2) var material_color_sampler: sampler;
// Function to convert RGB to HSL
fn rgb2hsl(c: vec3<f32>) -> vec3<f32> {
    let cMin = min(min(c.r, c.g), c.b);
    let cMax = max(max(c.r, c.g), c.b);
    let delta = cMax - cMin;
    var hsl: vec3<f32> = vec3<f32>(0.0, 0.0, (cMax + cMin) / 2.0);

    if (delta != 0.0) {
        if (hsl.z < 0.5) {
            hsl.y = delta / (cMax + cMin); // Saturation.
        } else {
            hsl.y = delta / (2.0 - cMax - cMin); // Saturation.
        }

        let deltaR = (((cMax - c.r) / 6.0) + (delta / 2.0)) / delta;
        let deltaG = (((cMax - c.g) / 6.0) + (delta / 2.0)) / delta;
        let deltaB = (((cMax - c.b) / 6.0) + (delta / 2.0)) / delta;

        // Hue.
        if (c.r == cMax) {
            hsl.x = deltaB - deltaG;
        } else if (c.g == cMax) {
            hsl.x = (1.0 / 3.0) + deltaR - deltaB;
        } else { // if (c.b == cMax)
            hsl.x = (2.0 / 3.0) + deltaG - deltaR;
        }

        hsl.x = fract(hsl.x);
    }

    return hsl;
}

// Helper function for hue to RGB
fn hue2rgb(hue_: f32) -> vec3<f32> {
    let hue = fract(hue_);
    return saturate(vec3<f32>(
        abs(hue * 6.0 - 3.0) - 1.0,
        2.0 - abs(hue * 6.0 - 2.0),
        2.0 - abs(hue * 6.0 - 4.0)
    ));
}

// Function to convert HSL to RGB
fn hsl2rgb(hsl: vec3<f32>) -> vec3<f32> {
    if (hsl.y == 0.0) {
        return vec3<f32>(hsl.z); // Luminance.
    } else {
        var b: f32 = 0.0;
        if (hsl.z < 0.5) {
            b = hsl.z * (1.0 + hsl.y);
        } else {
            b = hsl.z + hsl.y - hsl.y * hsl.z;
        }

        let a = 2.0 * hsl.z - b;


        return a + hue2rgb(hsl.x) * (b - a);
    }
}

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let color_rgb = textureSample(material_color_texture, material_color_sampler, mesh.uv);
    var color_hsl = rgb2hsl(color_rgb.xyz);
    color_hsl *= hsla_multiplier.xyz;
    return vec4<f32>(hsl2rgb(color_hsl), hsla_multiplier.w * color_rgb.w);
}
