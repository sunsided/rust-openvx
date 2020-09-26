/// A macro to extract the type from an enumerated attribute value.
#[macro_export]
// TODO: Needs testing
macro_rules! vx_type {
    ($e:expr) => {
        (($e as libopenvx_sys::vx_uint32) & libopenvx_sys::VX_TYPE_MASK) >> 8
    };
}

/// A macro to extract the enum type from an enumerated value.
///
/// # Examples
///
/// ```rust
/// use openvx::{vx_enum_type, constants};
///
/// let t = vx_enum_type!(constants::directives::VX_DIRECTIVE_DISABLE_LOGGING);
/// assert_eq!(t, libopenvx_sys::vx_enum_e_VX_ENUM_DIRECTIVE);
/// ```
#[macro_export]
macro_rules! vx_enum_type {
    ($e:expr) => {
        (($e as libopenvx_sys::vx_uint32) & libopenvx_sys::VX_ENUM_TYPE_MASK) >> 12
    };
}

/// A macro to extract the kernel library enumeration from a enumerated kernel value.
#[macro_export]
// TODO: Needs testing
macro_rules! vx_library {
    ($e:expr) => {
        (($e as libopenvx_sys::vx_uint32) & libopenvx_sys::VX_LIBRARY_MASK) >> 12
    };
}

/// Defines the manner in which to combine the Vendor and Object IDs to get the base value of the enumeration.
///
/// # Examples
///
/// ```rust
/// use openvx::vx_attribute_base;
/// use libopenvx_sys::{vx_int32, vx_vendor_id_e_VX_ID_KHRONOS, vx_type_e_VX_TYPE_THRESHOLD};
///
/// pub const VX_THRESHOLD_THRESHOLD_LOWER: vx_int32 = vx_attribute_base!(
///     vx_vendor_id_e_VX_ID_KHRONOS,
///     vx_type_e_VX_TYPE_THRESHOLD) + 0x2;
///
/// assert_eq!(VX_THRESHOLD_THRESHOLD_LOWER, 526850);
/// ```
#[macro_export]
macro_rules! vx_attribute_base {
    ($vendor:expr, $object:expr) => {
        ((($vendor as libopenvx_sys::vx_uint32) << 20)
            | (($object as libopenvx_sys::vx_uint32) << 8)) as libopenvx_sys::vx_int32
    };
}

/// Defines the manner in which to combine the Vendor and Library IDs to get the base value of the enumeration.
#[macro_export]
// TODO: Needs testing
macro_rules! vx_kernel_base {
    ($vendor:expr, $lib:expr) => {
        ((($vendor as libopenvx_sys::vx_uint32) << 20) | (($lib as libopenvx_sys::vx_uint32) << 12))
            as libopenvx_sys::vx_int32
    };
}

/// Defines the manner in which to combine the Vendor and Object IDs to get the base value of the enumeration.
///
/// # Examples
///
/// ```rust
/// use openvx::vx_enum_base;
/// use libopenvx_sys::{
///     vx_directive_e,
///     vx_vendor_id_e_VX_ID_KHRONOS,
///     vx_enum_e_VX_ENUM_DIRECTIVE,
///     vx_directive_e_VX_DIRECTIVE_ENABLE_LOGGING};
///
/// pub const VX_DIRECTIVE_ENABLE_LOGGING: vx_directive_e = vx_enum_base!(
///     vx_directive_e,
///     vx_vendor_id_e_VX_ID_KHRONOS,
///     vx_enum_e_VX_ENUM_DIRECTIVE) + 0x1;
///
/// assert_eq!(VX_DIRECTIVE_ENABLE_LOGGING, vx_directive_e_VX_DIRECTIVE_ENABLE_LOGGING);
/// ```
#[macro_export]
macro_rules! vx_enum_base {
    ($type:tt, $vendor:expr, $id:expr) => {
        ((($vendor as libopenvx_sys::vx_uint32) << 20) | (($id as libopenvx_sys::vx_uint32) << 12))
            as $type
    };
}
