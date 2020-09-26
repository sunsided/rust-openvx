use crate::types::{AsVxReference, VxReference};
use crate::{AsRaw, Release, Result, SetDirective, VxDirective, VxGraph, VxStatus};
use libopenvx_sys::*;

/// An opaque reference to the implementation context.
pub struct VxContext {
    raw: vx_context,
}

impl VxContext {
    fn wrap(context: vx_context) -> Self {
        Self { raw: context }
    }

    pub fn create() -> Self {
        let context = unsafe { vxCreateContext() };
        Self::wrap(context)
    }

    /// Enables recording information for graph debugging.
    pub fn enable_logging(&self) -> Result<&Self> {
        self.set_directive(VxDirective::EnableLogging)?;

        // https://www.khronos.org/registry/OpenVX/specs/1.3/html/OpenVX_Specification_1_3.html#group_log
        unsafe {
            vxRegisterLogCallback(
                self.as_raw(),
                Some(log_callback),
                vx_bool_e_vx_true_e as vx_enum, // TODO: Pass re-entrancy flag to caller
            );
        }
        Ok(self)
    }

    /// Disables recording information for graph debugging.
    pub fn disable_logging(&self) -> Result<&Self> {
        self.set_directive(VxDirective::DisableLogging)?;

        unsafe {
            vxRegisterLogCallback(self.as_raw(), None, vx_bool_e_vx_false_e as vx_enum);
        }
        Ok(self)
    }

    /// Enables performance counters for the context.
    pub fn enable_performance_counters(&self) -> Result<&Self> {
        self.set_directive(VxDirective::EnablePerformance)?;
        Ok(self)
    }

    /// Disables performance counters for the context. By default performance counters are disabled.
    pub fn disable_performance_counters(&self) -> Result<&Self> {
        self.set_directive(VxDirective::DisablePerformance)?;
        Ok(self)
    }

    pub fn create_graph(&self) -> VxGraph {
        let graph = unsafe { vxCreateGraph(self.raw) };
        VxGraph::from(graph)
    }
}

#[allow(unused_variables)]
extern "C" fn log_callback(
    context: vx_context,
    r#ref: vx_reference,
    status: vx_status,
    string: *const vx_char,
) {
    // TODO: Correlate vx_context with context instance for passing state

    debug_assert_ne!(status, vx_status_e_VX_SUCCESS);
    let status = VxStatus::from(status);

    if string.is_null() {
        return;
    }

    let string = unsafe { std::ffi::CStr::from_ptr(string) }
        .to_string_lossy()
        .into_owned();

    if string.len() == 0 {
        return;
    }

    eprint!("ERROR: {} - {}", status, string);

    let require_linebreak = string.chars().last().unwrap() != '\n';
    if require_linebreak {
        eprintln!();
    }
}

impl AsRaw for VxContext {
    type Result = vx_context;

    fn as_raw(&self) -> Self::Result {
        self.raw
    }
}

impl AsVxReference for VxContext {
    fn as_reference(&self) -> VxReference {
        assert!(!self.raw.is_null());
        VxReference::from(self.raw as vx_reference)
    }
}

impl Release for VxContext {
    fn release(&mut self) -> Result<()> {
        if self.raw.is_null() {
            return Ok(());
        }

        let status = unsafe { vxReleaseContext(&mut self.raw) };
        self.raw = std::ptr::null_mut();

        VxStatus::new_result(status, ())
    }
}

impl Drop for VxContext {
    fn drop(&mut self) {
        self.release().unwrap();
    }
}
