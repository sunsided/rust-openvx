use core::fmt;
use libopenvx_sys::*;
extern crate static_assertions as sa;
use self::sa::_core::fmt::Formatter;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt::Display;

/// The enumeration of all error status codes.
#[derive(Debug, Eq, Copy, Clone, Ord, Hash)]
pub enum VxError {
    /// Indicates a generic error code, used when no other describes the error.
    Failure,
    /// Indicates that the requested kernel is missing. TODO: \\see vx_kernel_e vxGetKernelByName.
    NotImplemented,
    /// Indicates that the requested set of parameters produce a configuration that cannot be supported.
    /// Refer to the supplied documentation on the configured kernels.
    /// TODO: \\see vx_kernel_e.
    /// This is also returned if a function to set an attribute is called on a Read-only attribute.
    NotSupported,
    /// Indicates that the given graph has failed verification due to an insufficient number of
    /// required parameters, which cannot be automatically created.
    /// Typically this indicates required atomic parameters.
    /// TODO: \\see vxVerifyGraph.
    NotSufficient,
    /// Indicates to the system that the parameter must be allocated by the system.
    NotAllocated,
    /// Indicates that the attempt to link two parameters together failed due to type incompatibilty.
    NotCompatible,
    /// Indicates that an internal or implicit resource can not be acquired (not memory).
    /// This is typically catastrophic. After detection, deconstruct the context.
    /// TODO: \\see vxVerifyGraph.
    NoResources,
    /// Indicates that an internal or implicit allocation failed. Typically catastrophic.
    /// After detection, deconstruct the context.
    /// TODO: \\see vxVerifyGraph.
    NoMemory,
    /// Indicates that the object refered to has been optimized out of existence.
    OptimizedAway,
    /// Indicates that the supplied parameter information does not match the kernel contract.
    InvalidParameters,
    /// This is returned from
    /// TODO: <tt>\\ref vxLoadKernels</tt>
    /// when the module does not contain the entry point.
    InvalidModule,
    /// Indicates that the reference provided is not valid.
    InvalidReference,
    /// Indicates that the link is not possible as specified. The parameters are incompatible.
    InvalidLink,
    /// Indicates that the supplied parameter is in an invalid format.
    InvalidFormat,
    /// Indicates that the supplied parameter is too big or too small in dimension.
    InvalidDimension,
    /// Indicates that the supplied parameter has an incorrect value.
    InvalidValue,
    /// Indicates that the supplied type parameter is incorrect.
    InvalidType,
    /// Indicates that the supplied graph has invalid connections (cycles).
    InvalidGraph,
    /// Indicates that the supplied node could not be created.
    InvalidNode,
    /// Indicates that the supplied parameter is from another scope and cannot be used in the current scope.
    InvalidScope,
    /// Indicates that the supplied graph already has been scheduled and may be currently executing.
    GraphScheduled,
    /// Indicates that the graph is stopped due to an error or a callback that abandoned execution.
    GraphAbandoned,
    /// Indicates that the graph has more than one node outputting to the same data object.
    /// This is an invalid graph structure.
    MultipleWriters,
    /// Indicates that an operation did not complete due to a reference count being non-zero.
    ReferenceNonzero,
    /// Indicates that an unknown status code was encountered.
    Unknown { code: vx_status_e },
}

impl VxError {
    pub fn new(status: vx_status_e) -> Self {
        assert!(status > vx_status_e_VX_STATUS_MIN);
        assert!(status < 0);
        Self::new_unchecked(status)
    }

    pub const fn new_unchecked(status: vx_status_e) -> Self {
        #[allow(non_upper_case_globals)]
        match status {
            vx_status_e_VX_ERROR_NOT_IMPLEMENTED => VxError::NotImplemented,
            vx_status_e_VX_ERROR_NOT_SUPPORTED => VxError::NotSupported,
            vx_status_e_VX_ERROR_NOT_SUFFICIENT => VxError::NotSufficient,
            vx_status_e_VX_ERROR_NOT_ALLOCATED => VxError::NotAllocated,
            vx_status_e_VX_ERROR_NOT_COMPATIBLE => VxError::NotCompatible,
            vx_status_e_VX_ERROR_NO_RESOURCES => VxError::NoResources,
            vx_status_e_VX_ERROR_NO_MEMORY => VxError::NoMemory,
            vx_status_e_VX_ERROR_OPTIMIZED_AWAY => VxError::OptimizedAway,
            vx_status_e_VX_ERROR_INVALID_PARAMETERS => VxError::InvalidParameters,
            vx_status_e_VX_ERROR_INVALID_MODULE => VxError::InvalidModule,
            vx_status_e_VX_ERROR_INVALID_REFERENCE => VxError::InvalidReference,
            vx_status_e_VX_ERROR_INVALID_LINK => VxError::InvalidLink,
            vx_status_e_VX_ERROR_INVALID_FORMAT => VxError::InvalidFormat,
            vx_status_e_VX_ERROR_INVALID_DIMENSION => VxError::InvalidDimension,
            vx_status_e_VX_ERROR_INVALID_VALUE => VxError::InvalidValue,
            vx_status_e_VX_ERROR_INVALID_TYPE => VxError::InvalidType,
            vx_status_e_VX_ERROR_INVALID_GRAPH => VxError::InvalidGraph,
            vx_status_e_VX_ERROR_INVALID_NODE => VxError::InvalidNode,
            vx_status_e_VX_ERROR_INVALID_SCOPE => VxError::InvalidScope,
            vx_status_e_VX_ERROR_GRAPH_SCHEDULED => VxError::GraphScheduled,
            vx_status_e_VX_ERROR_GRAPH_ABANDONED => VxError::GraphAbandoned,
            vx_status_e_VX_ERROR_MULTIPLE_WRITERS => VxError::MultipleWriters,
            vx_status_e_VX_ERROR_REFERENCE_NONZERO => VxError::ReferenceNonzero,
            vx_status_e_VX_FAILURE => VxError::Failure,
            n => VxError::Unknown { code: n },
        }
    }

