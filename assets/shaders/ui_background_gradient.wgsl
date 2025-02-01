#import bevy_ui::ui_vertex_output::UiVertexOutput

const MAX_NUMBER_OF_COLOURS: i32 = 8;
const GRADIENT_TYPE_LINEAR: u32 = 0;
const GRADIENT_TYPE_BLOCKS: u32 = 1;

@group(1) @binding(0) var<uniform> colors: array<vec4<f32>, MAX_NUMBER_OF_COLOURS>;
@group(1) @binding(1) var<uniform> number_of_colors: u32;
@group(1) @binding(2) var<uniform> offset: f32;
@group(1) @binding(3) var<uniform> gradient_type: u32;
@group(1) @binding(4) var<uniform> width_per_color: f32;

struct ColorRange {
    left: vec4<f32>,
    right: vec4<f32>,
}

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    if gradient_type == GRADIENT_TYPE_LINEAR {
        return linear(in);
    } else if gradient_type == GRADIENT_TYPE_BLOCKS {
        return blocks(in);
    } else {
        discard;
    }
}

fn blocks(in: UiVertexOutput) -> vec4<f32> {
    let range = width_per_color * f32(number_of_colors);

    var position = in.uv.x + offset;
    if position < 0.0 {
        // loop back around
        position += ceil(abs(position) / range) * range;
    }

    let value: f32 = position % range;
    let color_index = i32(floor(value / width_per_color));
    return colors[color_index];
}

// TODO implement `width_per_color`
fn linear(in: UiVertexOutput) -> vec4<f32> {
    var progress = fract(in.uv.x + offset);
    progress *= f32(number_of_colors);

    let amount_low = ceil(progress) - progress;
    let amount_high = progress - floor(progress);
    let base_color_range = get_base_color(progress);

    let color_left = base_color_range.left * amount_low;
    let color_right = base_color_range.right * amount_high;

    return vec4(color_left + color_right);
}

fn get_base_color(progress: f32) -> ColorRange {
    if progress < f32(number_of_colors - 1) {
        return ColorRange(
            colors[i32(floor(progress))],
            colors[i32(ceil(progress))],
        );
    } else {
        return ColorRange(
            colors[number_of_colors - 1],
            colors[0],
        );
    }
}
