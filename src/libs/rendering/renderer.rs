
pub trait Renderer{
    fn draw_tex(&mut self, texture_ticket : u64, x : i32, y : i32, w : u32, h: u32);
    fn clear(&mut self);
    fn present(&mut self);
}