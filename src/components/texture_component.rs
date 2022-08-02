use std::ops::Deref;
use rand::{RngCore, thread_rng};
use rand::rngs::ThreadRng;
use crate::{Component, ImageRaw};
use crate::rendering::resource_registry::ResourceRegistry;

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
    fn initialize(&mut self, args: (&ImageRaw, &mut dyn ResourceRegistry)){
        self.texture_ticket = Option::from(args.1.register_texture(args.0));
    }
}