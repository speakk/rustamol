#import bevy_sprite::mesh2d_struct
#import bevy_sprite::mesh2d_view_bind_group

struct ShaderMaterial {
    color: vec4<f32>;
    // 'flags' is a bit field indicating various options. u32 is 32 bits so we have up to 32 options.
    flags: u32;
    outline: u32;
};
let COLOR_MATERIAL_FLAGS_TEXTURE_BIT: u32 = 1u;

[[group(0), binding(0)]]
var<uniform> view: View;

[[group(1), binding(0)]]
var<uniform> material: ShaderMaterial;
[[group(1), binding(1)]]
var texture: texture_2d<f32>;
[[group(1), binding(2)]]
var texture_sampler: sampler;

[[group(2), binding(0)]]
var<uniform> mesh: Mesh2d;

struct FragmentInput {
    [[builtin(front_facing)]] is_front: bool;
    [[location(0)]] world_position: vec4<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
#ifdef VERTEX_TANGENTS
    [[location(3)]] world_tangent: vec4<f32>;
#endif
};


// TODO: Have this come externally, 1/spriteW, 1/spriteH
var<private> stepSize: vec2<f32> = vec2<f32>(.01, .01);

[[stage(fragment)]]
fn fragment(in: FragmentInput) -> [[location(0)]] vec4<f32> {
    var output_color: vec4<f32> = material.color;
    if ((material.flags & COLOR_MATERIAL_FLAGS_TEXTURE_BIT) != 0u) {
      output_color = output_color * textureSample(texture, texture_sampler, in.uv);
      if (material.outline == 0u) {
        return output_color;
      }

      var alpha: f32 = 4.0 * textureSample(texture, texture_sampler, in.uv).a;
      alpha = alpha - textureSample(texture, texture_sampler, in.uv + vec2<f32>(stepSize.x, 0.)).a;
      alpha = alpha - textureSample(texture, texture_sampler, in.uv + vec2<f32>(-stepSize.x, 0.)).a;
      alpha = alpha - textureSample(texture, texture_sampler, in.uv + vec2<f32>(0., stepSize.y)).a;
      alpha = alpha - textureSample(texture, texture_sampler, in.uv + vec2<f32>(0., -stepSize.y)).a;

      var result_color: vec4<f32> = vec4<f32>(0., 1., 1., alpha);
      if (result_color.a == 0.) {
        return output_color;
      } else {
        return result_color;
      }
    }

    return output_color;
}
