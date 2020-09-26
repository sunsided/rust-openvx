use crate::{AsVxReference, VxReference};
use libopenvx_sys::vx_scalar;

/// An opaque reference to a scalar.
///
/// A scalar can be up to 64 bits wide.
#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct VxScalar {
    raw: vx_scalar,
}

impl VxScalar {
    #[allow(dead_code)]
    pub fn is_null(&self) -> bool {
        self.raw.is_null()
    }
}

impl AsVxReference for VxScalar {
    fn as_reference(&self) -> VxReference {
        VxReference::from(self.raw)
    }
}

impl From<vx_scalar> for VxScalar {
    fn from(value: vx_scalar) -> Self {
        VxScalar { raw: value }
    }
}

impl Into<vx_scalar> for VxScalar {
    fn into(self) -> vx_scalar {
        return self.raw;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_null() {
        assert!(VxScalar::from(std::ptr::null_mut()).is_null());
    }
}
