struct ProjectionUniform {
    projection: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> projection: ProjectionUniform;

struct VertexInput {
    @location(0) position: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) rect_dim: vec4<f32>,
    // rect_pos: vec2<f32>
    // rect_size: vec2<f32>
    @location(2) rect_color: vec4<f32>,
    @location(3) border_radius: vec4<f32>,
    @location(4) border_size: vec4<f32>,
    @location(5) border_color: vec4<f32>,
    @location(6) outline: vec4<f32>,
    // outline_width: vec2<f32>
    // outline_offset: vec2<f32>
    @location(7) outline_color: vec4<f32>,
    @location(8) filters: vec4<f32>,
    // brightness: f32,
    // saturate: f32,
    // contrast: f32,
    // invert: f32,
    @location(9) grayscale: f32,
    @location(10) sepia: f32,
    @location(11) hue_rotate: f32,
};

struct InstanceInput {
    @location(1) dimensions: vec4<f32>,
    @location(2) rect_color: vec4<f32>,
    @location(3) border_radius: vec4<f32>,
    @location(4) border_size: vec4<f32>,
    @location(5) border_color: vec4<f32>,
    @location(6) outline: vec2<f32>,
    // outline_width: f32
    // outline_offset: f32
    @location(7) outline_color: vec4<f32>,
    @location(8) filters: vec4<f32>,
    // brightness: f32,
    // saturate: f32,
    // contrast: f32,
    // invert: f32,
    @location(9) grayscale: f32,
    @location(10) scale: vec2<f32>,
    @location(11) rotation: f32,
    @location(12) skew: vec2<f32>,
    @location(13) sepia: f32,
    @location(14) hue_rotate: f32,
}

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
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;

    let outline_width = vec2<f32>(instance.outline.x, instance.outline.x) * instance.scale;
    let outline_offset = vec2<f32>(instance.outline.y, instance.outline.y) * instance.scale;

    let scaled_dimensions = vec4<f32>(
        instance.dimensions.xy * instance.scale,
        instance.dimensions.zw * instance.scale
    );
    let position = model.position * scaled_dimensions.zw + scaled_dimensions.xy;
    out.clip_position = projection.projection * vec4<f32>(position * rotation_matrix(instance.rotation) * skew_matrix(instance.skew.x, instance.skew.y), 0.0, 1.0);

    out.uv = position;
    out.rect_color = instance.rect_color;
    out.rect_dim = vec4<f32>(
        scaled_dimensions.xy 
        + vec2<f32>(instance.border_size.x, instance.border_size.y) * instance.scale
        + vec2<f32>(outline_width + outline_offset),
        scaled_dimensions.zw 
        - vec2<f32>(
            (instance.border_size.x + instance.border_size.z) * instance.scale.x, 
            (instance.border_size.y + instance.border_size.w) * instance.scale.y
        )
        - vec2<f32>(outline_width * 2 + outline_offset * 2)
    );
    out.border_radius = instance.border_radius * vec4<f32>(instance.scale, instance.scale);
    out.border_size = vec4<f32>(instance.border_size.xy, instance.border_size.zw) * vec4<f32>(instance.scale, instance.scale);
    out.border_color = instance.border_color;
    out.outline = vec4<f32>(outline_width, outline_offset);
    out.outline_color = instance.outline_color;
    out.filters = instance.filters;
    out.grayscale = instance.grayscale;
    out.sepia = instance.sepia;
    out.hue_rotate = instance.hue_rotate;

    return out;
}

// MIT License. © 2023 Inigo Quilez, Munrocket
// https://gist.github.com/munrocket/30e645d584b5300ee69295e54674b3e4
// https://compute.toys/view/398
fn sdf_rounded_rect(p: vec2<f32>, b: vec2<f32>, r: vec4<f32>) -> f32 {
    var x = r.x;
    var y = r.y;
    x = select(r.z, r.x, p.x > 0.0);
    y = select(r.w, r.y, p.x > 0.0);
    x = select(y, x, p.y > 0.0);
    let q = abs(p) - b + x;
    return min(max(q.x, q.y), 0.0) + length(max(q, vec2<f32>(0.0))) - x;
}

