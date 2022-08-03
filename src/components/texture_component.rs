use crate::FileResource;
use crate::libs::ecs::component_pool::Component;
use crate::libs::loading::image_loader::{ImageLoader};
use crate::libs::loading::loader::Loader;
use crate::libs::rendering::resource_registry::ResourceRegistry;

pub struct TextureComponent{
    pub texture_ticket : Option<u64>
}

impl Component for TextureComponent {
    fn new() -> Self {
        TextureComponent{
            texture_ticket: None
        }
    }

    fn reset(&mut self) {
        self.texture_ticket = None
    }

    fn get_type_id() -> u64 {
        3949329495
    }
}

impl TextureComponent {
    pub(crate) fn initialize(&mut self, args: (&mut FileResource, &mut dyn ResourceRegistry)){
        let mut imageLoader = ImageLoader::new();

        self.texture_ticket = Option::from(args.1.register_texture(&mut imageLoader.load(args.0)));
    }
}