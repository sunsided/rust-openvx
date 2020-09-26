use crate::{AsVxReference, VxReference};
use libopenvx_sys::vx_matrix;

/// The Matrix Object. An MxN matrix of some unit type.
#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct VxMatrix {
    raw: vx_matrix,
}

impl VxMatrix {
    #[allow(dead_code)]
    pub fn is_null(&self) -> bool {
        self.raw.is_null()
    }
}

impl AsVxReference for VxMatrix {
    fn as_reference(&self) -> VxReference {
        VxReference::from(self.raw)
    }
}

impl From<vx_matrix> for VxMatrix {
    fn from(value: vx_matrix) -> Self {
        VxMatrix { raw: value }
    }
}

impl Into<vx_matrix> for VxMatrix {
    fn into(self) -> vx_matrix {
        return self.raw;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_null() {
        assert!(VxMatrix::from(std::ptr::null_mut()).is_null());
    }
}
