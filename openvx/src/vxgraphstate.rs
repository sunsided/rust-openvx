use core::fmt;
use libopenvx_sys::*;

/// The Graph State Enumeration.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum VxGraphState {
    /// The graph should be verified before execution.
    Unverified,
    /// The graph has been verified and has not been executed or scheduled for execution yet.
    Verified,
    /// The graph either has been scheduled and not completed, or is being executed.
    Running,
    /// The graph execution was abandoned.
    Abandoned,
    /// The graph execution is completed and the graph is not scheduled for execution.
    Completed,
    /// An unknown graph state.
    Unknown { state: vx_graph_state_e },
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
            state => VxGraphState::Unknown { state },
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
            VxGraphState::Unknown { state } => write!(f, "VX_GRAPH_STATE_UNKNOWN ({})", state),
        }
    }
}
