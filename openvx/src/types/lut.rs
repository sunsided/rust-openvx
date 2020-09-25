use crate::{AsVxReference, VxReference};
use libopenvx_sys::vx_lut;

/// The Look-Up Table (LUT) Object.
#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct VxLut {
    raw: vx_lut,
}

impl VxLut {
    #[allow(dead_code)]
    pub fn is_null(&self) -> bool {
        self.raw.is_null()
    }
}

impl AsVxReference for VxLut {
    fn as_reference(&mut self) -> VxReference {
        VxReference::from(self.raw)
    }
}

impl From<vx_lut> for VxLut {
    fn from(value: vx_lut) -> Self {
        VxLut { raw: value }
    }
}

impl Into<vx_lut> for VxLut {
    fn into(self) -> vx_lut {
        return self.raw;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_null() {
        assert!(VxLut::from(std::ptr::null_mut()).is_null());
    }
}
