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
            VxStatus::Error(n) => match n {
                vx_status_e_VX_ERROR_NOT_IMPLEMENTED => write!(f, "VX_ERROR_NOT_IMPLEMENTED"),
                vx_status_e_VX_ERROR_NOT_SUPPORTED => write!(f, "VX_ERROR_NOT_SUPPORTED"),
                vx_status_e_VX_ERROR_NOT_SUFFICIENT => write!(f, "VX_ERROR_NOT_SUFFICIENT"),
                vx_status_e_VX_ERROR_NOT_ALLOCATED => write!(f, "VX_ERROR_NOT_ALLOCATED"),
                vx_status_e_VX_ERROR_NOT_COMPATIBLE => write!(f, "VX_ERROR_NOT_COMPATIBLE"),
                vx_status_e_VX_ERROR_NO_RESOURCES => write!(f, "VX_ERROR_NO_RESOURCES"),
                vx_status_e_VX_ERROR_NO_MEMORY => write!(f, "VX_ERROR_NO_MEMORY"),
                vx_status_e_VX_ERROR_OPTIMIZED_AWAY => write!(f, "VX_ERROR_OPTIMIZED_AWAY"),
                vx_status_e_VX_ERROR_INVALID_PARAMETERS => write!(f, "VX_ERROR_INVALID_PARAMETERS"),
                vx_status_e_VX_ERROR_INVALID_MODULE => write!(f, "VX_ERROR_INVALID_MODULE"),
                vx_status_e_VX_ERROR_INVALID_REFERENCE => write!(f, "VX_ERROR_INVALID_REFERENCE"),
                vx_status_e_VX_ERROR_INVALID_LINK => write!(f, "VX_ERROR_INVALID_LINK"),
                vx_status_e_VX_ERROR_INVALID_DIMENSION => write!(f, "VX_ERROR_INVALID_DIMENSION"),
                vx_status_e_VX_ERROR_INVALID_VALUE => write!(f, "VX_ERROR_INVALID_VALUE"),
                vx_status_e_VX_ERROR_INVALID_TYPE => write!(f, "VX_ERROR_INVALID_TYPE"),
                vx_status_e_VX_ERROR_INVALID_GRAPH => write!(f, "VX_ERROR_INVALID_GRAPH"),
                vx_status_e_VX_ERROR_INVALID_NODE => write!(f, "VX_ERROR_INVALID_NODE"),
                vx_status_e_VX_ERROR_INVALID_SCOPE => write!(f, "VX_ERROR_INVALID_SCOPE"),
                vx_status_e_VX_ERROR_GRAPH_SCHEDULED => write!(f, "VX_ERROR_GRAPH_SCHEDULED"),
                vx_status_e_VX_ERROR_GRAPH_ABANDONED => write!(f, "VX_ERROR_GRAPH_ABANDONED"),
                vx_status_e_VX_ERROR_REFERENCE_NONZERO => write!(f, "VX_ERROR_REFERENCE_NONZERO"),
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
