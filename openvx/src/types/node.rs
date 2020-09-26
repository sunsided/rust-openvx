use crate::{AsVxReference, VxReference};
use libopenvx_sys::vx_node;

/// An opaque reference to a kernel node.
#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct VxNode {
    raw: vx_node,
}

impl VxNode {
    #[allow(dead_code)]
    pub fn is_null(&self) -> bool {
        self.raw.is_null()
    }
}

impl AsVxReference for VxNode {
    fn as_reference(&self) -> VxReference {
        VxReference::from(self.raw)
    }
}

impl From<vx_node> for VxNode {
    fn from(value: vx_node) -> Self {
        VxNode { raw: value }
    }
}

impl Into<vx_node> for VxNode {
    fn into(self) -> vx_node {
        return self.raw;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_null() {
        assert!(VxNode::from(std::ptr::null_mut()).is_null());
    }
}
