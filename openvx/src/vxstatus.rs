use core::fmt;
use libopenvx_sys::*;

pub enum VxStatus {
    /// No error.
    Success,
    /// Indicates a generic error code, used when no other describes the error.
    Failure,
    /// Indicates that the requested kernel is missing.
    Error(vx_status_e),
}

impl VxStatus {
    fn new(status: vx_status_e) -> Self {
        #[allow(non_upper_case_globals)]
        match status {
            vx_status_e_VX_SUCCESS => VxStatus::Success,
            vx_status_e_VX_FAILURE => VxStatus::Failure,
            n => VxStatus::Error(n),
        }
    }

    pub fn as_isize(&self) -> isize {
        match self {
            VxStatus::Success => vx_status_e_VX_SUCCESS as isize,
            VxStatus::Failure => vx_status_e_VX_FAILURE as isize,
            VxStatus::Error(n) => *n as isize,
        }
    }
}

impl From<vx_status_e> for VxStatus {
    fn from(status: vx_status_e) -> Self {
        VxStatus::new(status)
    }
}

impl Eq for VxStatus {}

impl PartialEq for VxStatus {
    fn eq(&self, other: &Self) -> bool {
        let left = self.as_isize();
        let right = other.as_isize();
        left == right
    }
}

impl fmt::Debug for VxStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            VxStatus::Success => write!(f, "VX_SUCCESS"),
            VxStatus::Failure => write!(f, "VX_FAILURE"),
            #[allow(non_upper_case_globals)]
            VxStatus::Error(n) => match n {
                vx_status_e_VX_ERROR_NOT_IMPLEMENTED => write!(f, "VX_ERROR_NOT_IMPLEMENTED"), // -2
                vx_status_e_VX_ERROR_NOT_SUPPORTED => write!(f, "VX_ERROR_NOT_SUPPORTED"),     // -3
                vx_status_e_VX_ERROR_NOT_SUFFICIENT => write!(f, "VX_ERROR_NOT_SUFFICIENT"),   // -4
                vx_status_e_VX_ERROR_NOT_ALLOCATED => write!(f, "VX_ERROR_NOT_ALLOCATED"),     // -5
                vx_status_e_VX_ERROR_NOT_COMPATIBLE => write!(f, "VX_ERROR_NOT_COMPATIBLE"),   // -6
                vx_status_e_VX_ERROR_NO_RESOURCES => write!(f, "VX_ERROR_NO_RESOURCES"),       // -7
                vx_status_e_VX_ERROR_NO_MEMORY => write!(f, "VX_ERROR_NO_MEMORY"),             // -8
                vx_status_e_VX_ERROR_OPTIMIZED_AWAY => write!(f, "VX_ERROR_OPTIMIZED_AWAY"),   // -9
                vx_status_e_VX_ERROR_INVALID_PARAMETERS => write!(f, "VX_ERROR_INVALID_PARAMETERS"), // -10
                vx_status_e_VX_ERROR_INVALID_MODULE => write!(f, "VX_ERROR_INVALID_MODULE"), // -11
                vx_status_e_VX_ERROR_INVALID_REFERENCE => write!(f, "VX_ERROR_INVALID_REFERENCE"), // -12
                vx_status_e_VX_ERROR_INVALID_LINK => write!(f, "VX_ERROR_INVALID_LINK"), // -13
                vx_status_e_VX_ERROR_INVALID_FORMAT => write!(f, "VX_ERROR_INVALID_FORMAT"), // -14
                vx_status_e_VX_ERROR_INVALID_DIMENSION => write!(f, "VX_ERROR_INVALID_DIMENSION"), // -15
                vx_status_e_VX_ERROR_INVALID_VALUE => write!(f, "VX_ERROR_INVALID_VALUE"), // -16
                vx_status_e_VX_ERROR_INVALID_TYPE => write!(f, "VX_ERROR_INVALID_TYPE"),   // -17
                vx_status_e_VX_ERROR_INVALID_GRAPH => write!(f, "VX_ERROR_INVALID_GRAPH"), // -18
                vx_status_e_VX_ERROR_INVALID_NODE => write!(f, "VX_ERROR_INVALID_NODE"),   // -19
                vx_status_e_VX_ERROR_INVALID_SCOPE => write!(f, "VX_ERROR_INVALID_SCOPE"), // -20
                vx_status_e_VX_ERROR_GRAPH_SCHEDULED => write!(f, "VX_ERROR_GRAPH_SCHEDULED"), // -21
                vx_status_e_VX_ERROR_GRAPH_ABANDONED => write!(f, "VX_ERROR_GRAPH_ABANDONED"), // -22
                vx_status_e_VX_ERROR_MULTIPLE_WRITERS => write!(f, "VX_ERROR_MULTIPLE_WRITERS"), // -23
                vx_status_e_VX_ERROR_REFERENCE_NONZERO => write!(f, "VX_ERROR_REFERENCE_NONZERO"), // -24
                _ => write!(f, "unknown error ({})", n),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    #![macro_use]
    use super::*;
    use libopenvx_sys::*;

    #[test]
    fn vx_status_e_success() {
        assert_eq!(VxStatus::from(vx_status_e_VX_SUCCESS), VxStatus::Success);
    }

    #[test]
    fn vx_status_e_failure() {
        assert_eq!(VxStatus::from(vx_status_e_VX_FAILURE), VxStatus::Failure);
    }

    #[test]
    fn vx_status_e_not_implemented() {
        assert_eq!(
            VxStatus::from(vx_status_e_VX_ERROR_NOT_IMPLEMENTED),
            VxStatus::Error(vx_status_e_VX_ERROR_NOT_IMPLEMENTED)
        );
    }
}
