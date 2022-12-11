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
    let model_matrix = instance.model_matrix;
    let world_position = model_matrix * vec4(input.position, 1.0);
    let world_normal = normalize((model_matrix * vec4(input.normal, 0.0)).xyz);

    let object_color = instance.color;
    let ambient_color = vec3(0.1, 0.1, 0.1);

    var color = (object_color.rgb * ambient_color);
    for (var i = 0u; i < arrayLength(&light_sources); i += 1) {
        let light_dir = -normalize(compute_light_direction(i, world_position.xyz));
        let light_color = light_sources[i].color;
        let lambertian = max(dot(world_normal, light_dir), 0.0);
        color += (object_color.rgb * light_color * lambertian);
    }

    output.position = camera.projection * camera.view * world_position;
    output.normal = normalize((camera.view * vec4(world_normal, 0.0)).xyz);
    output.texcoord = input.texcoord;
    output.color = vec4(color, object_color.a);
    return output;
}

@fragment
fn fragment_main(input : VertexOutput) -> @location(0) vec4<f32> {
    return input.color;
}
