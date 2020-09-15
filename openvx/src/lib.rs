mod vxstatus;

pub use crate::vxstatus::*;
use libopenvx_sys::*;

/// A macro to extract the type from an enumerated attribute value.
#[allow(unused_macros)]
// TODO: Needs testing
macro_rules! vx_type {
    ($e:literal) => {
        (($e as vx_uint32) & VX_TYPE_MASK) >> 8
    };
}

/// A macro to extract the enum type from an enumerated value.
#[allow(unused_macros)]
// TODO: Needs testing
macro_rules! vx_enum_type {
    ($e:literal) => {
        (($e as vx_uint32) & VX_ENUM_TYPE_MASK) >> 12
    };
}

/// A macro to extract the kernel library enumeration from a enumerated kernel value.
#[allow(unused_macros)]
// TODO: Needs testing
macro_rules! vx_library {
    ($e:literal) => {
        (($e as vx_uint32) & VX_LIBRARY_MASK) >> 12
    };
}

/// Defines the manner in which to combine the Vendor and Object IDs to get the base value of the enumeration.
macro_rules! vx_attribute_base {
    ($vendor:expr, $object:expr) => {
        ((($vendor as vx_uint32) << 20) | (($object as vx_uint32) << 8)) as vx_int32
    };
}

/// Defines the manner in which to combine the Vendor and Library IDs to get the base value of the enumeration.
// TODO: Needs testing
macro_rules! vx_kernel_base {
    ($vendor:expr, $lib:expr) => {
        ((($vendor as vx_uint32) << 20) | (($lib as vx_uint32) << 12)) as vx_int32
    };
}

/// Defines the manner in which to combine the Vendor and Object IDs to get the base value of the enumeration.
// TODO: Needs testing
macro_rules! vx_enum_base {
    ($vendor:expr, $id:expr) => {
        ((($vendor as vx_uint32) << 20) | (($id as vx_uint32) << 12)) as vx_int32
    };
}

/// The invalid kernel is used to for conformance failure in relation to some kernel operation (Get/Release).
///
/// If the kernel is executed it shall always return an error. The kernel has no parameters. To address by name use `org.khronos.openvx.invalid`.
pub const VX_KERNEL_INVALID: vx_int32 = vx_kernel_base!(
    vx_vendor_id_e_VX_ID_KHRONOS,
    vx_library_e_VX_LIBRARY_KHR_BASE
) + 0x0;

/// The accumulation kernel.
pub const VX_KERNEL_ACCUMULATE: vx_int32 = vx_kernel_base!(
    vx_vendor_id_e_VX_ID_KHRONOS,
    vx_library_e_VX_LIBRARY_KHR_BASE
) + 0x16;

/// The weighted accumulation kernel.
pub const VX_KERNEL_ACCUMULATE_WEIGHTED: vx_int32 = vx_kernel_base!(
    vx_vendor_id_e_VX_ID_KHRONOS,
    vx_library_e_VX_LIBRARY_KHR_BASE
) + 0x17;

/// The squared accumulation kernel.
pub const VX_KERNEL_ACCUMULATE_SQUARE: vx_int32 = vx_kernel_base!(
    vx_vendor_id_e_VX_ID_KHRONOS,
    vx_library_e_VX_LIBRARY_KHR_BASE
) + 0x18;

/// The value type of the threshold. Read-only. Use a `vx_enum` parameter. Will contain a `vx_threshold_type_e`.
pub const VX_THRESHOLD_TYPE: vx_int32 =
    vx_attribute_base!(vx_vendor_id_e_VX_ID_KHRONOS, vx_type_e_VX_TYPE_THRESHOLD) + 0x0;

/// The value type of the threshold. Read-only. Use a `vx_enum` parameter. Will contain a `vx_threshold_type_e`.
pub use VX_THRESHOLD_TYPE as VX_THRESHOLD_ATTRIBUTE_TYPE;

/// The value of the single threshold. Read-write. Use a `vx_int32` parameter.
pub const VX_THRESHOLD_THRESHOLD_VALUE: vx_int32 =
    vx_attribute_base!(vx_vendor_id_e_VX_ID_KHRONOS, vx_type_e_VX_TYPE_THRESHOLD) + 0x1;

/// The value of the single threshold. Read-write. Use a `vx_int32` parameter.
pub use VX_THRESHOLD_THRESHOLD_VALUE as VX_THRESHOLD_ATTRIBUTE_THRESHOLD_VALUE;

