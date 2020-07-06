use sdl2::rect::Point;
use specs::{Component, VecStorage, HashMapStorage};

//TODO: Use nalgebra and define a math module
pub type Vec2d = Point;

/// The position of an entity in world coordinates
#[derive(Component, Debug, Clone, PartialEq)]
#[storage(VecStorage)]
pub struct Position(pub Vec2d);

//TODO: Remove this impl after we switch to nalgebra
impl From<(f64, f64)> for Position {
    fn from((x, y): (f64, f64)) -> Self {
        Position(Vec2d::new(x as i32, y as i32))
    }
}

//TODO: Remove this impl after we switch to nalgebra
impl Position {
    pub fn into_f64(&self) -> (f64, f64) {
        (self.0.x() as f64, self.0.y() as f64)
    }

    pub fn set_x(&mut self, x: f64) {
        let (_, y) = self.into_f64();
        *self = Self::from((x, y))
    }

    pub fn set_y(&mut self, y: f64) {
        let (x, _) = self.into_f64();
        *self = Self::from((x, y))
    }
}

/// A marker component given to an entity to indicate that it represents one of
/// the players of the game. This component is automatically added when you call
/// `Game.add_player`.
#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[storage(HashMapStorage)]
pub struct Player;

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
