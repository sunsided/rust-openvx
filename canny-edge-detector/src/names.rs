use libopenvx_sys::*;
use std::borrow::Borrow;

pub fn set_graph_name<S>(graph: vx_graph, name: S) -> vx_graph
where
    S: Borrow<str>,
{
    set_reference_name(graph as vx_reference, name);
    graph
}

pub fn set_node_name<S>(node: vx_node, name: S) -> vx_node
where
    S: Borrow<str>,
{
    set_reference_name(node as vx_reference, name);
    node
}

pub fn set_reference_name<S>(reference: vx_reference, name: S) -> vx_reference
where
    S: Borrow<str>,
{
    unsafe {
        vxSetReferenceName(
            reference,
            std::ffi::CString::new(name.borrow())
                .expect("CString::new failed")
                .as_ptr(),
        );
    }
    reference
}
