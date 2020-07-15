use nphysics2d;

pub type Vec2 = nphysics2d::math::Vector<f64>;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}
