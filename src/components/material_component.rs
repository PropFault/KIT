use std::ops::Deref;
use crate::libs::ecs::component_pool::Component;

trait SpriteMaterial : Component{
    fn get_active_texture(&mut self) -> u64;
    fn update(&mut self, delta : f32);
}

struct Sprite{
    pub frames : Vec<u64>,
    pub fps : f32,
    timer : f32
}

impl Component for Sprite {
    fn new() -> Self {
        Sprite{
            frames: Vec::new(),
            fps: 0.0,
            timer: 0.0
        }
    }

    fn reset(&mut self) {
        self.frames.clear();
        self.fps = 0.0;
        self.timer = 0.0;
    }

    fn get_type_id() -> u64 {
        return 912939455231;
    }
}

impl SpriteMaterial for Sprite{
    fn get_active_texture(&mut self) -> u64 {
        let total_frames = (self.timer * self.fps).round() as usize;
        let index = total_frames % self.frames.len();
        return self.frames[index];
    }

    fn update(&mut self, delta: f32) {
        self.timer += delta;
    }
}