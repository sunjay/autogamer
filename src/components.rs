use specs::{World, WorldExt, Component, VecStorage, HashMapStorage};
use sdl2::rect::Point;

use crate::{Vec2, Image, Size};

macro_rules! components {
    ($($component:ident),* $(,)?) => {
        pub fn register_components(world: &mut World) {
            $(world.register::<$component>();)*
        }
    };
}

components! {
    Position,
    Player,
    Sprite,
    PlatformerControls,
    Health,
    ViewportTarget,
    Currency,
}

/// The position of an entity in world coordinates
#[derive(Component, Debug, Clone, PartialEq)]
#[storage(VecStorage)]
pub struct Position(pub Vec2);

/// A marker component given to an entity to indicate that it represents one of
/// the players of the game. This component is automatically added when you call
/// `Game.add_player`.
#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[storage(HashMapStorage)]
pub struct Player;

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
