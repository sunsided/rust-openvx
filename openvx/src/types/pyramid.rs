use crate::{AsVxReference, VxReference};
use libopenvx_sys::vx_pyramid;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct VxPyramid {
    raw: vx_pyramid,
}

impl VxPyramid {
    #[allow(dead_code)]
    pub fn is_null(&self) -> bool {
        self.raw.is_null()
    }
}

impl AsVxReference for VxPyramid {
    fn as_reference(&mut self) -> VxReference {
        VxReference::from(self.raw)
    }
}

impl From<vx_pyramid> for VxPyramid {
    fn from(value: vx_pyramid) -> Self {
        VxPyramid { raw: value }
    }
}

impl Into<vx_pyramid> for VxPyramid {
    fn into(self) -> vx_pyramid {
        return self.raw;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_null() {
        assert!(VxPyramid::from(std::ptr::null_mut()).is_null());
    }
}
