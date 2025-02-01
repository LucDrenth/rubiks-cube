use bevy::{prelude::*, render::render_resource::*};

// should match what is declared in the shader
const MAX_NUMBER_OF_COLORS: usize = 8;
const SHADER_PATH: &str = "shaders/ui_background_gradient.wgsl";

pub struct GradientShaderPlugin;

impl Plugin for GradientShaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiMaterialPlugin::<BackgroundGradientMaterial>::default())
            .add_systems(Update, animate_scroll);
    }
}

#[derive(Default)]
pub enum GradientType {
    #[default]
    Linear,
    Block,
}

impl Into<u32> for GradientType {
    fn into(self) -> u32 {
        match self {
            // values should match the ones in ui_background_gradient.wgsl
            GradientType::Linear => 0,
            GradientType::Block => 1,
        }
    }
}

#[derive(Default)]
pub struct BackgroundGradientMaterialBuilder {
    colors: Vec<Color>,
    offset: f32,
    scroll_speed: f32,
    gradient_type: GradientType,
    color_size: Option<ColorSize>,
}

pub enum ColorSize {
    Repeat(f32),
    WidthPerColor(f32),
}

impl BackgroundGradientMaterialBuilder {
    pub fn with_colors(mut self, colors: Vec<Color>) -> Result<Self, String> {
        if colors.len() > MAX_NUMBER_OF_COLORS {
            return Err(format!(
                "number of colors may not exceed {MAX_NUMBER_OF_COLORS}",
            ));
        }

        self.colors = colors;
        Ok(self)
    }
    pub fn with_offset(mut self, offset: f32) -> Self {
        self.offset = offset;
        self
    }
    pub fn with_scroll_speed(mut self, scroll_speed: f32) -> Self {
        self.scroll_speed = scroll_speed;
        self
    }
    pub fn with_gradient_type(mut self, gradient_type: GradientType) -> Self {
        self.gradient_type = gradient_type;
        self
    }
    pub fn with_color_size(mut self, color_size: ColorSize) -> Self {
        self.color_size = Some(color_size);
        self
    }
}

impl Into<BackgroundGradientMaterial> for BackgroundGradientMaterialBuilder {
    fn into(self) -> BackgroundGradientMaterial {
        let mut colors = [Color::default(); MAX_NUMBER_OF_COLORS];
        for (i, color) in self.colors.iter().enumerate() {
            colors[i] = *color;
        }

        let color_width = self.color_size.unwrap_or(ColorSize::Repeat(1.0));
        let width_per_color = match color_width {
            ColorSize::Repeat(repeat) => 1.0 / self.colors.len() as f32 / repeat,
            ColorSize::WidthPerColor(w) => w,
        };

        BackgroundGradientMaterial {
            colors: colors.map(|c| c.to_linear().to_f32_array().into()),
            number_of_colors: self.colors.len() as u32,
            offset: self.offset,
            scroll_speed: self.scroll_speed,
            gradient_type: self.gradient_type.into(),
            width_per_color,
        }
    }
}

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct BackgroundGradientMaterial {
    #[uniform(0)]
    pub colors: [Vec4; MAX_NUMBER_OF_COLORS],
    #[uniform(1)]
    pub number_of_colors: u32,
    #[uniform(2)]
    pub offset: f32,
    pub scroll_speed: f32,
    #[uniform(3)]
    gradient_type: u32,
    #[uniform(4)]
    width_per_color: f32,
}

impl UiMaterial for BackgroundGradientMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_PATH.into()
    }
}

fn animate_scroll(
    mut materials: ResMut<Assets<BackgroundGradientMaterial>>,
    query: Query<&MaterialNode<BackgroundGradientMaterial>>,
    time: Res<Time>,
) {
    for handle in &query {
        let Some(material) = materials.get_mut(handle) else {
            continue;
        };

        material.offset -= time.delta_secs() * material.scroll_speed;
    }
}
