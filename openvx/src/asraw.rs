pub trait AsRaw {
    type Result;

    fn as_raw(&mut self) -> Self::Result;
}
