use crate::libs::ecs::component_pool::Component;
use crate::quad::Quad;

pub struct QuadComponent{
    pub quad : Option<Box<dyn Quad>>,
    pub material : u64
}

impl Component for QuadComponent{
    fn new() -> Self {
        QuadComponent{
            quad: Option::None,
            material: 0
        }
    }

    fn reset(&mut self) {
        self.quad = Option::None;
        self.material = 0;
    }

    fn get_type_id() -> u64 {
        return 129400223;
    }
}