use crate::scene::Scene;

pub trait Renderer{
    fn render(&self, scene: dyn Scene) -> Vec<u8>;
}