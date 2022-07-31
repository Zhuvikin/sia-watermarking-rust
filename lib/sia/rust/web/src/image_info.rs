#[derive(Debug, PartialEq, Clone)]
pub struct WatermarkedImage {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) format: String,
    pub(crate) original: Vec<u8>,
    pub(crate) watermarked: Vec<u8>,
}
