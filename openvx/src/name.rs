use crate::types::AsVxReference;
use libopenvx_sys::{
    vxQueryReference, vxSetReferenceName, vx_char, vx_enum, vx_reference,
    vx_reference_attribute_e_VX_REFERENCE_NAME, vx_size,
};
use std::borrow::Borrow;

pub trait SetName {
    fn set_name<S>(&self, name: S) -> &Self
    where
        S: Borrow<str>;
}

pub trait GetName {
    fn get_name(&self) -> String;
}

impl<P> SetName for P
where
    P: AsVxReference,
{
    fn set_name<S>(&self, name: S) -> &Self
    where
        S: Borrow<str>,
    {
        set_name(self.as_reference(), name);
        self
    }
}

impl<P> GetName for P
where
    P: AsVxReference,
{
    fn get_name(&self) -> String {
        let mut ref_name: *mut vx_char = std::ptr::null_mut();

        unsafe {
            vxQueryReference(
                self.as_reference().into(),
                vx_reference_attribute_e_VX_REFERENCE_NAME as vx_enum,
                &mut ref_name as *mut _ as *mut std::ffi::c_void,
                std::mem::size_of_val(&ref_name) as vx_size,
            );

            ref_name_from_cstr(ref_name)
        }
    }
}

pub fn set_name<R, S>(reference: R, name: S) -> vx_reference
where
    R: Into<vx_reference>,
    S: Borrow<str>,
{
    let reference: vx_reference = reference.into();
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

unsafe fn ref_name_from_cstr(name: *const vx_char) -> String {
    if name.is_null() {
        String::from("INVALID_REF_NAME")
    } else {
        std::ffi::CStr::from_ptr(name)
            .to_string_lossy()
            .into_owned()
    }
}
