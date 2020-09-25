use libopenvx_sys::{_vx_reference, vx_reference};

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct VxReference {
    raw: vx_reference,
}

impl VxReference {
    pub fn is_null(&self) -> bool {
        self.raw.is_null()
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

pub trait AsVxReference {
    fn as_reference(&mut self) -> vx_reference;
}

#[cfg(test)]
mod tests {
    use crate::reference::VxReference;

    #[test]
    fn is_null() {
        assert!(VxReference::from(std::ptr::null_mut()).is_null());
    }
}
