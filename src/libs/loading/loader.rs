use crate::loading::resource::Resource;

pub trait Loader<T>{
    fn load(&mut self, resource: &dyn Resource) -> T;
}