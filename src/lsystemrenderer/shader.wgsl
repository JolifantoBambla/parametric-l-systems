struct Camera {
    view : mat4x4<f32>,
    projection : mat4x4<f32>,
};

struct LightSource {
    color: vec3<f32>,
    intensity: f32,
    position_or_direction: vec3<f32>,
    light_type: u32,
}

struct Instance {
    model_matrix: mat4x4<f32>,
    color: vec4<f32>,
}

struct VertexInput {
    @builtin(instance_index) instance : u32,
    @location(0) position : vec3<f32>,
    @location(1) normal : vec3<f32>,
    @location(2) texcoord : vec2<f32>,
};

struct VertexOutput {
    @builtin(position) position : vec4<f32>,
    @location(0) normal : vec3<f32>,
    @location(1) texcoord : vec2<f32>,
    @location(2) color: vec4<f32>,
};

@group(0) @binding(0) var<uniform> camera: Camera;
@group(1) @binding(0) var<storage> instances: array<Instance>;
@group(2) @binding(0) var<storage> light_sources: array<LightSource>;

fn compute_light_direction(light_index: u32, position: vec3<f32>) -> vec3<f32> {
    if light_sources[light_index].light_type == 0 {
        return light_sources[light_index].position_or_direction;
    } else {
        return position - light_sources[light_index].position_or_direction;
    }
}

@vertex
fn vertex_main(input : VertexInput) -> VertexOutput {
    var output : VertexOutput;
    let instance = instances[input.instance];
    let modelMatrix = instance.model_matrix;
    let world_position = modelMatrix * vec4(input.position, 1.0);
    let world_normal = modelMatrix * vec4(input.normal, 0.0);

    let baseColor = instance.color;
    let ambientColor = vec3(0.1, 0.1, 0.1);

    var color = vec3<f32>();
    for (var i = 0u; i < arrayLength(&light_sources); i += 1) {
        let lightDir = compute_light_direction(i, world_position.xyz);
        let lightColor = light_sources[i].color;
        let N = normalize(world_normal.xyz);
        let L = normalize(-lightDir);
        let NDotL = max(dot(N, L), 0.0);
        color += (baseColor.rgb * ambientColor) + (baseColor.rgb * NDotL);
    }

    output.position = camera.projection * camera.view * world_position;
    output.normal = normalize((camera.view * world_normal).xyz);
    output.texcoord = input.texcoord;
    output.color = vec4(color, baseColor.a);
    return output;
}

@fragment
fn fragment_main(input : VertexOutput) -> @location(0) vec4<f32> {
    return input.color;
}
