use crate::{AsVxReference, VxReference};
use libopenvx_sys::vx_convolution;

/// The Convolution Object. A user-defined convolution kernel of MxM elements.
#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct VxConvolution {
    raw: vx_convolution,
}

impl VxConvolution {
    #[allow(dead_code)]
    pub fn is_null(&self) -> bool {
        self.raw.is_null()
    }
}

impl AsVxReference for VxConvolution {
    fn as_reference(&self) -> VxReference {
        VxReference::from(self.raw)
    }
}

impl From<vx_convolution> for VxConvolution {
    fn from(value: vx_convolution) -> Self {
        VxConvolution { raw: value }
    }
}

impl Into<vx_convolution> for VxConvolution {
    fn into(self) -> vx_convolution {
        return self.raw;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_null() {
        assert!(VxConvolution::from(std::ptr::null_mut()).is_null());
    }
}
