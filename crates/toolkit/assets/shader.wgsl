@group(0) @binding(0) var texture: texture_2d<f32>;
@group(0) @binding(1) var t_sampler: sampler;

struct Instance {
    @location(1) uv: vec4<f32>,
    @location(2) size: vec2<f32>,

    @location(3) model_matrix_0: vec4<f32>,
    @location(4) model_matrix_1: vec4<f32>,
    @location(5) model_matrix_2: vec4<f32>,
    @location(6) model_matrix_3: vec4<f32>,
    @location(7) color: vec4<f32>,

    @location(8) stroke_color: vec4<f32>,
    @location(9) stroke_width: f32,
    @location(10) stroke_corners: vec4<f32>,

    @location(11) color_end: vec4<f32>,
    @location(12) degree: f32,

    @location(13) use_gradient: u32,
    @location(14) support_stroke: u32,
};

struct VertexPayload {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) size: vec2<f32>,
    @location(2) color: vec4<f32>,

    @location(3) stroke_color: vec4<f32>,
    @location(4) stroke_width: f32,
    @location(5) stroke_corners: vec4<f32>,
    @location(6) support_stroke: u32,

    @location(7) color_end: vec4<f32>,
    @location(8) degree: f32,
    @location(9) use_gradient: u32,
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
        mix(instance.uv.y, instance.uv.w, vertex.position.y)
    );

    let model = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    var out: VertexPayload;

    out.uv = local_uv;
    out.position = model * vec4<f32>(vertex.position, 1.0);
    out.size = instance.size;
    out.color = instance.color;

    out.stroke_color = instance.stroke_color;
    out.stroke_width = instance.stroke_width;
    out.stroke_corners = instance.stroke_corners;

    out.color_end = instance.color_end;
    out.degree = instance.degree;
    out.use_gradient = instance.use_gradient;
    out.support_stroke = instance.support_stroke;

    return out;
}

@fragment
fn fs_main(in: VertexPayload) -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
    //var texColor = textureSample(texture, t_sampler, in.uv);

    //if in.use_gradient >= 1u {
    //    let angle = in.degree * 3.14159265 / 180.0;
    //    let dir = vec2<f32>(cos(angle), sin(angle));
    //
    //    let centered_uv = in.uv - vec2<f32>(0.5, 0.5);
    //    let max_len = 0.707;
    //    let raw_t = dot(centered_uv, dir);
    //    let t = clamp((raw_t / max_len + 1.0) * 0.5, 0.0, 1.0);
    //
    //    return texColor * mix(in.color, in.color_end, t);
    //}

    //return texColor * in.color;
}

//@fragment
//fn fs_main(in: VertexPayload) -> @location(0) vec4<f32> {
//    let texColor = textureSample(texture, t_sampler, in.uv);
//    var baseColor = texColor * in.color;
//
//    if in.use_gradient >= 1u {
//        let angle = in.degree * 3.14159265 / 180.0;
//        let dir = vec2<f32>(cos(angle), sin(angle));
//        let centered_uv = in.uv - vec2<f32>(0.5);
//        let max_len = 0.707;
//        let raw_t = dot(centered_uv, dir);
//        let t = clamp((raw_t / max_len + 1.0) * 0.5, 0.0, 1.0);
//        baseColor = texColor * mix(in.color, in.color_end, t);
//    }
//
//    // Обводка (если включена)
//    if in.stroke_width > 0.0 && in.support_stroke >= 1u {
//        // Переводим пиксели в UV-координаты (нормализуем)
//        let stroke_uv = in.stroke_width / in.size.x; // Используем width как базовый размер
//        let aa_uv = 1.0 / in.size.x; // Антиалиасинг ~1 пиксель
//        
//        let dist_to_edge = min(
//            min(in.uv.x, 1.0 - in.uv.x),
//            min(in.uv.y, 1.0 - in.uv.y)
//        );
//        
//        let stroke_factor = 1.0 - smoothstep(
//            stroke_uv - aa_uv,
//            stroke_uv + aa_uv,
//            dist_to_edge
//        );
//        
//        return baseColor;
//        //return mix(baseColor, in.stroke_color, stroke_factor);
//    }
//
//    return baseColor;
//}
