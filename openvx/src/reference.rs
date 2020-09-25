use libopenvx_sys::vx_reference;

pub trait AsVxReference {
    fn as_reference(&mut self) -> vx_reference;
}
