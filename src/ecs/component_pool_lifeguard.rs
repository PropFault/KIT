use std::collections::HashMap;
use lifeguard::{InitializeWith, Pool, Recycleable};
use rand::{Rng, RngCore};
use rand::rngs::ThreadRng;
use crate::ecs::component_pool::Component;
use crate::ecs::component_pool::ComponentPool;
pub trait LifeguardComponent : Component + Recycleable{}

struct ComponentPoolLifeguard<T : LifeguardComponent>{
    pool : Pool<T>,
    rented_components : HashMap<u64, T>,
    rng : ThreadRng
}

impl <T : LifeguardComponent, A> ComponentPool<T, A> for ComponentPoolLifeguard<T>{
    fn reserve(&mut self, initializer: fn(&mut T, A)) -> u64 {
        let k = self.rng.next_u64()
        let v = self.pool.new_from()
        self.rented_components.insert(self.rng.next_u64(), )
    }

    fn checkout(&mut self, handle: u64) -> &mut T {
        todo!()
    }

    fn _return(&mut self, handle: u64) {
        todo!()
    }
}

impl ComponentPoolLifeguard<>