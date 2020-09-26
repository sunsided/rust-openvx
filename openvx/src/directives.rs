use crate::{AsVxReference, Result, VxStatus};
use libopenvx_sys::{vxDirective, vx_directive_e, vx_enum, vx_enum_e_VX_ENUM_DIRECTIVE};

/// Constants for the `vxDirective` API.
pub mod constants {
    use libopenvx_sys::{
        vx_directive_e, vx_enum_e_VX_ENUM_DIRECTIVE, vx_vendor_id_e_VX_ID_KHRONOS,
    };

    /// Base value for directive values.
    pub const VX_ENUM_DIRECTIVE_BASE: vx_directive_e = vx_enum_base!(
        vx_directive_e,
        vx_vendor_id_e_VX_ID_KHRONOS,
        vx_enum_e_VX_ENUM_DIRECTIVE
    );

    /// Disables recording information for graph debugging. See also [`VxDirective::DisableLogging`](../../enum.VxDirective.html#variant.DisableLogging).
    pub const VX_DIRECTIVE_DISABLE_LOGGING: vx_directive_e = VX_ENUM_DIRECTIVE_BASE + 0x0;

    /// Enables recording information for graph debugging. See also [`VxDirective::EnableLogging`](../../enum.VxDirective.html#variant.EnableLogging).
    pub const VX_DIRECTIVE_ENABLE_LOGGING: vx_directive_e = VX_ENUM_DIRECTIVE_BASE + 0x1;

    /// Disables performance counters for the context. By default performance counters are disabled. See also [`VxDirective::DisablePerformance`](../../enum.VxDirective.html#variant.DisablePerformance).
    pub const VX_DIRECTIVE_DISABLE_PERFORMANCE: vx_directive_e = VX_ENUM_DIRECTIVE_BASE + 0x2;

    /// Enables performance counters for the context. See also [`VxDirective::EnablePerformance`](../../enum.VxDirective.html#variant.EnablePerformance).
    pub const VX_DIRECTIVE_ENABLE_PERFORMANCE: vx_directive_e = VX_ENUM_DIRECTIVE_BASE + 0x3;
}

/// These enumerations are given to the `vxDirective` API to enable/disable platform optimizations
/// and/or features. Directives are not optional and usually are vendor-specific, by defining a
/// vendor range of directives and starting their enumeration from there.
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum VxDirective {
    /// Disables recording information for graph debugging.
    DisableLogging,
    /// Enables recording information for graph debugging.
    EnableLogging,
    /// Disables performance counters for the context. By default performance counters are disabled.
    DisablePerformance,
    /// Enables performance counters for the context.
    EnablePerformance,
    /// An unknown directive.
    Other { directive: vx_directive_e },
}

impl VxDirective {
    pub fn new(directive: vx_directive_e) -> Self {
        assert_eq!(vx_enum_type!(directive), vx_enum_e_VX_ENUM_DIRECTIVE);
        match directive {
            constants::VX_DIRECTIVE_DISABLE_LOGGING => VxDirective::DisableLogging,
            constants::VX_DIRECTIVE_ENABLE_LOGGING => VxDirective::EnableLogging,
            constants::VX_DIRECTIVE_DISABLE_PERFORMANCE => VxDirective::DisablePerformance,
            constants::VX_DIRECTIVE_ENABLE_PERFORMANCE => VxDirective::EnablePerformance,
            directive => VxDirective::Other { directive },
        }
    }

    /// Converts this instance into a [`vx_directive_e`].
    ///
    /// [`vx_directive_e`]: ../libopenvx_sys/type.vx_directive_e.html
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use openvx::{VxDirective, constants::directives};
    ///
    /// let x: VxDirective = VxDirective::EnableLogging;
    /// assert_eq!(x.to_raw(), directives::VX_DIRECTIVE_ENABLE_LOGGING);
    /// ```
    pub const fn to_raw(&self) -> vx_directive_e {
        match self {
            VxDirective::DisableLogging => constants::VX_DIRECTIVE_DISABLE_LOGGING,
            VxDirective::EnableLogging => constants::VX_DIRECTIVE_ENABLE_LOGGING,
            VxDirective::DisablePerformance => constants::VX_DIRECTIVE_DISABLE_PERFORMANCE,
            VxDirective::EnablePerformance => constants::VX_DIRECTIVE_ENABLE_PERFORMANCE,
            VxDirective::Other { directive } => *directive,
        }
    }
}

