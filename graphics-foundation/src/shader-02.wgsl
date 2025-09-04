struct Uniforms {
    time: f32;
    key1: f32;
    mouse: vec2<f32>;
};

[[group(0), binding(0)]]
var<uniform> uniforms: Uniforms;

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] uv: vec2<f32>;
};

[[stage(vertex)]]
fn vs_main([[builtin(vertex_index)]] in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;

    let x = f32(i32(in_vertex_index) % 2) * 2.0 - 1.0;
    let y = f32(i32(in_vertex_index) / 2) * 2.0 - 1.0;
    
    out.clip_position = vec4<f32>(x, -y, 0.0, 1.0);
    out.uv = vec2<f32>((x + 1.0) / 2.0, (y + 1.0) / 2.0);

    return out;
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    let uv = in.uv;

    let dist = distance(uv, uniforms.mouse);

    let pulse = sin(uniforms.time * 2.0) * 0.5 + 0.5;
    let circle = 1.0 - step(0.1 + pulse * 0.1, dist);

    let col = mix(vec3<f32>(circle), 1.0 - vec3<f32>(circle), uniforms.key1);

    return vec4<f32>(col.rgb, 1.0);
}