use libopenvx_sys::*;
use openvx::name::set_name;
use std::borrow::Borrow;

pub fn set_node_name<S>(node: vx_node, name: S) -> vx_node
where
    S: Borrow<str>,
{
    set_name(node as vx_reference, name);
    node
}
