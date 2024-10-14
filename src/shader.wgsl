// Vertex shader

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) vert_position: vec2<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    // [0.5, 0, -0.5]
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    // [-0.5, 0.5, -0.5]
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    let scaled_time = ub.time / 1000.0;
    let rotated = rotate(vec2<f32>(x,y), 3.0, scaled_time);

    out.vert_position = rotated;
    out.clip_position = vec4<f32>(rotated, 0.0, 1.0);
    return out;
}

// Fragment shader
struct UB {
   time : f32,
   _padding1: f32,
   _padding2: f32,
   _padding3: f32,
};

@group(0) @binding(0)
var<uniform> ub: UB;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Treat y axis as red [0,1]
    // Treat [0,1] as green
    // Treat [-1,0] as blue
    let r = (in.vert_position.y + 0.5) ;
    let g = in.vert_position.x * 2.0;
    let b = (in.vert_position.x * -2.0);
    // Modulate by [0,1]
    let rgb = vec3<f32>(r,g,b);
    return vec4<f32>(rgb, 1.0);
}

fn rotate(clip_in : vec2<f32>, frequency: f32, time: f32) -> vec2<f32> {
    let angle = f32(frequency) * time;
    let x = clip_in.x * cos(angle) - clip_in.y * sin(angle);
    let y = clip_in.x * sin(angle) + clip_in.y * cos(angle);
    return vec2<f32>(x, y);
}
