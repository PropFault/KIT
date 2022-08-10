use crate::libs::ecs::component_pool::Component;

struct QuadComponent{
    pub x : f32,
    pub y : f32,
    pub z : f32,
    pub w : f32,
    pub h : f32,
    pub d : f32,
    pub material : u64
}

impl Component for QuadComponent{
    fn new() -> Self {
        QuadComponent{
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
            h: 1.0,
            d: 1.0,
            material: 0
        }
    }

    fn reset(&mut self) {
        self.x = 1.0;
        self.y = 1.0;
        self.z = 1.0;
        self.w = 1.0;
        self.h = 1.0;
        self.d = 1.0;
        self.material = 0;
    }

    fn get_type_id() -> u64 {
        return 129400223;
    }
}