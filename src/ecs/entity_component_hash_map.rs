use std::collections::HashMap;
use crate::ecs::entity_component_map::EntityComponentMap;

pub type EntityComponentHashMap = HashMap<u64, HashMap<u64, Vec<u64>>>;

impl EntityComponentMap for EntityComponentHashMap{
    fn add_component(&mut self, entity: u64, component_type_id: u64, component: u64) {
        self.entry(entity).or_insert_with(HashMap::new)
            .entry(component_type_id).or_insert_with(Vec::new)
            .push(component);
    }

    fn get_component(&self, entity: u64, component_type_id: u64) -> Option<&u64> {
        return match self.get_components(entity, component_type_id) {
            Some( components) => components.first(),
            None => None,
        }
    }
    fn get_components(&self, entity: u64, component_type_id: u64) -> Option<&Vec<u64>> {
        return match self.get(&entity) {
            Some(t_map) => {
                return t_map.get(&component_type_id);
            }
            None => None,
        }
    }

    fn get_all_components(&self, component_type_id: u64) -> HashMap<u64, Vec<u64>> {
        let mut map : HashMap<u64, Vec<u64>> = HashMap::new();
        for key in self.keys(){
            if let Some(components) = self.get_components(*key, component_type_id) {
                map.insert(*key, components.clone());
            }
        }
        map
    }

    fn remove_component(&mut self, entity: u64, component: u64) {
        for t_map in self.entry(entity).or_insert_with(HashMap::new){
            t_map.1.retain( |v| *v != component);
        }
    }
}