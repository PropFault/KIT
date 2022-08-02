use std::collections::HashMap;
use lifeguard::{InitializeWith, Pool, Recycleable, Recycled};
use rand::{Rng, RngCore};
use rand::rngs::ThreadRng;
use crate::libs::ecs::component_pool::Component;

pub trait LifeguardComponent : Component + Recycleable{}

pub struct ComponentPoolLifeguard<T : LifeguardComponent>{
    pool : Pool<T>,
    rented_components : HashMap<u64, T>,
    rng : ThreadRng
}

impl <T : LifeguardComponent> ComponentPool<T> for ComponentPoolLifeguard<T>{
    fn reserve<A>(&mut self, initializer: fn(&mut T, A), args : A) -> u64 {
        let k = self.rng.next_u64();
        let mut v = self.pool.detached();
        initializer(&mut v, args);
        self.rented_components.insert(k, v);
        k
    }

    fn checkout(&mut self, handle: u64) -> Option<&mut T> {
        self.rented_components.get_mut(&handle)
    }

    fn _return(&mut self, handle: u64) {
        if let Some(rented) = self.rented_components.remove(&handle){
            self.pool.attach(rented);
        }
    }
}

impl<T: LifeguardComponent> ComponentPoolLifeguard<T>{
    pub fn new() -> ComponentPoolLifeguard<T>{
        return ComponentPoolLifeguard{
            pool: Pool::with_size(10),
            rented_components: HashMap::new(),
            rng: ThreadRng::default()
        }
    }
}