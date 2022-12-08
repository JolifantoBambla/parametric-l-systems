struct Camera {
    view : mat4x4<f32>,
    projection : mat4x4<f32>,
};

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

@vertex
fn vertex_main(input : VertexInput) -> VertexOutput {
    var output : VertexOutput;
    let instance = instances[input.instance];
    let modelMatrix = instance.model_matrix;
    let world_normal = (modelMatrix * vec4(input.normal, 0.0)).xyz;

    // Some hardcoded lighting
    let lightDir = vec3(0.25, 0.5, -1.0);
    let lightColor = vec3(1.0, 1.0, 1.0);
    let ambientColor = vec3(0.1, 0.1, 0.1);

    //let baseColor = textureSample(baseColorTexture, materialSampler, input.texcoord) * material.baseColorFactor;
    let baseColor = instance.color;
    // An extremely simple directional lighting model, just to give our model some shape.
    let N = normalize(world_normal);
    let L = normalize(lightDir);
    let NDotL = max(dot(N, L), 0.0);
    let surfaceColor = (baseColor.rgb * ambientColor) + (baseColor.rgb * NDotL);
    let color = vec4(surfaceColor, baseColor.a);

    output.position = camera.projection * camera.view * modelMatrix * vec4(input.position, 1.0);
    output.normal = normalize((camera.view * vec4(world_normal, 0.0)).xyz);
    output.texcoord = input.texcoord;
    output.color = color;
    return output;
}

@fragment
fn fragment_main(input : VertexOutput) -> @location(0) vec4<f32> {
    return input.color;
}
