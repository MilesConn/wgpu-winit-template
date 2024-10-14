use bevy_crevice::{glsl::GlslStruct, std140::AsStd140};

#[derive(AsStd140, GlslStruct, Default)]
pub(crate) struct UniformBuffer {
    pub(crate) time: f32,
    _padding: [f32; 3],
}

impl UniformBuffer {
    pub fn new(time: f32) -> Self {
        UniformBuffer {
            time,
            ..Default::default()
        }
    }
}