/// The value of the lower threshold. Read-write. Use a `vx_int32` parameter.
pub const VX_THRESHOLD_THRESHOLD_LOWER: vx_int32 =
    vx_attribute_base!(vx_vendor_id_e_VX_ID_KHRONOS, vx_type_e_VX_TYPE_THRESHOLD) + 0x2;

/// The value of the lower threshold. Read-write. Use a `vx_int32` parameter.
pub use VX_THRESHOLD_THRESHOLD_LOWER as VX_THRESHOLD_ATTRIBUTE_THRESHOLD_LOWER;

/// The value of the lower threshold. Read-write. Use a `vx_int32` parameter.
pub const VX_THRESHOLD_THRESHOLD_UPPER: vx_int32 =
    vx_attribute_base!(vx_vendor_id_e_VX_ID_KHRONOS, vx_type_e_VX_TYPE_THRESHOLD) + 0x3;

/// The value of the upper threshold. Read-write. Use a `vx_int32` parameter.
pub use VX_THRESHOLD_THRESHOLD_UPPER as VX_THRESHOLD_ATTRIBUTE_THRESHOLD_UPPER;

/// The value of the TRUE threshold (default value is 255). Read-write. Use a `vx_int32` parameter.
pub const VX_THRESHOLD_TRUE_VALUE: vx_int32 =
    vx_attribute_base!(vx_vendor_id_e_VX_ID_KHRONOS, vx_type_e_VX_TYPE_THRESHOLD) + 0x4;

/// The value of the TRUE threshold (default value is 255). Read-write. Use a `vx_int32` parameter.
pub use VX_THRESHOLD_TRUE_VALUE as VX_THRESHOLD_ATTRIBUTE_TRUE_VALUE;

/// The value of the FALSE threshold (default value is 0). Read-write. Use a `vx_int32` parameter.
pub const VX_THRESHOLD_FALSE_VALUE: vx_int32 =
    vx_attribute_base!(vx_vendor_id_e_VX_ID_KHRONOS, vx_type_e_VX_TYPE_THRESHOLD) + 0x5;

/// The value of the FALSE threshold (default value is 0). Read-write. Use a `vx_int32` parameter.
pub use VX_THRESHOLD_FALSE_VALUE as VX_THRESHOLD_ATTRIBUTE_FALSE_VALUE;

/// The data type of the threshold's value. Read-only. Use a `vx_enum` parameter. Will contain a `vx_type_e`.
pub const VX_THRESHOLD_DATA_TYPE: vx_int32 =
    vx_attribute_base!(vx_vendor_id_e_VX_ID_KHRONOS, vx_type_e_VX_TYPE_THRESHOLD) + 0x6;

/// The data type of the threshold's value. Read-only. Use a `vx_enum` parameter. Will contain a `vx_type_e`.
pub use VX_THRESHOLD_DATA_TYPE as VX_THRESHOLD_ATTRIBUTE_DATA_TYPE;

/// The input image format the threshold was created for. Read-only. Use a `vx_enum` parameter. Will contain a `vx_df_image_e`.
pub const VX_THRESHOLD_INPUT_FORMAT: vx_int32 =
    vx_attribute_base!(vx_vendor_id_e_VX_ID_KHRONOS, vx_type_e_VX_TYPE_THRESHOLD) + 0x7;

/// The output image format the threshold was created for. Read-only. Use a `vx_enum` parameter. Will contain a `vx_df_image_e`.
pub const VX_THRESHOLD_OUTPUT_FORMAT: vx_int32 =
    vx_attribute_base!(vx_vendor_id_e_VX_ID_KHRONOS, vx_type_e_VX_TYPE_THRESHOLD) + 0x8;

pub const VX_BIDIRECTIONAL: vx_int32 =
    vx_enum_base!(vx_vendor_id_e_VX_ID_KHRONOS, vx_enum_e_VX_ENUM_DIRECTION) + 0x2;

#[cfg(test)]
mod tests {
    #![macro_use]
    use super::*;

    #[test]
    fn vx_attribute_base_works() {
        assert_eq!(VX_THRESHOLD_THRESHOLD_LOWER, 526850);
        assert_eq!(VX_THRESHOLD_ATTRIBUTE_THRESHOLD_UPPER, 526851);
    }
}
