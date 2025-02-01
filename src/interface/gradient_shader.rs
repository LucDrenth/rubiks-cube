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
}

impl Into<BackgroundGradientMaterial> for BackgroundGradientMaterialBuilder {
    fn into(self) -> BackgroundGradientMaterial {
        let mut colors = [Color::default(); MAX_NUMBER_OF_COLORS];
        for (i, color) in self.colors.iter().enumerate() {
            colors[i] = *color;
        }

        BackgroundGradientMaterial {
            colors: colors.map(|c| c.to_linear().to_f32_array().into()),
            number_of_colors: self.colors.len() as u32,
            offset: self.offset,
            scroll_speed: self.scroll_speed,
            gradient_type: self.gradient_type.into(),
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
