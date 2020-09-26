use crate::name::set_name;
use crate::{SetName, VxStatus};
use libopenvx_sys::{
    vxGetStatus, vxQueryReference, vx_context, vx_convolution, vx_delay, vx_distribution, vx_enum,
    vx_graph, vx_image, vx_kernel, vx_lut, vx_matrix, vx_node, vx_parameter, vx_pyramid,
    vx_reference, vx_reference_attribute_e_VX_REFERENCE_COUNT, vx_scalar, vx_size, vx_threshold,
    vx_uint32,
};
use std::borrow::Borrow;

pub trait AsVxReference {
    fn as_reference(&self) -> VxReference;
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct VxReference {
    raw: vx_reference,
}

impl VxReference {
    pub fn is_null(&self) -> bool {
        self.raw.is_null()
    }

    /// Returns the reference count of the object.
    pub fn get_reference_count(&self) -> usize {
        let mut ref_count: vx_uint32 = 0;

        unsafe {
            vxQueryReference(
                self.raw,
                vx_reference_attribute_e_VX_REFERENCE_COUNT as vx_enum,
                &mut ref_count as *mut _ as *mut std::ffi::c_void,
                std::mem::size_of_val(&ref_count) as vx_size,
            );
        }

        ref_count as usize
    }

    pub fn check(&self) {
        let status = unsafe { vxGetStatus(self.raw) };
        let status = VxStatus::from(status);
        if status != VxStatus::Success {
            panic!("ERROR: failed with status {}", status);
        }
    }
}

pub trait Check {
    fn check_status(&self) -> &Self;
}

impl<P> Check for P
where
    P: AsVxReference,
{
    fn check_status(&self) -> &Self {
        let status = unsafe { vxGetStatus(self.as_reference().raw) };
        let status = VxStatus::from(status);
        if status != VxStatus::Success {
            panic!("ERROR: failed with status {}", status);
        }
        self
    }
}

impl SetName for VxReference {
    fn set_name<S>(&self, name: S) -> &Self
    where
        S: Borrow<str>,
    {
        set_name(self.raw, name);
        self
    }
}

impl From<vx_reference> for VxReference {
    fn from(value: vx_reference) -> Self {
        VxReference { raw: value }
    }
}

impl Into<vx_reference> for VxReference {
    fn into(self) -> vx_reference {
        return self.raw;
    }
}

impl From<vx_graph> for VxReference {
    fn from(value: vx_graph) -> Self {
        VxReference {
            raw: value as vx_reference,
        }
    }
}

impl From<vx_node> for VxReference {
    fn from(value: vx_node) -> Self {
        VxReference {
            raw: value as vx_reference,
        }
    }
}

impl From<vx_parameter> for VxReference {
    fn from(value: vx_parameter) -> Self {
        VxReference {
            raw: value as vx_reference,
        }
    }
}

impl From<vx_image> for VxReference {
    fn from(value: vx_image) -> Self {
        VxReference {
            raw: value as vx_reference,
        }
    }
}

impl From<vx_kernel> for VxReference {
    fn from(value: vx_kernel) -> Self {
        VxReference {
            raw: value as vx_reference,
        }
    }
}

impl From<vx_scalar> for VxReference {
    fn from(value: vx_scalar) -> Self {
        VxReference {
            raw: value as vx_reference,
        }
    }
}

impl From<vx_context> for VxReference {
    fn from(value: vx_context) -> Self {
        VxReference {
            raw: value as vx_reference,
        }
    }
}

impl From<vx_lut> for VxReference {
    fn from(value: vx_lut) -> Self {
        VxReference {
            raw: value as vx_reference,
        }
    }
}

impl From<vx_distribution> for VxReference {
    fn from(value: vx_distribution) -> Self {
        VxReference {
            raw: value as vx_reference,
        }
    }
}

impl From<vx_matrix> for VxReference {
    fn from(value: vx_matrix) -> Self {
        VxReference {
            raw: value as vx_reference,
        }
    }
}

impl From<vx_threshold> for VxReference {
    fn from(value: vx_threshold) -> Self {
        VxReference {
            raw: value as vx_reference,
        }
    }
}

impl From<vx_convolution> for VxReference {
    fn from(value: vx_convolution) -> Self {
        VxReference {
            raw: value as vx_reference,
        }
    }
}

impl From<vx_pyramid> for VxReference {
    fn from(value: vx_pyramid) -> Self {
        VxReference {
            raw: value as vx_reference,
        }
    }
}

impl From<vx_delay> for VxReference {
    fn from(value: vx_delay) -> Self {
        VxReference {
            raw: value as vx_reference,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_null() {
        assert!(VxReference::from(std::ptr::null_mut() as vx_reference).is_null());
    }
}
