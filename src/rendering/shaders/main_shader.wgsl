struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) v_tex_translation: vec2<f32>
 };

struct Uniforms {
    model_trans: mat4x4<f32>,
    camera_view: mat4x4<f32>,
}

@group(0)
@binding(0)
var<uniform> r_data: Uniforms;

@vertex
fn vs_main(
    @location(0) a_position : vec3<f32>,
    @location(1) a_tex_translation : vec2<f32>,
) ->  VertexOutput{
    var result: VertexOutput;
    result.v_tex_translation = a_tex_translation;
    result.position = r_data.camera_view * (r_data.model_trans * vec4<f32>(a_position, 1.));
    return result;
}

@group(1)
@binding(0)
var t_diffuse: texture_2d<f32>;

@group(1)
@binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, vertex.v_tex_translation);
}