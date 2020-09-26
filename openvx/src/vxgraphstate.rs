use core::fmt;
use libopenvx_sys::*;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum VxGraphState {
    Unverified,
    Verified,
    Running,
    Abandoned,
    Completed,
    Unknown,
}

impl VxGraphState {
    const fn new(status: vx_graph_state_e) -> Self {
        #[allow(non_upper_case_globals)]
        match status {
            vx_graph_state_e_VX_GRAPH_STATE_UNVERIFIED => VxGraphState::Unverified,
            vx_graph_state_e_VX_GRAPH_STATE_VERIFIED => VxGraphState::Verified,
            vx_graph_state_e_VX_GRAPH_STATE_RUNNING => VxGraphState::Running,
            vx_graph_state_e_VX_GRAPH_STATE_ABANDONED => VxGraphState::Abandoned,
            vx_graph_state_e_VX_GRAPH_STATE_COMPLETED => VxGraphState::Completed,
            _ => VxGraphState::Unknown,
        }
    }
}

impl From<vx_graph_state_e> for VxGraphState {
    fn from(status: vx_graph_state_e) -> Self {
        VxGraphState::new(status)
    }
}

impl fmt::Display for VxGraphState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            VxGraphState::Unverified => write!(f, "VX_GRAPH_STATE_UNVERIFIED"),
            VxGraphState::Verified => write!(f, "VX_GRAPH_STATE_VERIFIED"),
            VxGraphState::Running => write!(f, "VX_GRAPH_STATE_RUNNING"),
            VxGraphState::Abandoned => write!(f, "VX_GRAPH_STATE_ABANDONED"),
            VxGraphState::Completed => write!(f, "VX_GRAPH_STATE_COMPLETED"),
            VxGraphState::Unknown => write!(f, "VX_GRAPH_STATE_UNKNOWN"),
        }
    }
}
