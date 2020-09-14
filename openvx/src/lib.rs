mod vxstatus;

pub use crate::vxstatus::*;
use libopenvx_sys::*;

/// A macro to extract the type from an enumerated attribute value.
macro_rules! vx_type {
    ($e:literal) => {
        (($e as vx_uint32) & VX_TYPE_MASK) >> 8
    };
}

/// A macro to extract the enum type from an enumerated value.
macro_rules! vx_enum_type {
    ($e:literal) => {
        (($e as vx_uint32) & VX_ENUM_TYPE_MASK) >> 12
    };
}

/// A macro to extract the kernel library enumeration from a enumerated kernel value.
macro_rules! vx_library {
    ($e:literal) => {
        (($e as vx_uint32) & VX_LIBRARY_MASK) >> 12
    };
}

// Defines the manner in which to combine the Vendor and Object IDs to get the base value of the enumeration.
macro_rules! vx_attribute_base {
    ($vendor:expr, $object:expr) => {
        ((($vendor as vx_uint32) << 20) | (($object as vx_uint32) << 8)) as vx_int32
    };
}

// Defines the manner in which to combine the Vendor and Library IDs to get the base value of the enumeration.
macro_rules! vx_kernel_base {
    ($vendor:expr, $lib:expr) => {
        ((($vendor as vx_uint32) << 20) | (($lib as vx_uint32) << 12)) as vx_int32
    };
}

// Defines the manner in which to combine the Vendor and Object IDs to get the base value of the enumeration.
macro_rules! vx_enum_base {
    ($vendor:expr, $id:expr) => {
        ((($vendor as vx_uint32) << 20) | (($id as vx_uint32) << 12)) as vx_int32
    };
}

pub const VX_KERNEL_INVALID: vx_int32 = vx_kernel_base!(
    vx_vendor_id_e_VX_ID_KHRONOS,
    vx_library_e_VX_LIBRARY_KHR_BASE
) + 0x0;
pub const VX_KERNEL_ACCUMULATE: vx_int32 = vx_kernel_base!(
    vx_vendor_id_e_VX_ID_KHRONOS,
    vx_library_e_VX_LIBRARY_KHR_BASE
) + 0x16;
pub const VX_KERNEL_ACCUMULATE_WEIGHTED: vx_int32 = vx_kernel_base!(
    vx_vendor_id_e_VX_ID_KHRONOS,
    vx_library_e_VX_LIBRARY_KHR_BASE
) + 0x17;
pub const VX_KERNEL_ACCUMULATE_SQUARE: vx_int32 = vx_kernel_base!(
    vx_vendor_id_e_VX_ID_KHRONOS,
    vx_library_e_VX_LIBRARY_KHR_BASE
) + 0x18;

pub const VX_THRESHOLD_TYPE: vx_int32 =
    vx_attribute_base!(vx_vendor_id_e_VX_ID_KHRONOS, vx_type_e_VX_TYPE_THRESHOLD) + 0x0;
pub const VX_THRESHOLD_THRESHOLD_VALUE: vx_int32 =
    vx_attribute_base!(vx_vendor_id_e_VX_ID_KHRONOS, vx_type_e_VX_TYPE_THRESHOLD) + 0x1;
pub const VX_THRESHOLD_THRESHOLD_LOWER: vx_int32 =
    vx_attribute_base!(vx_vendor_id_e_VX_ID_KHRONOS, vx_type_e_VX_TYPE_THRESHOLD) + 0x2;
pub const VX_THRESHOLD_THRESHOLD_UPPER: vx_int32 =
    vx_attribute_base!(vx_vendor_id_e_VX_ID_KHRONOS, vx_type_e_VX_TYPE_THRESHOLD) + 0x3;
pub const VX_THRESHOLD_TRUE_VALUE: vx_int32 =
    vx_attribute_base!(vx_vendor_id_e_VX_ID_KHRONOS, vx_type_e_VX_TYPE_THRESHOLD) + 0x4;
pub const VX_THRESHOLD_FALSE_VALUE: vx_int32 =
    vx_attribute_base!(vx_vendor_id_e_VX_ID_KHRONOS, vx_type_e_VX_TYPE_THRESHOLD) + 0x5;
pub const VX_THRESHOLD_DATA_TYPE: vx_int32 =
    vx_attribute_base!(vx_vendor_id_e_VX_ID_KHRONOS, vx_type_e_VX_TYPE_THRESHOLD) + 0x6;

pub use VX_THRESHOLD_DATA_TYPE as VX_THRESHOLD_ATTRIBUTE_DATA_TYPE;
pub use VX_THRESHOLD_FALSE_VALUE as VX_THRESHOLD_ATTRIBUTE_FALSE_VALUE;
pub use VX_THRESHOLD_THRESHOLD_LOWER as VX_THRESHOLD_ATTRIBUTE_THRESHOLD_LOWER;
pub use VX_THRESHOLD_THRESHOLD_UPPER as VX_THRESHOLD_ATTRIBUTE_THRESHOLD_UPPER;
pub use VX_THRESHOLD_THRESHOLD_VALUE as VX_THRESHOLD_ATTRIBUTE_THRESHOLD_VALUE;
pub use VX_THRESHOLD_TRUE_VALUE as VX_THRESHOLD_ATTRIBUTE_TRUE_VALUE;
pub use VX_THRESHOLD_TYPE as VX_THRESHOLD_ATTRIBUTE_TYPE;

pub const VX_BIDIRECTIONAL: vx_int32 =
    vx_enum_base!(vx_vendor_id_e_VX_ID_KHRONOS, vx_enum_e_VX_ENUM_DIRECTION) + 0x2;

#[cfg(test)]
mod tests {
    #![macro_use]
    use super::*;

    #[test]
    fn vx_type_works() {
        assert_eq!(vx_type!(12), 12);
    }

    #[test]
    fn vx_enum_type_works() {
        assert_eq!(vx_enum_type!(12), 12);
    }
}
