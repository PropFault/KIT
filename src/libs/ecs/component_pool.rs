use rand::{RngCore, thread_rng};

pub trait Component{
    fn new() -> Self;
    fn reset(&mut self);
    fn get_type_id() -> u64;
}

pub trait ComponentPool<T> {
    fn reserve<A>(&mut self, initializer: fn(&mut T, A), args : A) -> u64;
    fn checkout(&mut self, handle: u64) -> Option<&mut T>;
    fn _return(&mut self, handle: u64);
}