pub trait Component{
    fn new() -> Self where Self: Sized;
    fn reset(&mut self);
}

pub trait ComponentPool<T, A> {
    fn reserve(&mut self, initializer: fn(&mut T, A)) -> u64;
    fn checkout(&mut self, handle: u64) -> &mut T;
    fn _return(&mut self, handle: u64);
}