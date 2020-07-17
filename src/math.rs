use nphysics2d::{object::DefaultBodyHandle, ncollide2d::shape::ShapeHandle};

// Math types
pub type Vec2 = nphysics2d::math::Vector<f64>;
pub type Point2 = nphysics2d::math::Point<f64>;
pub type Mat2 = nphysics2d::math::Matrix<f64>;
pub type Velocity2 = nphysics2d::math::Velocity<f64>;
pub type Force2 = nphysics2d::math::Force<f64>;
pub type Isometry = nphysics2d::math::Isometry<f64>;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

// Physics types
pub type BasicMaterial = nphysics2d::material::BasicMaterial<f64>;
pub type RigidBodyDesc = nphysics2d::object::RigidBodyDesc<f64>;
pub type RigidBody = nphysics2d::object::RigidBody<f64>;
pub type ColliderDesc = nphysics2d::object::ColliderDesc<f64>;
pub type Collider = nphysics2d::object::Collider<f64, DefaultBodyHandle>;

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Rect {width: f64, height: f64},
    Ellipse {width: f64, height: f64},
    Polyline {points: Vec<Vec2>},
    Polygon {points: Vec<Vec2>},
}

impl Shape {
    pub(crate) fn handle(&self, margin: f64) -> ShapeHandle<f64> {
        todo!()
    }
}
