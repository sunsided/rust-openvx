use crate::{AsVxReference, VxReference};
use libopenvx_sys::vx_kernel;

/// An opaque reference to the descriptor of a kernel.
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct VxKernel {
    raw: vx_kernel,
}

impl VxKernel {
    #[allow(dead_code)]
    pub fn is_null(&self) -> bool {
        self.raw.is_null()
    }
}

impl AsVxReference for VxKernel {
    fn as_reference(&mut self) -> VxReference {
        VxReference::from(self.raw)
    }
}

impl From<vx_kernel> for VxKernel {
    fn from(value: vx_kernel) -> Self {
        VxKernel { raw: value }
    }
}

impl Into<vx_kernel> for VxKernel {
    fn into(self) -> vx_kernel {
        return self.raw;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_null() {
        assert!(VxKernel::from(std::ptr::null_mut()).is_null());
    }
}