fn brightness_matrix(brightness: f32) -> mat4x4<f32> {
    return mat4x4<f32>( 1, 0, 0, 0,
			0, 1, 0, 0,
			0, 0, 1, 0,
			brightness, brightness, brightness, 1 );
}

fn contrast_matrix(contrast: f32) -> mat4x4<f32> {
    let t = (1.0 - contrast) / 2.0;

    return mat4x4<f32>( contrast, 0, 0, 0,
			0, contrast, 0, 0,
			0, 0, contrast, 0,
			t, t, t, 1);
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
        dot(color, vec3<f32>(0.272, -0.715, 0.213)) * (1.0 - cos(angle)) + cos(angle) * color.b + sin(angle) * color.r,
    );
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let outline_width = in.outline.xy;
    let outline_offset = in.outline.zw;

    let brightness = in.filters[0];
    let saturate = in.filters[1];
    let contrast = in.filters[2];
    let invert = in.filters[3];

    var pos: vec2<f32> = in.rect_dim.xy;
    var size: vec2<f32> = in.rect_dim.zw;
    var dist: f32 = sdf_rounded_rect(
        in.uv - pos - (size / 2.0),
        size / 2.0,
        in.border_radius
    );
    let rect_alpha = 1.0 - smoothstep(0.0, 2.0, dist);
    var color: vec4<f32> = vec4<f32>(in.rect_color.rgb, in.rect_color.a * rect_alpha);

    if (in.border_size.x > 0.0 || in.border_size.y > 0.0 || in.border_size.w > 0.0 || in.border_size.z > 0.0) {
	size += vec2<f32>((in.border_size.x + in.border_size.z), (in.border_size.y + in.border_size.w)) / 2.0;
    	pos -= vec2<f32>(in.border_size.x, in.border_size.y) / 2.0;
    	let border_dist = sdf_rounded_rect(
    	    in.uv - pos - (size / 2.0),
    	    size / 2.0,
    	    in.border_radius
    	);
    	let border_alpha = 1.0 - smoothstep(0.0, 2.0, border_dist);
    	let border_color = vec4<f32>(in.border_color.rgb, in.border_color.a * border_alpha);

	color = mix(color, border_color, smoothstep(0.0, 1.0, dist));
	dist = border_dist;
    }

    if (outline_offset.x > 0.0 || outline_offset.y > 0.0) {
        size += outline_offset * 2.0;
    	pos -= outline_offset;
        color = mix(color, vec4<f32>(0.0), smoothstep(0.0, 1.0, dist));
    	dist = sdf_rounded_rect(
    	    in.uv - pos - (size / 2.0),
    	    size / 2.0,
    	    in.border_radius
    	);
    }

    if (outline_width.x > 0.0 || outline_width.y > 0.0) {
        size += outline_width * 2.0;
    	pos -= outline_width;
    	let outline_dist = sdf_rounded_rect(
    	    in.uv - pos - (size / 2.0),
    	    size / 2.0,
    	    in.border_radius
    	);
    	let outline_alpha = 1.0 - smoothstep(0.0, 2.0, outline_dist);
    	let outline_color = vec4<f32>(in.outline_color.rgb, in.outline_color.a * outline_alpha);

        color = mix(color, outline_color, smoothstep(0.0, 1.0, dist));
        dist = outline_dist;
    }

    color = brightness_matrix(brightness) * contrast_matrix(contrast) * saturation_matrix(saturate) * color;

    let hue_rotate = hue_rotate(color.rgb, in.hue_rotate);
    let sepia = sepia(hue_rotate, in.sepia);
    return vec4<f32>(mix(sepia, vec3<f32>(1.0) - sepia, invert), color.a);
}
