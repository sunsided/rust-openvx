use crate::{VxError, VxStatus};

/// [`Result<T>`][`Result`] is the type used for returning and propagating
/// errors. It is an enum with the variants, [`Ok(T)`], representing
/// success and containing a value, and [`Err(VxError)`], representing error
/// and containing an error value.
///
/// ```
/// # #[allow(dead_code)]
/// enum Result<T> {
///    Ok(T),
///    Err(openvx::VxError),
/// }
/// ```
///
/// [`Result`]: type.Result.html
/// [`Ok(T)`]: type.Result.html#variant.Ok
/// [`Err(VxError)`]: type.Result.html#variant.Err
pub type Result<T> = std::result::Result<T, VxError>;

impl<T> Into<VxStatus> for Result<T> {
    fn into(self) -> VxStatus {
        if let Err(error) = self {
            return VxStatus::Error(error);
        }
        VxStatus::Success
    }
}
