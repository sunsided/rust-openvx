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
