struct ProjectionUniform {
    projection: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> projection: ProjectionUniform;

struct InstanceData {
    rect_pos: vec2<f32>,
    rect_size: vec2<f32>,
    outline_width: f32,
    outline_offset: f32,
    scale: vec2<f32>,
    skew: vec2<f32>,
    rotation: f32,
    invert: f32,
    brightness: f32,
    saturate: f32,
    contrast: f32,
    grayscale: f32,
    sepia: f32,
    hue_rotate: f32,
    rect_color: vec4<f32>,
    outline_color: vec4<f32>,
    border_radius: vec4<f32>,
    border_size: vec4<f32>,
    border_top_color: vec4<f32>,
    border_right_color: vec4<f32>,
    border_bottom_color: vec4<f32>,
    border_left_color: vec4<f32>,
};
@group(1) @binding(1)
var<storage, read> instance_data: array<InstanceData>;

struct VertexInput {
    @location(0) position: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) rect_pos: vec2<f32>,
    @location(2) rect_size: vec2<f32>,
    @location(3) border_radius: vec4<f32>,
    @location(4) border_size: vec4<f32>,
    @location(5) outline_width: vec2<f32>,
    @location(6) outline_offset: vec2<f32>,
    @location(7) instance_index: u32,
};

fn rotation_matrix(angle: f32) -> mat2x2<f32> {
    let angle_inner = angle * 3.14159265359 / 180.0;
    let sinTheta = sin(angle_inner);
    let cosTheta = cos(angle_inner);
    return mat2x2<f32>(
        cosTheta, -sinTheta,
        sinTheta, cosTheta
    );
}

fn skew_matrix(skewX: f32, skewY: f32) -> mat2x2<f32> {
    return mat2x2<f32>(
        vec2<f32>(1.0, skewY * 3.14159265359 / 180.0),
        vec2<f32>(skewX * 3.14159265359 / 180.0, 1.0)
    );
}

@vertex
fn vs_main(
    model: VertexInput,
    @builtin(instance_index) instanceIndex: u32,
) -> VertexOutput {
    var out: VertexOutput;
    let instance = instance_data[instanceIndex];

    let scale = instance.scale;

    let scaled_dimensions = vec4<f32>(
        instance.rect_pos * scale,
        instance.rect_size * scale
    );

    let position = model.position * scaled_dimensions.zw + scaled_dimensions.xy;

    out.clip_position = projection.projection * vec4<f32>(
        position * rotation_matrix(instance.rotation) * skew_matrix(instance.skew.x, instance.skew.y), 
        0.0, 
        1.0
    );

    let outline_width = vec2<f32>(instance.outline_width, instance.outline_width) * scale;
    let outline_offset = vec2<f32>(instance.outline_offset, instance.outline_offset) * scale;

    let border_size = instance.border_size * vec4<f32>(scale, scale);

    out.uv = position;
    out.rect_pos = scaled_dimensions.xy + vec2<f32>(
        border_size.x, 
        border_size.y
    ) * scale + vec2<f32>(
        outline_width + outline_offset
    );
    out.rect_size = scaled_dimensions.zw - vec2<f32>(
        border_size.x + border_size.z,
        border_size.y + border_size.w
    ) * scale - vec2<f32>(
        outline_width + outline_offset
    ) * 2;
    out.border_radius = instance.border_radius * vec4<f32>(scale, scale);
    out.border_size = border_size;
    out.outline_width = outline_width;
    out.outline_offset = outline_offset;
    out.instance_index = instanceIndex;

    return out;
}

fn sdf_rounded_rect(p: vec2<f32>, b: vec2<f32>, r: vec4<f32>) -> f32 {
    var x = r.x;
    var y = r.y;

    x = select(r.z, r.x, p.x > 0.0);
    y = select(y, x, p.y > 0.0);

    let q = abs(p) - b + x;
    return min(max(q.x, q.y), 0.0) + length(max(q, vec2<f32>(0.0))) - x;
}

fn brightness_matrix(brightness: f32) -> mat4x4<f32> {
    return mat4x4<f32>(
        1, 0, 0, 0,
        0, 1, 0, 0,
        0, 0, 1, 0,
        brightness, brightness, brightness, 1
    );
}

fn contrast_matrix(contrast: f32) -> mat4x4<f32> {
    let t = (1.0 - contrast) / 2.0;
    return mat4x4<f32>(
        contrast, 0, 0, 0,
        0, contrast, 0, 0,
        0, 0, contrast, 0,
        t, t, t, 1
    );
}

