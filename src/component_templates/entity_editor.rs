use std::ops::{Deref, DerefMut};

use specs::{World, Entity, Component, WorldExt, ReadStorage, WriteStorage, storage::{InsertResult, GenericWriteStorage}};

/// Provides read and write access to the components of a single entity
pub struct EntityEditor<'a> {
    world: &'a World,
    entity: Entity,
}

impl<'a> EntityEditor<'a> {
    pub fn new(world: &'a World, entity: Entity) -> Self {
        Self {world, entity}
    }

    /// Adds a component to this entity
    pub fn add<C: Component>(&self, component: C) -> InsertResult<C> {
        self.world.write_component().insert(self.entity, component)
    }

    /// Removes a component from this entity
    pub fn remove<C: Component>(&self) {
        self.world.write_component::<C>().remove(self.entity);
    }

    /// Checks if this entity has the given component
    pub fn contains<C: Component>(&self) -> bool {
        self.world.read_component::<C>().contains(self.entity)
    }

    /// Gets an immutable reference to the given component or returns `None` if
    /// the entity does not currently have that component
    pub fn get<C: Component>(&self) -> Option<ComponentReadGuard<C>> {
        if self.contains::<C>() {
            Some(ComponentReadGuard {
                storage: self.world.read_component(),
                entity: self.entity,
            })
        } else {
            None
        }
    }

    /// Gets an mutable reference to the given component or returns `None` if
    /// the entity does not currently have that component
    pub fn get_mut<C: Component>(&self) -> Option<ComponentWriteGuard<C>> {
        if self.contains::<C>() {
            Some(ComponentWriteGuard {
                storage: self.world.write_component(),
                entity: self.entity,
            })
        } else {
            None
        }
    }

    /// Gets a mutable reference to the given component or creates and
    /// inserts it using the `Default` trait
    pub fn get_mut_or_default<C: Component + Default>(&self) -> ComponentWriteGuard<C> {
        let mut storage = self.world.write_component();
        storage.get_mut_or_default(self.entity);

        ComponentWriteGuard {
            storage,
            entity: self.entity,
        }
    }
}

/// Provides immutable access to a component through the `Deref` trait
pub struct ComponentReadGuard<'a, T: Component> {
    storage: ReadStorage<'a, T>,
    entity: Entity,
}

impl<'a, T: Component> Deref for ComponentReadGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.storage.get(self.entity)
            .expect("bug: read guard guarantees that entity exists")
    }
}

/// Provides immutable and mutable access to a component through the `Deref` and
/// `DerefMut` traits
pub struct ComponentWriteGuard<'a, T: Component> {
    storage: WriteStorage<'a, T>,
    entity: Entity,
}

impl<'a, T: Component> Deref for ComponentWriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.storage.get(self.entity)
            .expect("bug: write guard guarantees that entity exists")
    }
}

impl<'a, T: Component> DerefMut for ComponentWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.storage.get_mut(self.entity)
            .expect("bug: write guard guarantees that entity exists")
    }
}
