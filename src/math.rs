use std::fmt;

use nphysics2d::{
    object::DefaultBodyHandle,
    ncollide2d::{bounding_volume::local_aabb, shape::ShapeHandle},
};

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

// Shapes
pub type ShapeRect = nphysics2d::ncollide2d::shape::Cuboid<f64>;
pub type ShapeCircle = nphysics2d::ncollide2d::shape::Ball<f64>;
pub type ShapePolyline = nphysics2d::ncollide2d::shape::Polyline<f64>;
pub type ShapeConvexPolygon = nphysics2d::ncollide2d::shape::ConvexPolygon<f64>;
pub type ShapeCompound = nphysics2d::ncollide2d::shape::Compound<f64>;
/// Aabb = Axis-aligned bounding box
pub type Aabb = nphysics2d::ncollide2d::bounding_volume::AABB<f64>;

pub trait AabbIntersection {
    /// Assuming that this Aabb is intersecting with the given Aabb, this method
    /// computes the Aabb that represents that intersection.
    fn intersected(&self, other: &Self) -> Self;
}

impl AabbIntersection for Aabb {
    fn intersected(&self, other: &Self) -> Self {
        let mins1 = self.mins();
        let mins2 = other.mins();
        let maxs1 = self.maxs();
        let maxs2 = other.maxs();

        let mins = Point2::new(
            maxs1.x.min(maxs2.x),
            maxs1.y.min(maxs2.y),
        );
        let maxs = Point2::new(
            mins1.x.max(mins2.x),
            mins1.y.max(mins2.y),
        );

        Aabb::new(mins, maxs)
    }

}

#[derive(Clone)]
pub enum Shape {
    Rect(ShapeRect),
    Circle(ShapeCircle),
    Polyline(ShapePolyline),
    ConvexPolygon(ShapeConvexPolygon),
    Compound(ShapeCompound),
}

impl Shape {
    pub fn bounds(&self) -> Aabb {
        match self {
            Shape::Rect(shape) => local_aabb(shape),
            Shape::Circle(shape) => local_aabb(shape),
            Shape::Polyline(shape) => local_aabb(shape),
            Shape::ConvexPolygon(shape) => local_aabb(shape),
            Shape::Compound(shape) => local_aabb(shape),
        }
    }

    /// Returns the center of the bounding box of this shape
    ///
    /// Note that this is a local center, the actual center of an entity needs
    /// to take into account the entity's position as well as the offset field
    /// of the collider.
    pub fn center(&self) -> Point2 {
        self.bounds().center()
    }

    pub(crate) fn to_handle(&self) -> ShapeHandle<f64> {
        use Shape::*;
        match self {
            Rect(shape) => ShapeHandle::new(shape.clone()),
            Circle(shape) => ShapeHandle::new(shape.clone()),
            Polyline(shape) => ShapeHandle::new(shape.clone()),
            ConvexPolygon(shape) => ShapeHandle::new(shape.clone()),
            Compound(shape) => ShapeHandle::new(shape.clone()),
        }
    }

    pub(crate) fn rect(size: Size) -> Self {
        let half_extents = Vec2::new(size.width as f64 / 2.0, size.height as f64 / 2.0);
        Shape::Rect(ShapeRect::new(half_extents))
    }

    pub(crate) fn from_shapes(shapes: &[(Vec2, Self)]) -> Option<Self> {
        match shapes {
            [] => None,
            [(_, shape)] => Some(shape.clone()),
            shapes => {
                let shapes = shapes.iter()
                    .map(|(pos, shape)| {
                        (Isometry::new(*pos, 0.0), shape.to_handle())
                    })
                    .collect();
                Some(Shape::Compound(ShapeCompound::new(shapes)))
            },
        }
    }
}

impl fmt::Debug for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Shape::Rect(_) => f.debug_tuple("Rect")
                .field(&"Cuboid { .. }")
                .finish(),
            Shape::Circle(_) => f.debug_tuple("Circle")
                .field(&"Ball { .. }")
                .finish(),
            Shape::Polyline(_) => f.debug_tuple("Polyline")
                .field(&"Polyline { .. }")
                .finish(),
            Shape::ConvexPolygon(_) => f.debug_tuple("ConvexPolygon")
                .field(&"ConvexPolygon { .. }")
                .finish(),
            Shape::Compound(_) => f.debug_tuple("Compound")
                .field(&"Compound { .. }")
                .finish(),
        }
    }
}
