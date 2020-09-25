use crate::{AsVxReference, VxReference};
use libopenvx_sys::vx_delay;

/// The delay object. This is like a ring buffer of objects that is
/// maintained by the OpenVX implementation.
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct VxDelay {
    raw: vx_delay,
}

impl VxDelay {
    #[allow(dead_code)]
    pub fn is_null(&self) -> bool {
        self.raw.is_null()
    }
}

impl AsVxReference for VxDelay {
    fn as_reference(&mut self) -> VxReference {
        VxReference::from(self.raw)
    }
}

impl From<vx_delay> for VxDelay {
    fn from(value: vx_delay) -> Self {
        VxDelay { raw: value }
    }
}

impl Into<vx_delay> for VxDelay {
    fn into(self) -> vx_delay {
        return self.raw;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_null() {
        assert!(VxDelay::from(std::ptr::null_mut()).is_null());
    }
}
