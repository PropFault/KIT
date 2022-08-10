use std::borrow::Borrow;
use std::collections::HashMap;
use std::ops::Deref;
use crate::libs::ecs::entity_component_map::EntityComponentMap;
use crate::Pump;

pub trait System{
    fn think(&mut self, entity_ticket : u64, component_ticket : u64, delta: f64);
    fn get_handled_type(&self) -> u64;
}

pub struct SystemPump<'a>{
    systems : Vec<Box<dyn System>>,
    ecs : &'a dyn EntityComponentMap
}

impl<'a> Pump for SystemPump<'a> {
    fn pump(&mut self) {
        for mut system in self.systems.as_slice(){
            let comps = self.ecs.get_all_components(system.get_handled_type());
            for ref ent in comps{
                for comp in ent.1.as_slice(){
                    system.think(ent.0, *comp, 0.1);
                }
            }
        }
    }
}

impl<'a> SystemPump<'a>{
    pub fn new(ecs : &'a dyn EntityComponentMap) -> SystemPump<'a> {
        SystemPump{
            systems: Vec::new(),
            ecs
        }
    }

    pub fn push(&mut self, system: Box<dyn System>){
        self.systems.push(system);
    }

    pub fn remove(&mut self, idx: usize){
        self.systems.remove(idx);
    }
}
