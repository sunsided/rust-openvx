use crate::{AsVxReference, VxReference};
use libopenvx_sys::vx_distribution;

/// The Distribution object. This has a user-defined number of bins over
/// a user-defined range (within a uint32_t range).
#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct VxDistribution {
    raw: vx_distribution,
}

impl VxDistribution {
    #[allow(dead_code)]
    pub fn is_null(&self) -> bool {
        self.raw.is_null()
    }
}

impl AsVxReference for VxDistribution {
    fn as_reference(&mut self) -> VxReference {
        VxReference::from(self.raw)
    }
}

impl From<vx_distribution> for VxDistribution {
    fn from(value: vx_distribution) -> Self {
        VxDistribution { raw: value }
    }
}

impl Into<vx_distribution> for VxDistribution {
    fn into(self) -> vx_distribution {
        return self.raw;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_null() {
        assert!(VxDistribution::from(std::ptr::null_mut()).is_null());
    }
}
