use crate::libs::loading::image_loader::ImageRaw;

pub trait ResourceRegistry{
    fn register_texture(& mut self, image : &mut ImageRaw) -> u64;
}