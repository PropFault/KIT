use crate::libs::loading::image_loader::ImageRaw;

pub trait ResourceRegistry<'a>{
    fn register_texture(& 'a mut self, image : &mut ImageRaw) -> u64;
}