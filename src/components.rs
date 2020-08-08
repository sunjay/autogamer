use specs::{World, WorldExt, Component, VecStorage, HashMapStorage, FlaggedStorage, NullStorage};
use nphysics2d::{
    math::ForceType,
    object::{BodyStatus, DefaultBodyHandle, DefaultColliderHandle, Body, BodyPart},
    material::MaterialHandle,
};
use sdl2::rect::Point;

use crate::{
    Vec2,
    Isometry,
    Velocity2,
    Image,
    Size,
    Point2,
    Force2,
    Shape,
    BasicMaterial,
    RigidBodyDesc,
    RigidBody,
    ColliderDesc,
    Collider,
    ShapeRect,
};

pub use nphysics2d::ncollide2d::pipeline::CollisionGroups;

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
    Wallet,
    Currency,
}

/// A marker component given to an entity to indicate that it represents one of
/// the players of the game. This component is automatically added when you call
/// `Game.add_player`.
#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[storage(NullStorage)]
pub struct Player;

/// The position of an entity in world coordinates
#[derive(Component, Debug, Clone, PartialEq)]
#[storage(FlaggedStorage)]
pub struct Position(pub Vec2);

/// A physics rigid body
#[derive(Component, Debug, Clone)]
#[storage(FlaggedStorage)]
pub struct PhysicsBody {
    /// Hidden because this field should not be modified outside this crate but
    /// we still want struct update syntax to work
    #[doc(hidden)]
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
            gravity_enabled: true,
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
        let Self {
            handle: _,
            gravity_enabled,
            body_status,
            velocity,
            angular_inertia,
            mass,
            local_center_of_mass,
            external_forces: _,
        } = *self;

        RigidBodyDesc::new()
            .gravity_enabled(gravity_enabled)
            .status(body_status)
            .velocity(velocity)
            .angular_inertia(angular_inertia)
            .mass(mass)
            .local_center_of_mass(local_center_of_mass)
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

        // Update properites -- need to check if they have changed first so we
        // don't invalidate the cache every frame for no reason

        // These methods do not invalidate the cache if the value hasn't changed
        // so no extra check is necessary
        rigid_body.enable_gravity(gravity_enabled);
        rigid_body.set_status(body_status);

        let rb_vel = rigid_body.velocity();
        if rb_vel.angular != velocity.angular || rb_vel.linear != velocity.linear {
            rigid_body.set_velocity(velocity);
        }

        let local_inertia = rigid_body.local_inertia();
        if local_inertia.angular != angular_inertia {
            rigid_body.set_angular_inertia(angular_inertia);
        }
        if local_inertia.linear != mass {
            rigid_body.set_mass(mass);
        }

        if rigid_body.local_center_of_mass() != local_center_of_mass {
            rigid_body.set_local_center_of_mass(local_center_of_mass);
        }

        // Applies forces by draining external force property
        let force = *external_forces;
        if force.linear != Vec2::new(0.0, 0.0) || force.angular != 0.0 {
            rigid_body.apply_force(0, &force, ForceType::Force, true);
            *external_forces = Force2::zero();
        }
    }

    pub(crate) fn update_from_rigid_body(&mut self, rigid_body: &RigidBody) {
        let Self {
            handle: _,
            gravity_enabled,
            body_status,
            velocity,
            angular_inertia,
            mass,
            local_center_of_mass,
            // Not a part of the RigidBody (specific to this component)
            external_forces: _,
        } = self;

        *gravity_enabled = rigid_body.gravity_enabled();
        *body_status = rigid_body.status();
        *velocity = *rigid_body.velocity();
        // Adapted from: https://github.com/amethyst/specs-physics/blob/8ec2243f25e5b994af3a6a0c2ae80bc5ebf65b7f/src/bodies.rs#L118-L120
        let local_inertia = rigid_body.local_inertia();
        *angular_inertia = local_inertia.angular;
        *mass = local_inertia.linear;
        *local_center_of_mass = rigid_body.local_center_of_mass();
    }
}

