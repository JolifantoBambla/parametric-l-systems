struct Camera {
    position: vec4<f32>,
    view: mat4x4<f32>,
    projection: mat4x4<f32>,
};

struct ModelTransform {
    model_matrix: mat4x4<f32>,
    normal_matrix: mat4x4<f32>,
}

struct LightSource {
    color: vec3<f32>,
    intensity: f32,
    position_or_direction: vec3<f32>,
    light_type: u32,
}

struct Material {
    albedo: vec4<f32>,
    specular_color: vec3<f32>,
    shininess: f32,
}

struct Instance {
    transform: ModelTransform,
    material: Material,
}

struct VertexInput {
    @builtin(instance_index) instance : u32,
    @location(0) position : vec3<f32>,
    @location(1) normal : vec3<f32>,
    @location(2) texcoord : vec2<f32>,
};

struct VertexOutput {
    @builtin(position) position_cs : vec4<f32>,
    @location(0) position : vec3<f32>,
    @location(1) normal : vec3<f32>,
    @location(2) albedo: vec4<f32>,
    @location(3) specular_color: vec3<f32>,
    @location(4) shininess: f32,
};

@group(0) @binding(0) var<uniform> camera: Camera;
@group(1) @binding(0) var<uniform> model_transform: ModelTransform;
@group(1) @binding(1) var<storage> instances: array<Instance>;
@group(2) @binding(0) var<uniform> ambient_light: vec4<f32>;
@group(2) @binding(1) var<storage> light_sources: array<LightSource>;

fn comput_view_projection() -> mat4x4<f32> {
    return camera.projection * camera.view;
}

fn compute_model_matrix(instance_id: u32) -> mat4x4<f32> {
    let instance = instances[instance_id];
    return model_transform.model_matrix * instance.transform.model_matrix;
}

fn compute_normal_matrix(instance_id: u32) -> mat4x4<f32> {
    let instance = instances[instance_id];
    return model_transform.normal_matrix * instance.transform.normal_matrix;
}

@vertex
fn depth_pre_pass(input: VertexInput) -> @builtin(position) vec4<f32> {
    let model_matrix = compute_model_matrix(input.instance);
    let world_position = model_matrix * vec4(input.position, 1.0);
    return comput_view_projection() * world_position;
}

fn compute_light_direction_and_distance(light_index: u32, position: vec3<f32>) -> vec4<f32> {
    if (light_sources[light_index].light_type == 0) {
        return vec4(-normalize(light_sources[light_index].position_or_direction), 1.0);
    } else {
        let direction = light_sources[light_index].position_or_direction - position;
        return vec4(normalize(direction), length(direction));
    }
}

@vertex
fn vertex_main(input: VertexInput) -> VertexOutput {
    let instance = instances[input.instance];
    let model_matrix = compute_model_matrix(input.instance);
    let world_position = model_matrix * vec4(input.position, 1.0);
    let world_normal = normalize((compute_normal_matrix(input.instance) * vec4(input.normal, 0.0)).xyz);

    var output : VertexOutput;
    output.position_cs = comput_view_projection() * world_position;
    output.position = world_position.xyz;
    output.normal = world_normal;
    output.albedo = instance.material.albedo;
    output.specular_color = instance.material.specular_color;
    output.shininess = instance.material.shininess;
    return output;
}

@fragment
fn fragment_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let position = input.position;
    let normal = normalize(input.normal);
    let albedo = input.albedo.rgb;
    let alpha = input.albedo.a;
    let specular_color = input.specular_color;
    let shininess = input.shininess;

    let view_direction = normalize(camera.position.xyz - position);

    let ambient = ambient_light.rgb;

    var diffuse = vec3<f32>();
    var specular = vec3<f32>();

    for (var i = 0u; i < arrayLength(&light_sources); i += 1) {
        let light_color = light_sources[i].color;
        let light_dir_distance = compute_light_direction_and_distance(i, position.xyz);
        let light_direction = light_dir_distance.xyz;
        let recip_distance_squared = 1. / (light_dir_distance.w * light_dir_distance.w);
        let light = light_color * light_sources[i].intensity * recip_distance_squared;

        let halfway = normalize(light_direction + view_direction);

        let lambertian = max(dot(normal, light_direction), 0.0);
        diffuse += lambertian * light;

        specular += pow(max(dot(normal, halfway), 0.0), shininess) * light;
    }
    let color = (ambient + diffuse) * albedo + specular * specular_color;

    return vec4(pow(color, vec3(1.0 / 2.2)), alpha);
}
