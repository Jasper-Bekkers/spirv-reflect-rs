#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ReflectResourceType {
    Undefined,
    Sampler,
    CombinedImageSampler,
    ConstantBufferView,
    ShaderResourceView,
    UnorderedAccessView,
}

impl Default for ReflectResourceType {
    fn default() -> Self {
        ReflectResourceType::Undefined
    }
}
