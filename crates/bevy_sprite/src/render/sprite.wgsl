#ifdef TONEMAP_IN_SHADER
#import bevy_core_pipeline::tonemapping
#endif

#import bevy_render::view  View

@group(0) @binding(0)
var<uniform> view: View;

struct VertexOutput {
    @location(0) uv: vec2<f32>,
#ifdef COLORED
    @location(1) color: vec4<f32>,
#endif
    @builtin(position) position: vec4<f32>,
};

fn saturation_mat(saturation: f32) -> mat4x4<f32> {
    let rwgt = 0.3086;
    let gwgt = 0.6094;
    let bwgt = 0.082;

    let r_sat = (1. - saturation) * rwgt;
    let g_sat = (1. - saturation) * gwgt;
    let b_sat = (1. - saturation) * bwgt;

    let s_mat = mat4x4<f32>(
        r_sat + saturation, r_sat, r_sat, 0.,
        g_sat, g_sat + saturation, g_sat, 0.,
        b_sat, b_sat, b_sat + saturation, 0.,
        0., 0., 0., 1.
    );

    return s_mat;
}

fn value_mat(value: f32) -> mat4x4<f32> {
    let v_mat = mat4x4<f32>(
        value, 0., 0., 0.,
        0., value, 0., 0.,
        0., 0., value, 0.,
        0., 0., 0., 1.
    );

    return v_mat;
}

fn srgb_to_linear(srgb: vec4<f32>) -> vec4<f32> {
    let color_srgb: vec3<f32> = srgb.rgb;
    let selector: vec3<f32> = ceil(color_srgb - 0.04045);
    let under: vec3<f32> = color_srgb / 12.92;
    let over: vec3<f32> = pow((color_srgb + 0.055) / 1.055, vec3<f32>(2.4));
    let result: vec3<f32> = mix(under, over, selector);

    return vec4<f32>(result, srgb.a);
}

fn color_luma(color: vec3<f32>) -> f32 {
    return 0.2126 * color.r + 0.7162 * color.g + 0.0722 * color.b;
}

@vertex
fn vertex(
    @location(0) vertex_position: vec3<f32>,
    @location(1) vertex_uv: vec2<f32>,
#ifdef COLORED
    @location(2) vertex_color: vec4<f32>,
#endif
) -> VertexOutput {
    var out: VertexOutput;
    out.uv = vertex_uv;
    out.position = view.view_proj * vec4<f32>(vertex_position, 1.0);
#ifdef COLORED
    out.color = vertex_color;
#endif
    return out;
}

@group(1) @binding(0)
var sprite_texture: texture_2d<f32>;
@group(1) @binding(1)
var sprite_sampler: sampler;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var color = textureSample(sprite_texture, sprite_sampler, in.uv);

    ///////////
    color = vec4<f32>(color.rgb * color.a, color.a);
    // arbitrary sat + value to simulate color_dodge / overlay -sh photoshop's blending modes
    color = saturation_mat(1.5) * value_mat(3.5) * color;
    // green tint
    let green_color_cast = vec4<f32>(0.25, 1.0, 0.25, 0.75);
    let pre_cast = srgb_to_linear(color);
    let luma = color_luma(pre_cast.rgb);
    color = vec4<f32>(green_color_cast.rgb * luma, pre_cast.a * green_color_cast.a);
    //////////

#ifdef COLORED
    color = in.color * color;
#endif

#ifdef TONEMAP_IN_SHADER
    color = bevy_core_pipeline::tonemapping::tone_mapping(color, view.color_grading);
#endif

    return color;
}
