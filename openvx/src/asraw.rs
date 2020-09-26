pub trait AsRaw {
    type Result;

    fn as_raw(&self) -> Self::Result;
}