    /// Converts this instance into a [`vx_status_e`].
    ///
    /// [`vx_status_e`]: ../libopenvx_sys/type.vx_status_e.html
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use openvx::{VxStatus, VxError};
    ///
    /// let x: VxStatus = VxStatus::Error(VxError::Failure);
    /// assert_eq!(x.to_raw(), libopenvx_sys::vx_status_e_VX_FAILURE);
    /// ```
    pub const fn to_raw(&self) -> vx_status_e {
        match self {
            VxError::NotImplemented => vx_status_e_VX_ERROR_NOT_IMPLEMENTED,
            VxError::NotSupported => vx_status_e_VX_ERROR_NOT_SUPPORTED,
            VxError::NotSufficient => vx_status_e_VX_ERROR_NOT_SUFFICIENT,
            VxError::NotAllocated => vx_status_e_VX_ERROR_NOT_ALLOCATED,
            VxError::NotCompatible => vx_status_e_VX_ERROR_NOT_COMPATIBLE,
            VxError::NoResources => vx_status_e_VX_ERROR_NO_RESOURCES,
            VxError::NoMemory => vx_status_e_VX_ERROR_NO_MEMORY,
            VxError::OptimizedAway => vx_status_e_VX_ERROR_OPTIMIZED_AWAY,
            VxError::InvalidParameters => vx_status_e_VX_ERROR_INVALID_PARAMETERS,
            VxError::InvalidModule => vx_status_e_VX_ERROR_INVALID_MODULE,
            VxError::InvalidReference => vx_status_e_VX_ERROR_INVALID_REFERENCE,
            VxError::InvalidLink => vx_status_e_VX_ERROR_INVALID_LINK,
            VxError::InvalidFormat => vx_status_e_VX_ERROR_INVALID_FORMAT,
            VxError::InvalidDimension => vx_status_e_VX_ERROR_INVALID_DIMENSION,
            VxError::InvalidValue => vx_status_e_VX_ERROR_INVALID_VALUE,
            VxError::InvalidType => vx_status_e_VX_ERROR_INVALID_TYPE,
            VxError::InvalidGraph => vx_status_e_VX_ERROR_INVALID_GRAPH,
            VxError::InvalidNode => vx_status_e_VX_ERROR_INVALID_NODE,
            VxError::InvalidScope => vx_status_e_VX_ERROR_INVALID_SCOPE,
            VxError::GraphScheduled => vx_status_e_VX_ERROR_GRAPH_SCHEDULED,
            VxError::GraphAbandoned => vx_status_e_VX_ERROR_GRAPH_ABANDONED,
            VxError::MultipleWriters => vx_status_e_VX_ERROR_MULTIPLE_WRITERS,
            VxError::ReferenceNonzero => vx_status_e_VX_ERROR_REFERENCE_NONZERO,
            VxError::Failure => vx_status_e_VX_FAILURE,
            VxError::Unknown { code } => *code,
        }
    }
}

/// Indicates that the specified status code did not encode an error.
#[derive(Debug)]
pub struct InvalidErrorStatusError {
    value: isize,
}

impl Display for InvalidErrorStatusError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        sa::const_assert!(vx_status_e_VX_SUCCESS == 0);
        sa::const_assert!(vx_status_e_VX_FAILURE == -1);
        sa::const_assert!(vx_status_e_VX_STATUS_MIN < vx_status_e_VX_FAILURE);

        write!(
            f,
            "Status error value is expected to be in range {} < value < {}, but was {}",
            vx_status_e_VX_STATUS_MIN, vx_status_e_VX_FAILURE, self.value
        )
    }
}

impl TryFrom<vx_status_e> for VxError {
    type Error = InvalidErrorStatusError;

