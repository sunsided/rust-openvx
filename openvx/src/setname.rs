use crate::AsVxReference;
use libopenvx_sys::{vxSetReferenceName, vx_reference};
use std::borrow::Borrow;

pub trait SetName {
    fn set_name<S>(&mut self, name: S) -> &mut Self
    where
        S: Borrow<str>;
}

impl<P> SetName for P
where
    P: AsVxReference,
{
    fn set_name<S>(&mut self, name: S) -> &mut Self
    where
        S: Borrow<str>,
    {
        set_name(self.as_reference(), name);
        self
    }
}

pub fn set_name<S>(reference: vx_reference, name: S) -> vx_reference
where
    S: Borrow<str>,
{
    assert!(!reference.is_null());
    unsafe {
        vxSetReferenceName(
            reference,
            std::ffi::CString::new(name.borrow())
                .expect("CString::new failed")
                .as_ptr(),
        );
    }

    reference
}
