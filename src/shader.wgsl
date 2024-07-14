// Vertex shader

struct WindowProps {
  scale: vec2f,
  offset: vec2f,
};

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@group(0)
@binding(0)
var<uniform> props: WindowProps;

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.clip_position = vec4<f32>(model.position * props.scale + props.offset, 0.0, 1.0);
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}

// struct OurStruct {
//   color: vec4f,
//   scale: vec2f,
//   offset: vec2f,
// };

// @group(0)
// @binding(0)
// var<uniform> ourStruct: OurStruct;

// @vertex 
// fn vs_main(
//     @builtin(vertex_index) vertexIndex: u32
// ) -> @builtin(position) vec4f {
//     var pos = array(
//         vec2f(0.0, 0.5),  // top center
//         vec2f(-0.5, -0.5),  // bottom left
//         vec2f(0.5, -0.5)   // bottom right
//     );

//     return vec4f(
//         pos[vertexIndex] * ourStruct.scale + ourStruct.offset, 0.0, 1.0
//     );
// }

// @fragment
// fn fs_main() -> @location(0) vec4f {
//     return ourStruct.color;
// }