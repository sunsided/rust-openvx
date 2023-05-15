use crate::Result;

pub trait Release {
    fn release(&mut self) -> Result<()>;
}
