pub use nphysics2d::ncollide2d::query::Proximity;

use std::collections::HashMap;

use specs::{shrev::EventChannel, Entity};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContactType {
    /// Indicates that two collision objects started being in contact
    ///
    /// This event is generated whenever there is a contact between two
    /// collision objects that did not have any contact at the last update.
    Started,

    /// Indicates that two collision objects stopped being in contact
    ///
    /// This event is generated whenever there is not a contact between two
    /// collision objects that did have at least one contact at the last update.
    Stopped,
}

/// Event for when two collision objects start or stop being in contact
#[derive(Debug, Clone)]
pub struct ContactEvent {
    pub collider1: Entity,
    pub collider2: Entity,

    /// The type of contact that occurred between the two colliders
    pub contact_type: ContactType,
}

pub type ContactEvents = EventChannel<ContactEvent>;

/// Event for when two collision objects start or stop being in close proximity,
/// contact, or disjoint
#[derive(Debug, Clone)]
pub struct ProximityEvent {
    pub collider1: Entity,
    pub collider2: Entity,

    /// The previous state of proximity between the two collision objects
    pub prev_status: Proximity,
    /// The new state of proximity between the two collision objects
    pub current_status: Proximity,
}

pub type ProximityEvents = EventChannel<ProximityEvent>;

/// The entities that are touching or intersecting with a given entity.
#[derive(Debug, Clone, PartialEq)]
pub struct Collisions {
    /// The entities touching the top of the entity with this component
    pub touching_top: Vec<Entity>,
    /// The entities touching the bottom of the entity with this component
    pub touching_bottom: Vec<Entity>,
    /// The entities touching the left of the entity with this component
    pub touching_left: Vec<Entity>,
    /// The entities touching the right of the entity with this component
    pub touching_right: Vec<Entity>,
    /// The entities intersecting the entity with this component
    pub intersecting: Vec<Entity>,
}

impl Default for Collisions {
    fn default() -> Self {
        Self::new()
    }
}

impl Collisions {
    pub const fn new() -> Self {
        Self {
            //TODO: Ideally we would use a HashSet, but HashSet::new() isn't const fn yet
            touching_top: Vec::new(),
            touching_bottom: Vec::new(),
            touching_left: Vec::new(),
            touching_right: Vec::new(),
            intersecting: Vec::new(),
        }
    }
}

/// A map of entity to the other entities colliding or intersecting with it.
///
/// Updated by the `CollisionDetector` system after physics information has been
/// updated.
#[derive(Debug, Default)]
pub struct CollisionsMap {
    //TODO: Should we remove entities that are no longer colliding with
    //anything? This can potentially leak memory for short-lived colliders.
    collisions: HashMap<Entity, Collisions>,
}

impl CollisionsMap {
    /// Gets the collisions that have been recorded for the given entity
    pub fn get(&self, entity: Entity) -> &Collisions {
        // Possible versions of this API:
        // 1. take `&self` and return `Option<&Collisions>`
        // 2. take `&mut self` and insert a default set of collisions when none
        //    exist yet
        // 3. return an immutable, static empty set of collisions
        //
        // Since an entry is only added to this map once a collision is actually
        // detected for the first time, it's possible that another piece of code
        // will try to get the collisions and find that there is nothing in the
        // map yet. This can be a little awkward to handle and (1) forces the
        // user to consider that case when in reality they probably just want an
        // empty set of collisions. (2) returns an empty set of collisions, but
        // might end up populating the map with a bunch of entities that don't
        // need to be there. (3) provides a convenient API without unnecessary
        // allocations. That's why the code below is the way it is.
        static DEFAULT_COLLISIONS: Collisions = Collisions::new();

        self.collisions.get(&entity).unwrap_or(&DEFAULT_COLLISIONS)
    }

    /// Gets the collisions that have been recorded for the given entity and
    /// inserts a default set of collisions if none have been recorded yet.
    pub fn get_or_default(&mut self, entity: Entity) -> &mut Collisions {
        self.collisions.entry(entity).or_default()
    }

    /// Gets the collisions that have been recorded for two distinct entities
    /// and inserts a default set of collisions if none have been recorded yet.
    ///
    /// # Panics
    ///
    /// Panics if both requested entities are the same entity.
    pub fn get_or_default2(&mut self, entity1: Entity, entity2: Entity) -> (&mut Collisions, &mut Collisions) {
        assert_ne!(entity1, entity2, "bug: entities were not distinct");

        let collisions1 = self.get_or_default(entity1) as *mut _;
        let collisions2 = self.get_or_default(entity2);

        // Safety: since we have checked that the keys are distinct, their
        // values must be distinct too. Thus, we do not mutably reference the
        // same data twice.
        let collisions1 = unsafe { &mut *collisions1 };
        (collisions1, collisions2)
    }
}