/// Provides a generic API to give platform-specific directives to the implementations.
pub trait SetDirective {
    /// Sets a directive on the current reference.
    ///
    /// # Arguments
    ///
    /// * `directive` - The directive to set.
    ///
    /// # Errors
    ///
    /// This function may return the following error codes:
    ///
    /// * [`VX_ERROR_INVALID_REFERENCE`] - the current reference is not valid
    /// * [`VX_ERROR_NOT_SUPPORTED`] - the directive is not supported (on the current reference)
    ///
    /// # Notes
    ///
    /// The performance counter directives are only available for the reference [`VxContext`].
    /// Error [`VX_ERROR_NOT_SUPPORTED`] is returned when used with any other reference.
    ///
    /// [`VxContext`]: types/struct.VxContext.html
    /// [`VX_ERROR_INVALID_REFERENCE`]: enum.VxError.html#variant.InvalidReference
    /// [`VX_ERROR_NOT_SUPPORTED`]: enum.VxError.html#variant.NotSupported
    fn set_directive(&self, directive: VxDirective) -> Result<&Self>;
}

impl<P> SetDirective for P
where
    P: AsVxReference,
{
    fn set_directive(&self, directive: VxDirective) -> Result<&Self> {
        let result =
            unsafe { vxDirective(self.as_reference().into(), directive.to_raw() as vx_enum) };
        VxStatus::new_result(result, self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use libopenvx_sys::*;

    #[test]
    fn constants_are_enum_directive() {
        assert_eq!(
            vx_enum_type!(constants::VX_DIRECTIVE_DISABLE_LOGGING),
            vx_enum_e_VX_ENUM_DIRECTIVE
        );
    }

    #[test]
    fn directives_new() {
        assert_eq!(
            VxDirective::new(vx_directive_e_VX_DIRECTIVE_DISABLE_LOGGING),
            VxDirective::DisableLogging
        );
        assert_eq!(
            VxDirective::new(vx_directive_e_VX_DIRECTIVE_ENABLE_LOGGING),
            VxDirective::EnableLogging
        );
        assert_eq!(
            VxDirective::new(vx_directive_e_VX_DIRECTIVE_DISABLE_PERFORMANCE),
            VxDirective::DisablePerformance
        );
        assert_eq!(
            VxDirective::new(vx_directive_e_VX_DIRECTIVE_ENABLE_PERFORMANCE),
            VxDirective::EnablePerformance
        );
        assert_eq!(
            VxDirective::new(vx_directive_e_VX_DIRECTIVE_ENABLE_PERFORMANCE + 1),
            VxDirective::Other {
                directive: vx_directive_e_VX_DIRECTIVE_ENABLE_PERFORMANCE + 1
            }
        );
    }

    #[test]
    fn directives_toraw() {
        assert_eq!(
            VxDirective::DisableLogging.to_raw(),
            vx_directive_e_VX_DIRECTIVE_DISABLE_LOGGING
        );
        assert_eq!(
            VxDirective::EnableLogging.to_raw(),
            vx_directive_e_VX_DIRECTIVE_ENABLE_LOGGING
        );
        assert_eq!(
            VxDirective::DisablePerformance.to_raw(),
            vx_directive_e_VX_DIRECTIVE_DISABLE_PERFORMANCE
        );
        assert_eq!(
            VxDirective::EnablePerformance.to_raw(),
            vx_directive_e_VX_DIRECTIVE_ENABLE_PERFORMANCE
        );
        assert_eq!(
            VxDirective::Other {
                directive: vx_directive_e_VX_DIRECTIVE_ENABLE_PERFORMANCE + 1
            }
            .to_raw(),
            vx_directive_e_VX_DIRECTIVE_ENABLE_PERFORMANCE + 1
        );
    }
}
