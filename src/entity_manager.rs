use std::collections::HashSet;

#[derive(PartialEq, Copy, Clone, Eq, Hash)]
pub struct Entity(pub u64);

#[derive(Clone)]
pub struct EntityManager {
    // todo: remove pub
    pub entities: HashSet<Entity>,
    current_entity_id: Entity,
}

impl EntityManager {
    pub fn new() -> EntityManager {
        // purposely starting from 1.
        EntityManager {
            current_entity_id: Entity(1),
            entities: HashSet::new(),
        }
    }

    pub fn create(&mut self) -> Entity {
        let new_id = Entity(self.current_entity_id.0);
        self.entities.insert(new_id);
        self.current_entity_id = Entity(self.current_entity_id.0 + 1);
        return new_id;
    }

    // todo: how to export self.entities iter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entitys() {
        let p = Entity(20);
        assert_eq!(p.0, 20);
    }

    #[test]
    fn entity_manager() {
        let p = EntityManager::new();
        assert_eq!(p.current_entity_id.0, 1);
    }

    #[test]
    fn entity_create() {
        let p = EntityManager::new();
        let new_e = p.create();
        let new_f = p.create();
        assert_ne!(new_e, new_f);
    }
}
