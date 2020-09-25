use crate::vxerror::VxError;
use core::fmt;
use libopenvx_sys::*;
use std::cmp::Ordering;

/// [`Result<T>`][`Result`] is the type used for returning and propagating
/// errors. It is an enum with the variants, [`Ok(T)`], representing
/// success and containing a value, and [`Err(VxError)`], representing error
/// and containing an error value.
///
/// ```
/// # #[allow(dead_code)]
/// enum Result<T> {
///    Ok(T),
///    Err(openvx::VxError),
/// }
/// ```
///
/// [`Result`]: type.Result.html
/// [`Ok(T)`]: type.Result.html#variant.Ok
/// [`Err(VxError)`]: type.Result.html#variant.Err
pub type Result<T> = std::result::Result<T, VxError>;

#[derive(Debug, Eq, Copy, Clone, Ord, Hash)]
pub enum VxStatus {
    /// Encodes the success value.
    Success,
    /// Contains the error value.
    Error(VxError),
}

impl VxStatus {
    /// Constructs a
    const fn new(status: vx_status_e) -> Self {
        #[allow(non_upper_case_globals)]
        match status {
            vx_status_e_VX_SUCCESS => VxStatus::Success,
            n => VxStatus::Error(VxError::new_unchecked(n)),
        }
    }

    /// Converts this instance into a [`vx_status_e`].
    ///
    /// [`vx_status_e`]: ../libopenvx_sys/type.c.html
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use openvx::{VxStatus, VxError};
    ///
    /// let x: vxstatus = VxStatus::Success;
    /// assert_eq!(x.to_raw(), libopenvx_sys::vx_status_e_VX_SUCCESS);
    ///
    /// let x: vxstatus = VxStatus::Error(VxError::Failure);
    /// assert_eq!(x.to_raw(), libopenvx_sys::vx_status_e_VX_FAILURE);
    /// ```
    pub const fn to_raw(&self) -> vx_status_e {
        match self {
            VxStatus::Success => vx_status_e_VX_SUCCESS,
            VxStatus::Error(err) => err.to_raw(),
        }
    }

    /// Returns `true` if the status is [`Success`].
    ///
    /// [`Success`]: enum.VxStatus.html#variant.Success
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use openvx::{VxStatus, VxError};
    ///
    /// let x: VxStatus = VxStatus::Success;
    /// assert_eq!(x.is_ok(), true);
    ///
    /// let x: VxStatus = VxStatus::Error(VxError::Failure);
    /// assert_eq!(x.is_ok(), false);
    /// ```
    pub fn is_ok(&self) -> bool {
        *self == VxStatus::Success
    }

    /// Returns `true` if the status is [`Error`].
    ///
    /// [`Error`]: enum.VxStatus.html#variant.Error
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use openvx::{VxStatus, VxError};
    ///
    /// let x: VxStatus = VxStatus::Success;
    /// assert_eq!(x.is_err(), false);
    ///
    /// let x: VxStatus = VxStatus::Error(VxError::Failure);
    /// assert_eq!(x.is_err(), true);
    /// ```
    #[must_use]
    #[inline]
    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }
}

impl From<vx_status_e> for VxStatus {
    fn from(status: vx_status_e) -> Self {
        VxStatus::new(status)
    }
}

impl PartialEq for VxStatus {
    fn eq(&self, other: &Self) -> bool {
        let left = self.to_raw();
        let right = other.to_raw();
        left == right
    }
}

impl PartialOrd for VxStatus {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let left = self.to_raw();
        let right = other.to_raw();
        left.partial_cmp(&right)
    }
}

impl fmt::Display for VxStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            VxStatus::Success => write!(f, "VX_SUCCESS"),
            VxStatus::Error(err) => err.fmt(f),
        }
    }
}

impl<T> Into<VxStatus> for Result<T> {
    fn into(self) -> VxStatus {
        if let Err(error) = self {
            return VxStatus::Error(error);
        }
        VxStatus::Success
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
    fn vx_status_e_success() {
        assert_eq!(VxStatus::from(vx_status_e_VX_SUCCESS), VxStatus::Success);
    }
}
