@group(0) @binding(0) var texture: texture_2d<f32>;
@group(0) @binding(1) var t_sampler: sampler;

struct Uniforms {
    model: mat4x4<f32>,
    color_start: vec4<f32>,
    color_end: vec4<f32>,
    use_gradient: u32,
};

@group(1) @binding(0) var<uniform> uniforms: Uniforms;

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
}

struct VertexPayload {
    @builtin(position) position: vec4<f32>,
    @location(0) texCoord: vec2<f32>,
    @location(1) uv: vec2<f32>,
};

@vertex
fn vs_main(vertex: Vertex) -> VertexPayload {
    var out: VertexPayload;
    out.position = uniforms.model * vec4<f32>(vertex.position, 1.0);
    out.texCoord = vec2<f32>(vertex.position.x, -vertex.position.y);
    out.uv = vertex.uv;
    return out;
}

@fragment
fn fs_main(in: VertexPayload) -> @location(0) vec4<f32> {
    var texColor = textureSample(texture, t_sampler, in.texCoord);

    if uniforms.use_gradient == 1u {
        return texColor * mix(uniforms.color_start, uniforms.color_end, in.uv.x);
    }

    return texColor * uniforms.color_start;
}
