use crate::{AsVxReference, VxReference};
use libopenvx_sys::vx_parameter;

/// An opaque reference to a single parameter.
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct VxParameter {
    raw: vx_parameter,
}

impl VxParameter {
    #[allow(dead_code)]
    pub fn is_null(&self) -> bool {
        self.raw.is_null()
    }
}

impl AsVxReference for VxParameter {
    fn as_reference(&mut self) -> VxReference {
        VxReference::from(self.raw)
    }
}

impl From<vx_parameter> for VxParameter {
    fn from(value: vx_parameter) -> Self {
        VxParameter { raw: value }
    }
}

impl Into<vx_parameter> for VxParameter {
    fn into(self) -> vx_parameter {
        return self.raw;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_null() {
        assert!(VxParameter::from(std::ptr::null_mut()).is_null());
    }
}
