use crate::Result;

pub trait CheckStatus {
    fn check_status(&self) -> Result<&Self>;
}
