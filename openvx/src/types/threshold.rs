use crate::{AsVxReference, VxReference};
use libopenvx_sys::vx_threshold;

/// The Threshold Object. A thresholding object contains the types and
/// limit values of the thresholding required.
#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct VxThreshold {
    raw: vx_threshold,
}

impl VxThreshold {
    #[allow(dead_code)]
    pub fn is_null(&self) -> bool {
        self.raw.is_null()
    }
}

impl AsVxReference for VxThreshold {
    fn as_reference(&self) -> VxReference {
        VxReference::from(self.raw)
    }
}

impl From<vx_threshold> for VxThreshold {
    fn from(value: vx_threshold) -> Self {
        VxThreshold { raw: value }
    }
}

impl Into<vx_threshold> for VxThreshold {
    fn into(self) -> vx_threshold {
        return self.raw;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_null() {
        assert!(VxThreshold::from(std::ptr::null_mut()).is_null());
    }
}
