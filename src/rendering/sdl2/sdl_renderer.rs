use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};
use crate::libs::rendering::renderer::Renderer;
use crate::SDLResourceRegistry;

pub struct SDLRenderer<'a, T: RenderTarget, C>{
    canvas : Canvas<T>,
    registry : SDLResourceRegistry<'a, C>
}

impl<'a, T: RenderTarget, C> Renderer for SDLRenderer<'a, T, C> {
    fn draw_tex(&mut self, texture_ticket: u64, x: i32, y: i32, w: u32, h: u32) {
        self.canvas.copy(self.registry.checkout_texture(texture_ticket), None, Some(Rect::new(x, y, w, h))).expect("");
    }
}

impl<'a, T: RenderTarget, C> SDLRenderer<'a, T, C>{
    pub fn new(canvas: Canvas<T>, registry: SDLResourceRegistry<'a, C>) -> SDLRenderer<'a, T, C> {
        SDLRenderer{ canvas, registry }
    }
}