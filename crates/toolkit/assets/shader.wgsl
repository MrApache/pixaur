@group(0) @binding(0) var texture: texture_2d<f32>;
@group(0) @binding(1) var t_sampler: sampler;

struct Instance {
    @location(1) uv: vec4<f32>,

    @location(2) model_matrix_0: vec4<f32>,
    @location(3) model_matrix_1: vec4<f32>,
    @location(4) model_matrix_2: vec4<f32>,
    @location(5) model_matrix_3: vec4<f32>,
    @location(6) color_start: vec4<f32>,
    @location(7) color_end: vec4<f32>,
    @location(8) use_gradient: u32,
};

struct VertexPayload {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color_start: vec4<f32>,
    @location(2) color_end: vec4<f32>,
    @location(3) use_gradient: u32,
};

struct Vertex {
    @location(0) position: vec3<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) vertex_index: u32,
    vertex: Vertex,
    instance: Instance
) -> VertexPayload {

    let local_uv = vec2<f32>(
        mix(instance.uv.x, instance.uv.z, vertex.position.x),
        mix(instance.uv.w, instance.uv.y, vertex.position.y)
    );

    let model = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    var out: VertexPayload;
    out.position = model * vec4<f32>(vertex.position, 1.0);
    out.uv = local_uv;
    out.color_start = instance.color_start;
    out.color_end = instance.color_end;
    out.use_gradient = instance.use_gradient;
    return out;
}

@fragment
fn fs_main(in: VertexPayload) -> @location(0) vec4<f32> {
    var texColor = textureSample(texture, t_sampler, in.uv);

    if in.use_gradient == 1u {
        return texColor * mix(in.color_start, in.color_end, in.uv.x);
    }

    return texColor * in.color_start;
}
