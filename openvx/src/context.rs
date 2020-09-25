use crate::{AsRaw, AsVxReference, Release, Result, VxStatus};
use libopenvx_sys::*;

pub struct Context {
    context: vx_context,
}

impl Context {
    pub fn create() -> Self {
        let context = unsafe { vxCreateContext() };
        Self { context }
    }
}

impl AsRaw for Context {
    type Result = vx_context;

    fn as_raw(&mut self) -> Self::Result {
        self.context
    }
}

impl AsVxReference for Context {
    fn as_reference(&mut self) -> vx_reference {
        assert!(!self.context.is_null());
        self.context as vx_reference
    }
}

impl Release for Context {
    fn release(&mut self) -> Result<()> {
        if self.context.is_null() {
            return Ok(());
        }

        let status = unsafe { vxReleaseContext(&mut self.context) };
        self.context = std::ptr::null_mut();

        VxStatus::new_result(status, ())
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        self.release().unwrap();
    }
}
