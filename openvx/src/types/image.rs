use crate::{AsVxReference, VxReference};
use libopenvx_sys::vx_image;

/// An opaque reference to an image.
#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct VxImage {
    raw: vx_image,
}

impl VxImage {
    #[allow(dead_code)]
    pub fn is_null(&self) -> bool {
        self.raw.is_null()
    }
}

impl AsVxReference for VxImage {
    fn as_reference(&mut self) -> VxReference {
        VxReference::from(self.raw)
    }
}

impl From<vx_image> for VxImage {
    fn from(value: vx_image) -> Self {
        VxImage { raw: value }
    }
}

impl Into<vx_image> for VxImage {
    fn into(self) -> vx_image {
        return self.raw;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_null() {
        assert!(VxImage::from(std::ptr::null_mut()).is_null());
    }
}
