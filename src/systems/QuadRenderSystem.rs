use crate::ComponentPool;
use crate::libs::ecs::component_pool::ReadableComponentPool;
use crate::libs::ecs::system::System;
use crate::quad::Quad;

struct QuadRenderSystem<'a>{
    quad_pool : &'a mut dyn ReadableComponentPool<dyn Quad>

}

impl<'a> System for QuadRenderSystem<'a>{
    fn think(&mut self, entity_ticket: u64, component_ticket: u64, delta: f64) {
        let quad = self.quad_pool.checkout(component_ticket);

    }

    fn get_handled_type(&self) -> u64 {

    }
}