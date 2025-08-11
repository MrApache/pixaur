@group(0) @binding(0) var texture: texture_2d<f32>;
@group(0) @binding(1) var t_sampler: sampler;

struct InstanceInput {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
    @location(9) color_start: vec4<f32>,
    @location(10) color_end: vec4<f32>,
    @location(11) use_gradient: u32,
    @location(12) uv: vec4<f32>, // (u_min, v_min, u_max, v_max)
};

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
}

struct VertexPayload {
    @builtin(position) position: vec4<f32>,
    @location(0) texCoord: vec2<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) color_start: vec4<f32>,
    @location(3) color_end: vec4<f32>,
    @location(4) use_gradient: u32,
};

@vertex
fn vs_main(vertex: Vertex, instance: InstanceInput) -> VertexPayload {
    let model = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    var out: VertexPayload;
    out.position = model * vec4<f32>(vertex.position, 1.0);
    out.texCoord = vec2<f32>(vertex.position.x, -vertex.position.y);
    out.uv = vertex.uv;
    out.color_start = instance.color_start;
    out.color_end = instance.color_end;
    out.use_gradient = instance.use_gradient;
    return out;
}

@fragment
fn fs_main(in: VertexPayload) -> @location(0) vec4<f32> {
    var texColor = textureSample(texture, t_sampler, in.texCoord);

    if in.use_gradient == 1u {
        return texColor * mix(in.color_start, in.color_end, in.uv.x);
    }

    return texColor * in.color_start;
}
