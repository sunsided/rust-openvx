use crate::{AsVxReference, VxReference};
use libopenvx_sys::vx_graph;

/// An opaque reference to a graph.
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct VxGraph {
    raw: vx_graph,
}

impl VxGraph {
    #[allow(dead_code)]
    pub fn is_null(&self) -> bool {
        self.raw.is_null()
    }
}

impl AsVxReference for VxGraph {
    fn as_reference(&mut self) -> VxReference {
        VxReference::from(self.raw)
    }
}

impl From<vx_graph> for VxGraph {
    fn from(value: vx_graph) -> Self {
        VxGraph { raw: value }
    }
}

impl Into<vx_graph> for VxGraph {
    fn into(self) -> vx_graph {
        return self.raw;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_null() {
        assert!(VxGraph::from(std::ptr::null_mut()).is_null());
    }
}
