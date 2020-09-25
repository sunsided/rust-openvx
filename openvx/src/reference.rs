use crate::setname::set_name;
use crate::SetName;
use libopenvx_sys::{_vx_reference, vx_reference};
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
    fn from(value: *mut _vx_reference) -> Self {
        VxReference { raw: value }
    }
}

impl Into<vx_reference> for VxReference {
    fn into(self) -> vx_reference {
        return self.raw;
    }
}

#[cfg(test)]
mod tests {
    use crate::reference::VxReference;

    #[test]
    fn is_null() {
        assert!(VxReference::from(std::ptr::null_mut()).is_null());
    }
}
