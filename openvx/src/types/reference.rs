use crate::setname::set_name;
use crate::SetName;
use libopenvx_sys::{
    vx_context, vx_convolution, vx_delay, vx_distribution, vx_graph, vx_image, vx_kernel, vx_lut,
    vx_matrix, vx_node, vx_parameter, vx_pyramid, vx_reference, vx_scalar, vx_threshold,
};
use std::borrow::Borrow;

pub trait AsVxReference {
    fn as_reference(&mut self) -> VxReference;
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct VxReference {
    raw: vx_reference,
}

impl VxReference {
    pub fn is_null(&self) -> bool {
        self.raw.is_null()
    }
}

impl SetName for VxReference {
    fn set_name<S>(&mut self, name: S) -> &mut Self
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
