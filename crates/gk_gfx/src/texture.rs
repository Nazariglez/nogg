pub trait GKTexture {
    fn size(&self) -> (f32, f32);
}

#[derive(Debug, Default, Copy, Clone)]
pub struct TextureDescriptor<'a> {
    pub label: Option<&'a str>,
    pub format: TextureFormat,
}

#[derive(Debug, Default, Copy, Clone)]
pub enum TextureFormat {
    #[default]
    Rgba8UnormSrgb,
    // Depth16,
    Depth32Float,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct TextureData<'a> {
    pub bytes: &'a [u8],
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Default, Copy, Clone)]
pub enum TextureWrap {
    #[default]
    Clamp,
    Repeat,
    MirrorRepeat,
}

#[derive(Debug, Default, Copy, Clone)]
pub enum TextureFilter {
    #[default]
    Linear,
    Nearest,
}

pub trait GKSampler {}

#[derive(Debug, Default, Copy, Clone)]
pub struct SamplerDescriptor<'a> {
    pub label: Option<&'a str>,
    pub wrap_x: TextureWrap,
    pub wrap_y: TextureWrap,
    pub wrap_z: TextureWrap,
    pub mag_filter: TextureFilter,
    pub min_filter: TextureFilter,
    pub mipmap_filter: Option<TextureFilter>,
}
