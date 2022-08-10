use std::borrow::BorrowMut;
use std::collections::HashMap;
use lifeguard::{Pool, Recycleable};
use rand::RngCore;
use rand::rngs::ThreadRng;
use crate::libs::ecs::component_pool::{Component, ComponentPool, ReadableComponentPool};

struct LifeguardComponentAdapter<T : Component>{
    pub component : Option<T>
}

trait LifeguardComponent : Component + Recycleable{}

impl<T : Component> Component for LifeguardComponentAdapter<T> {
    fn new() -> Self {
        LifeguardComponentAdapter{
            component: None
        }
    }

    fn reset(&mut self) {
        self.component = None;
    }

    fn get_type_id() -> u64 {
        return T::get_type_id();
    }
}

impl<T : Component> Recycleable for LifeguardComponentAdapter<T> {
    fn new() -> Self {
        <Self as Component>::new()
    }

    fn reset(&mut self) {
        <Self as Component>::reset(self);
    }
}

impl<T: Component> LifeguardComponent for LifeguardComponentAdapter<T> {

}

pub struct ComponentPoolLifeguard<T : Component>{
    pool : Pool<LifeguardComponentAdapter<T>>,
    rented_components : HashMap<u64, LifeguardComponentAdapter<T>>,
    rng : ThreadRng,
    none_ref : Option<T>
}

impl<T: Component> ReadableComponentPool<T> for ComponentPoolLifeguard<T> {
    fn checkout(&mut self, handle: u64) -> &mut Option<T> {
        match self.rented_components.get_mut(&handle){
            Some(c) => &mut c.component,
            None => &mut self.none_ref
        }
    }

    fn _return(&mut self, handle: u64) {
        if let Some(rented) = self.rented_components.remove(&handle){
            self.pool.attach(rented);
        }
    }
}

impl <T : Component> ComponentPool<T> for ComponentPoolLifeguard<T>{
    fn reserve<A>(&mut self, initializer: fn(&mut T, A), args : A) -> u64 {
        let k = self.rng.next_u64();
        let mut v = self.pool.detached();
        v.component = Some(T::new());
        initializer(v.component.as_mut().unwrap(), args);
        self.rented_components.insert(k, v);
        k
    }
}

impl<T: Component> ComponentPoolLifeguard<T>{
    pub fn new() -> ComponentPoolLifeguard<T>{
        return ComponentPoolLifeguard{
            pool: Pool::with_size(10),
            rented_components: HashMap::new(),
            rng: ThreadRng::default(),
            none_ref: None
        }
    }
}