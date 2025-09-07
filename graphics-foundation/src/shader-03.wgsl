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

fn invert (in: vec3<f32>) -> vec3<f32> {
    return 1.0 - in;
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    let uv = in.uv;

    // WGSL has auto types for simple variables
    // Variables set with let are immutable
    let r = 0.2;
    // You specify the type of a variable like this
    // Variables set with var can be changed
    var g: f32 = 0.8;
    g = uniforms.mouse.x;
    // You can convert the type of a variable like this
    let b = f32(0.4);

    // To create a vector you specify its number and type
    var col = vec3<f32>(r, g, b);

    col = mix(col, invert(col), uniforms.key1);

    return vec4<f32>(col, 1.0);
}