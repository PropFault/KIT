
pub trait Component{
    fn new() -> Self;
    fn reset(&mut self);
    fn get_type_id() -> u64;
}



pub trait ReadableComponentPool<T> {
    fn checkout(&mut self, handle: u64) -> &mut Option<T>;
    fn _return(&mut self, handle: u64);
}

pub trait ComponentPool<T> : ReadableComponentPool<T>{
    fn reserve<A>(&mut self, initializer: fn(&mut T, A), args : A) -> u64;

}