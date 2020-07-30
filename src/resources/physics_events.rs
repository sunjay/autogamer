pub use nphysics2d::ncollide2d::query::Proximity;

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
