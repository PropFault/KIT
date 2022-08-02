use std::collections::HashMap;

pub trait EntityComponentMap{
    fn add_component(&mut self, entity: u64, component_type_id: u64, component: u64);
    fn get_component(&self, entity: u64, component_type_id: u64) -> Option<&u64>;
    fn get_components(&self, entity: u64, component_type_id: u64) -> Option<&Vec<u64>>;
    fn get_all_components(&self, component_type_id: u64) -> HashMap<u64, Vec<u64>>;
    fn remove_component(&mut self, entity: u64, component: u64);
}