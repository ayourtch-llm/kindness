use std::any::{Any, TypeId};
use std::collections::HashMap;

pub type EntityId = u64;

pub struct World {
    next_id: EntityId,
    entities: HashMap<EntityId, HashMap<TypeId, Box<dyn Any>>>,
}

impl World {
    pub fn new() -> Self {
        World {
            next_id: 0,
            entities: HashMap::new(),
        }
    }

    pub fn create_entity(&mut self) -> EntityId {
        let id = self.next_id;
        self.next_id += 1;
        self.entities.insert(id, HashMap::new());
        id
    }

    pub fn add_component<T: 'static>(&mut self, entity: EntityId, component: T) {
        if let Some(components) = self.entities.get_mut(&entity) {
            components.insert(TypeId::of::<T>(), Box::new(component));
        }
    }

    pub fn get_component<T: 'static>(&self, entity: EntityId) -> Option<&T> {
        self.entities
            .get(&entity)?
            .get(&TypeId::of::<T>())?
            .downcast_ref::<T>()
    }

    pub fn remove_component<T: 'static>(&mut self, entity: EntityId) -> bool {
        if let Some(components) = self.entities.get_mut(&entity) {
            components.remove(&TypeId::of::<T>()).is_some()
        } else {
            false
        }
    }

    pub fn query<T: 'static>(&self) -> Vec<(EntityId, &T)> {
        let type_id = TypeId::of::<T>();
        let mut results = Vec::new();
        for (&entity_id, components) in &self.entities {
            if let Some(component) = components.get(&type_id) {
                if let Some(val) = component.downcast_ref::<T>() {
                    results.push((entity_id, val));
                }
            }
        }
        results
    }

    pub fn destroy_entity(&mut self, entity: EntityId) -> bool {
        self.entities.remove(&entity).is_some()
    }
}
