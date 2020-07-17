use specs::{World, WorldExt, Component, VecStorage, HashMapStorage, FlaggedStorage};
use nphysics2d::{
    ncollide2d::pipeline::CollisionGroups,
    object::{BodyStatus, DefaultBodyHandle, DefaultColliderHandle, Body},
    math::ForceType,
};
use sdl2::rect::Point;

use crate::{
    Vec2,
    Velocity2,
    Image,
    Size,
    Point2,
    Force2,
    Shape,
    Isometry,
    BasicMaterial,
    RigidBodyDesc,
    RigidBody,
};

macro_rules! components {
    ($($component:ident),* $(,)?) => {
        pub fn register_components(world: &mut World) {
            $(world.register::<$component>();)*
        }
    };
}

components! {
    Player,
    Position,
    PhysicsBody,
    PhysicsCollider,
    Sprite,
    CharacterSprites,
    PlatformerControls,
    Health,
    ViewportTarget,
    Currency,
}

/// A marker component given to an entity to indicate that it represents one of
/// the players of the game. This component is automatically added when you call
/// `Game.add_player`.
#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[storage(HashMapStorage)]
pub struct Player;

/// The position of an entity in world coordinates
#[derive(Component, Debug, Clone, PartialEq)]
#[storage(FlaggedStorage)]
pub struct Position(pub Vec2);

/// A physics rigid body
#[derive(Component, Debug, Clone)]
#[storage(FlaggedStorage)]
pub struct PhysicsBody {
    pub handle: Option<DefaultBodyHandle>,
    pub gravity_enabled: bool,
    pub body_status: BodyStatus,
    pub velocity: Velocity2,
    pub angular_inertia: f64,
    pub mass: f64,
    pub local_center_of_mass: Point2,
    pub external_forces: Force2,
}

impl Default for PhysicsBody {
    fn default() -> Self {
        Self {
            handle: Default::default(),
            gravity_enabled: Default::default(),
            body_status: BodyStatus::Dynamic,
            velocity: Velocity2::zero(),
            angular_inertia: Default::default(),
            mass: Default::default(),
            local_center_of_mass: Point2::new(0.0, 0.0),
            external_forces: Force2::zero(),
        }
    }
}

impl PhysicsBody {
    pub(crate) fn to_rigid_body_desc(&self) -> RigidBodyDesc {
        RigidBodyDesc::new()
            .gravity_enabled(self.gravity_enabled)
            .status(self.body_status)
            .velocity(self.velocity)
            .angular_inertia(self.angular_inertia)
            .mass(self.mass)
            .local_center_of_mass(self.local_center_of_mass)
    }

    /// Updates the given rigid body and applies the external forces on this
    /// body to it
    pub(crate) fn apply_to_rigid_body(&mut self, rigid_body: &mut RigidBody) {
        let Self {
            handle: _,
            gravity_enabled,
            body_status,
            velocity,
            angular_inertia,
            mass,
            local_center_of_mass,
            ref mut external_forces,
        } = *self;

        // Update properites
        rigid_body.enable_gravity(gravity_enabled);
        rigid_body.set_status(body_status);
        rigid_body.set_velocity(velocity);
        rigid_body.set_angular_inertia(angular_inertia);
        rigid_body.set_mass(mass);
        rigid_body.set_local_center_of_mass(local_center_of_mass);

        // Applies forces by draining external force property
        let force = *external_forces;
        *external_forces = Force2::zero();
        rigid_body.apply_force(0, &force, ForceType::Force, true);
    }
}

/// A physics collider
#[derive(Component, Debug, Clone)]
#[storage(FlaggedStorage)]
pub struct PhysicsCollider {
    pub(crate) handle: Option<DefaultColliderHandle>,
    pub shape: Shape,
    pub offset_from_parent: Isometry,
    pub density: f64,
    pub material: BasicMaterial,
    pub margin: f64,
    pub collision_groups: CollisionGroups,
    pub linear_prediction: f64,
    pub angular_prediction: f64,
    pub sensor: bool,
}

/// Defines the image that an entity should be drawn with
///
/// The entity must have a Position component so the renderer knows where to
/// draw its sprite.
#[derive(Component, Debug, Clone, PartialEq)]
#[storage(VecStorage)]
pub struct Sprite {
    /// The image to draw
    pub image: Image,
    /// The size of the rectangle used to align the image
    ///
    /// The position of this entity is considered the top left corner of the
    /// rectangle used for alignment.
    pub align_size: Size,
    /// The pivot point to rotate the image around, relative to the position of
    /// this entity.
    ///
    /// If `None`, the image is rotated about its center.
    pub pivot: Option<Point>,
    /// The order in which the sprite should be drawn. Sprites with a higher
    /// draw order will be drawn above sprites with a lower draw order.
    pub draw_order: u8,
}

#[derive(Component, Debug, Clone, PartialEq)]
#[storage(HashMapStorage)]
pub struct CharacterSprites {
    pub idle: Option<Sprite>,
}

impl CharacterSprites {
    pub fn default_sprite(&self) -> Option<Sprite> {
        self.idle.clone()
    }
}

/// An entity with this component will respond to arrow key presses by setting
/// its velocity to the configured values. `left_velocity` and `right_velocity`
/// will be applied to the x-axis velocity. `jump_velocity` will be applied to
/// the y-axis velocity.
#[derive(Component, Debug, Clone, PartialEq)]
#[storage(HashMapStorage)]
pub struct PlatformerControls {
    pub left_velocity: f64,
    pub right_velocity: f64,
    pub jump_velocity: f64,
}

/// The health of an entity
#[derive(Component, Debug, Clone, PartialEq)]
#[storage(HashMapStorage)]
pub struct Health(pub u32);

/// If an entity is given this component, the viewport will attempt to center
/// itself around the position of the entity.
///
/// Warning: Multiple entities should not have this component at the same time.
#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[storage(HashMapStorage)]
pub struct ViewportTarget;

/// If an entity with this component collides with a player, that player will
/// collect this amount of currency and this entity will be removed
///
/// Note that the entity must have some collision geometry in order for
/// collisions to be detected.
#[derive(Component, Debug, Clone, PartialEq)]
#[storage(HashMapStorage)]
pub struct Currency(pub u32);
