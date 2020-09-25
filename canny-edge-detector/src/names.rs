use libopenvx_sys::*;
use openvx::setname::set_name;
use std::borrow::Borrow;

pub fn set_graph_name<S>(graph: vx_graph, name: S) -> vx_graph
where
    S: Borrow<str>,
{
    set_name(graph as vx_reference, name);
    graph
}

pub fn set_node_name<S>(node: vx_node, name: S) -> vx_node
where
    S: Borrow<str>,
{
    set_name(node as vx_reference, name);
    node
}
