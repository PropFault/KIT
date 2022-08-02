use crate::quad::Quad;

pub trait Display : Quad {
    //major functions
    fn present(&self) -> bool;
    fn write_ready(&self) -> bool;
    fn set_buffer(&self, buffer: Vec<u8>) -> bool;
}