fn saturation_matrix(saturation: f32) -> mat4x4<f32> {
    let luminance = vec3<f32>(0.3086, 0.6094, 0.0820);
    let one_minus_sat = 1.0 - saturation;

    var red: vec3<f32> = vec3<f32>(luminance.x * one_minus_sat);
    red += vec3<f32>(saturation, 0, 0);

    var green: vec3<f32> = vec3<f32>(luminance.y * one_minus_sat);
    green += vec3<f32>(0, saturation, 0);

    var blue: vec3<f32> = vec3<f32>(luminance.z * one_minus_sat);
    blue += vec3<f32>(0, 0, saturation);

    return mat4x4<f32>(
        vec4<f32>(red, 0.0),
        vec4<f32>(green, 0.0),
        vec4<f32>(blue, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 1.0)
    );
}

fn sepia(color: vec3<f32>, sepia: f32) -> vec3<f32> {
    let sepia_matrix = vec3<f32>(
        dot(color.rgb, vec3<f32>(0.393, 0.769, 0.189)),
        dot(color.rgb, vec3<f32>(0.349, 0.686, 0.168)),
        dot(color.rgb, vec3<f32>(0.272, 0.534, 0.131))
    );
    return mix(color.rgb, sepia_matrix, sepia);
}

fn hue_rotate(color: vec3<f32>, angle: f32) -> vec3<f32> {
    return vec3<f32>(
        dot(color, vec3<f32>(0.213, 0.715, -0.213)) * (1.0 - cos(angle)) + cos(angle) * color.r + sin(angle) * color.b,
        dot(color, vec3<f32>(-0.213, 0.715, 0.715)) * (1.0 - cos(angle)) + cos(angle) * color.g + sin(angle) * color.g,
        dot(color, vec3<f32>(0.272, -0.715, 0.213)) * (1.0 - cos(angle)) + cos(angle) * color.b + sin(angle) * color.r
    );
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let instance = instance_data[in.instance_index];

    // Rectangle
    var pos: vec2<f32> = in.rect_pos;
    var size: vec2<f32> = in.rect_size;
    var dist: f32 = sdf_rounded_rect(
        in.uv - pos - (size / 2.0),
        size / 2.0,
        in.border_radius
    );
    let rect_alpha = 1.0 - smoothstep(0.0, 2.0, dist);
    var color: vec4<f32> = vec4<f32>(instance.rect_color.rgb, instance.rect_color.a * rect_alpha);

    // Border
    size += vec2<f32>((in.border_size.x + in.border_size.z), (in.border_size.y + in.border_size.w)) / 2.0;
    pos -= vec2<f32>(in.border_size.x, in.border_size.y) / 2.0;
    let border_dist = sdf_rounded_rect(
        in.uv - pos - (size / 2.0),
        size / 2.0,
        in.border_radius
    );
    let border_alpha = 1.0 - smoothstep(0.0, 2.0, border_dist);
    let is_top = (in.uv.y > pos.y + size.y - in.border_size.y);
    let is_bottom = (in.uv.y < pos.y + in.border_size.w);
    let is_left = (in.uv.x < pos.x + in.border_size.x);
    let is_right = (in.uv.x > pos.x + size.x - in.border_size.z);
    let border_color = select(
        select(
            instance.border_top_color,
            instance.border_bottom_color,
            is_bottom
        ),
        select(
            instance.border_left_color,
            instance.border_right_color,
            is_right
        ),
        is_left || is_right
    );

    color = mix(color, vec4<f32>(border_color.rgb, border_color.a * border_alpha), smoothstep(0.0, 1.0, dist));
    dist = border_dist;

    // Outline offset
    size += in.outline_offset * 2.0;
    pos -= in.outline_offset;
    color = mix(color, vec4<f32>(0.0), smoothstep(0.0, 1.0, dist));
    dist = sdf_rounded_rect(
        in.uv - pos - (size / 2.0),
        size / 2.0,
        in.border_radius
    );
    
    // Outline width
    size += in.outline_width * 2.0;
    pos -= in.outline_width;
    let outline_dist = sdf_rounded_rect(
        in.uv - pos - (size / 2.0),
        size / 2.0,
        in.border_radius
    );
    let outline_alpha = 1.0 - smoothstep(0.0, 2.0, outline_dist);
    let outline_color = vec4<f32>(instance.outline_color.rgb, instance.outline_color.a * outline_alpha);
    color = mix(color, outline_color, smoothstep(0.0, 1.0, dist));
    dist = outline_dist;

    color = brightness_matrix(instance.brightness) 
          * contrast_matrix(instance.contrast) 
          * saturation_matrix(instance.saturate) 
          * color;
    let hue_rotate = hue_rotate(color.rgb, instance.hue_rotate);
    let sepia_color = sepia(hue_rotate, instance.sepia);

    return vec4<f32>(mix(sepia_color, vec3<f32>(1.0) - sepia_color, instance.invert), color.a);
}
