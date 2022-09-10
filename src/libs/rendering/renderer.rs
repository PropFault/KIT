use std::sync::{Arc, RwLock};
use crate::libs::rendering::resource_registry::ResourceRegistry;

pub trait Renderer<'a>{
    fn draw_tex(&mut self, texture_ticket : u64, x : i32, y : i32, w : u32, h: u32);
    fn clear(&mut self);
    fn present(&mut self);
    fn registry(&'a mut self) -> &'a mut dyn ResourceRegistry;
}