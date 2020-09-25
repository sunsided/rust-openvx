use crate::Result;

pub trait Release: Drop {
    fn release(&mut self) -> Result<()>;
}