    fn try_from(value: vx_status_e) -> Result<Self, Self::Error> {
        // vx_status_e_VX_STATUS_MIN will be mapped to VxError::Unknown
        if value >= 0 {
            return Err(InvalidErrorStatusError {
                value: value as isize,
            });
        }
        Ok(Self::new_unchecked(value))
    }
}

impl TryFrom<isize> for VxError {
    type Error = InvalidErrorStatusError;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        // vx_status_e_VX_STATUS_MIN will be mapped to VxError::Unknown
        if value >= 0 {
            return Err(InvalidErrorStatusError { value });
        }
        Ok(Self::new_unchecked(value as vx_status_e))
    }
}

impl Into<vx_status_e> for VxError {
    #[inline(always)]
    fn into(self) -> vx_status_e {
        self.to_raw()
    }
}

impl Into<isize> for VxError {
    #[inline(always)]
    fn into(self) -> isize {
        let value: vx_status_e = self.into();
        value as isize
    }
}

impl PartialEq for VxError {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        let left: vx_status_e = (*self).into();
        let right: vx_status_e = (*other).into();
        left == right
    }
}

impl PartialOrd for VxError {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let left: vx_status_e = (*self).into();
        let right: vx_status_e = (*other).into();
        left.partial_cmp(&right)
    }
}

impl fmt::Display for VxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[allow(non_upper_case_globals)]
        match *self {
            VxError::Failure => write!(f, "VX_FAILURE"), // -1
            VxError::NotImplemented => write!(f, "VX_ERROR_NOT_IMPLEMENTED"), // -2
            VxError::NotSupported => write!(f, "VX_ERROR_NOT_SUPPORTED"), // -3
            VxError::NotSufficient => write!(f, "VX_ERROR_NOT_SUFFICIENT"), // -4
            VxError::NotAllocated => write!(f, "VX_ERROR_NOT_ALLOCATED"), // -5
            VxError::NotCompatible => write!(f, "VX_ERROR_NOT_COMPATIBLE"), // -6
            VxError::NoResources => write!(f, "VX_ERROR_NO_RESOURCES"), // -7
            VxError::NoMemory => write!(f, "VX_ERROR_NO_MEMORY"), // -8
            VxError::OptimizedAway => write!(f, "VX_ERROR_OPTIMIZED_AWAY"), // -9
            VxError::InvalidParameters => write!(f, "VX_ERROR_INVALID_PARAMETERS"), // -10
            VxError::InvalidModule => write!(f, "VX_ERROR_INVALID_MODULE"), // -11
            VxError::InvalidReference => write!(f, "VX_ERROR_INVALID_REFERENCE"), // -12
            VxError::InvalidLink => write!(f, "VX_ERROR_INVALID_LINK"), // -13
            VxError::InvalidFormat => write!(f, "VX_ERROR_INVALID_FORMAT"), // -14
            VxError::InvalidDimension => write!(f, "VX_ERROR_INVALID_DIMENSION"), // -15
            VxError::InvalidValue => write!(f, "VX_ERROR_INVALID_VALUE"), // -16
            VxError::InvalidType => write!(f, "VX_ERROR_INVALID_TYPE"), // -17
            VxError::InvalidGraph => write!(f, "VX_ERROR_INVALID_GRAPH"), // -18
            VxError::InvalidNode => write!(f, "VX_ERROR_INVALID_NODE"), // -19
            VxError::InvalidScope => write!(f, "VX_ERROR_INVALID_SCOPE"), // -20
            VxError::GraphScheduled => write!(f, "VX_ERROR_GRAPH_SCHEDULED"), // -21
            VxError::GraphAbandoned => write!(f, "VX_ERROR_GRAPH_ABANDONED"), // -22
            VxError::MultipleWriters => write!(f, "VX_ERROR_MULTIPLE_WRITERS"), // -23
            VxError::ReferenceNonzero => write!(f, "VX_ERROR_REFERENCE_NONZERO"), // -24
            n => write!(f, "unknown error ({})", n),
        }
    }
}

#[cfg(test)]
mod tests {
    #![macro_use]
    use super::*;

    #[test]
    fn vx_status_min() {
        assert_eq!(vx_status_e_VX_STATUS_MIN, -25);
    }

    #[test]
    fn vx_status_e_failure() {
        assert_eq!(
            VxError::try_from(vx_status_e_VX_FAILURE).unwrap(),
            VxError::Failure
        );
    }

    #[test]
    fn vx_status_e_not_implemented() {
        assert_eq!(
            VxError::try_from(vx_status_e_VX_ERROR_NOT_IMPLEMENTED).unwrap(),
            VxError::NotImplemented
        );
    }

    #[test]
    fn invalid_error() {
        assert!(VxError::try_from(vx_status_e_VX_SUCCESS).is_err());
    }

    #[test]
    fn unknown_status() {
        assert_eq!(
            VxError::try_from(vx_status_e_VX_STATUS_MIN).unwrap(),
            VxError::Unknown {
                code: vx_status_e_VX_STATUS_MIN
            }
        );
    }
}
