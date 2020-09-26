use crate::{AsVxReference, Release, Result, VxGraphState, VxReference, VxStatus};
use libopenvx_sys::{
    vxQueryGraph, vxReleaseGraph, vx_enum, vx_graph, vx_graph_attribute_e_VX_GRAPH_NUMNODES,
    vx_graph_attribute_e_VX_GRAPH_NUMPARAMETERS, vx_graph_attribute_e_VX_GRAPH_PERFORMANCE,
    vx_graph_attribute_e_VX_GRAPH_STATE, vx_graph_state_e, vx_perf_t, vx_size, vx_uint32,
};

/// An opaque reference to a graph.
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct VxGraph {
    raw: vx_graph,
}

impl VxGraph {
    pub fn as_raw(&self) -> vx_graph {
        self.raw
    }

    #[allow(dead_code)]
    pub fn is_null(&self) -> bool {
        self.raw.is_null()
    }

    /// Returns the number of nodes in the graph.
    pub fn get_num_nodes(&self) -> usize {
        let mut num_nodes: vx_uint32 = 0;

        unsafe {
            vxQueryGraph(
                self.raw,
                vx_graph_attribute_e_VX_GRAPH_NUMNODES as vx_enum,
                &mut num_nodes as *mut _ as *mut std::ffi::c_void,
                std::mem::size_of_val(&num_nodes) as vx_size,
            );
        }

        num_nodes as usize
    }

    /// Returns the number of explicitly declared parameters on the graph.
    pub fn get_num_parameters(&self) -> usize {
        let mut num_params: vx_uint32 = 0;

        unsafe {
            vxQueryGraph(
                self.raw,
                vx_graph_attribute_e_VX_GRAPH_NUMPARAMETERS as vx_enum,
                &mut num_params as *mut _ as *mut std::ffi::c_void,
                std::mem::size_of_val(&num_params) as vx_size,
            );
        }

        num_params as usize
    }

    /// Returns the state of the graph.
    pub fn get_state(&self) -> VxGraphState {
        let mut state: vx_graph_state_e = 0 as vx_graph_state_e;

        unsafe {
            vxQueryGraph(
                self.raw,
                vx_graph_attribute_e_VX_GRAPH_STATE as vx_enum,
                &mut state as *mut _ as *mut std::ffi::c_void,
                std::mem::size_of_val(&state) as vx_size,
            );
        }

        VxGraphState::from(state)
    }

    /// Returns the overall performance of the graph.
    ///
    /// The accuracy of timing information is platform dependent.
    /// Performance tracking must have been enabled.
    /// TODO: \\ref vx_directive_e
    pub fn get_performance(&self) -> () {
        let mut perf = vx_perf_t {
            min: 0,
            max: 0,
            beg: 0,
            end: 0,
            num: 0,
            sum: 0,
            avg: 0,
            tmp: 0,
        };

        unsafe {
            vxQueryGraph(
                self.raw,
                vx_graph_attribute_e_VX_GRAPH_PERFORMANCE as vx_enum,
                &mut perf as *mut _ as *mut std::ffi::c_void,
                std::mem::size_of_val(&perf) as vx_size,
            );
        }

        unimplemented!()
    }
}

impl Release for VxGraph {
    fn release(&mut self) -> Result<()> {
        if self.raw.is_null() {
            return Ok(());
        }

        let status = unsafe { vxReleaseGraph(&mut self.raw) };
        self.raw = std::ptr::null_mut();

        VxStatus::new_result(status, ())
    }
}

impl Drop for VxGraph {
    fn drop(&mut self) {
        self.release().unwrap();
    }
}

impl AsVxReference for VxGraph {
    fn as_reference(&self) -> VxReference {
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