/// A physics collider
#[derive(Component, Debug, Clone)]
#[storage(FlaggedStorage)]
pub struct PhysicsCollider {
    /// Hidden because this field should not be modified outside this crate but
    /// we still want struct update syntax to work
    #[doc(hidden)]
    pub handle: Option<DefaultColliderHandle>,
    /// Updating this after the component is initially added is not supported
    pub shape: Shape,
    /// The offset of this collider from the position of the parent body
    ///
    /// If there is no parent body, this field is ignored
    pub offset: Vec2,
    /// Updating this after the component is initially added is not supported
    pub density: f64,
    /// Updating this after the component is initially added is not supported
    pub material: BasicMaterial,
    pub margin: f64,
    pub collision_groups: CollisionGroups,
    /// Updating this after the component is initially added is not supported
    pub sensor: bool,
}

impl Default for PhysicsCollider {
    fn default() -> Self {
        Self {
            handle: Default::default(),
            shape: Shape::Rect(ShapeRect::new(Vec2::new(0.0, 0.0))),
            offset: Default::default(),
            density: Default::default(),
            material: Default::default(),
            margin: 0.01,
            collision_groups: Default::default(),
            sensor: Default::default(),
        }
    }
}

impl PhysicsCollider {
    /// Collision group for any ground, wall, or other obstacle
    pub const GROUND_COLLISION_GROUP: usize = 0;
    pub const PLAYER_COLLISION_GROUP: usize = 1;
    pub const ENEMY_COLLISION_GROUP: usize = 2;

    pub fn ground_collision_groups() -> CollisionGroups {
        CollisionGroups::new()
            .with_membership(&[Self::GROUND_COLLISION_GROUP])
            .with_blacklist(&[Self::GROUND_COLLISION_GROUP])
    }

    pub fn player_collision_groups() -> CollisionGroups {
        CollisionGroups::new()
            .with_membership(&[Self::PLAYER_COLLISION_GROUP])
            .with_blacklist(&[Self::PLAYER_COLLISION_GROUP])
    }

    pub fn enemy_collision_groups() -> CollisionGroups {
        CollisionGroups::new()
            .with_membership(&[Self::ENEMY_COLLISION_GROUP])
            .with_blacklist(&[Self::ENEMY_COLLISION_GROUP])
    }

    pub(crate) fn to_collider_desc(&self, base_pos: Vec2) -> ColliderDesc {
        let Self {
            handle: _,
            ref shape,
            offset,
            density,
            material,
            margin,
            collision_groups,
            sensor,
        } = *self;

        ColliderDesc::new(shape.to_handle())
            .position(Isometry::new(base_pos + offset, 0.0))
            .density(density)
            .material(MaterialHandle::new(material))
            .margin(margin)
            .collision_groups(collision_groups)
            .sensor(sensor)
    }

    pub(crate) fn update_collider(&self, collider: &mut Collider, base_pos: Vec2) {
        let Self {
            handle: _,
            // Updating shape is not currently supported because the various
            // shape primitives do not implement PartialEq
            shape: _,
            offset,
            density,
            // Updating the material is not supported and checking if it changed
            // isn't easy because BasicMaterial doesn't implement PartialEq
            material: _,
            margin,
            // Updating shape is not currently supported because the collision
            // groups type does not implement PartialEq
            collision_groups: _,
            sensor,
        } = *self;

        // Need to check each property first so we don't invalidate caches when
        // there is no update

        let pos = base_pos + offset;
        if collider.position().translation.vector != pos {
            collider.set_position(Isometry::new(pos, 0.0));
        }

        // No way to update the density currently in nphysics API
        assert!((density - collider.density()).abs() < 0.0001,
            "changing collider density is not supported");

        if collider.margin() != margin {
            collider.set_margin(margin);
        }

        // No way to update is_sensor
        assert_eq!(collider.is_sensor(), sensor,
            "changing the sensor property of a collider is not supported");
    }
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
#[storage(NullStorage)]
pub struct ViewportTarget;

/// The amount of currency collected by this entity so far.
///
/// The balance may become negative if enough negative-value currency components
/// are collected.
///
/// This component must be present for an entity to be able to interact with
/// other entities that have `Currency` components.
#[derive(Component, Debug, Default, Clone, PartialEq)]
#[storage(HashMapStorage)]
pub struct Wallet(pub i32);

/// If an entity with this component collides with an entity that has a
/// `Wallet` component, this amount of currency will be added to the wallet
/// and the entity with this currency component will be removed.
///
/// If the amount is negative, this will actually remove currency from the
/// wallet instead of adding it.
///
/// Note that the entity must have some collision geometry in order for
/// collisions to be detected.
#[derive(Component, Debug, Clone, PartialEq)]
#[storage(HashMapStorage)]
pub struct Currency(pub i32);
