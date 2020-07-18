use autogamer as ag;
use pyo3::prelude::*;

pub fn add_geometry_mod(_py: Python, pymod: &PyModule) -> PyResult<()> {
    pymod.add_class::<Shape>()?;
    pymod.add_class::<ShapeRect>()?;

    Ok(())
}

#[pyclass(subclass)]
#[derive(Debug, Default, Clone)]
pub struct Shape {
}

impl Shape {
    pub fn to_shape(shape: &PyAny) -> Option<ag::Shape> {
        if let Ok(shape) = shape.downcast::<PyCell<ShapeRect>>() {
            return Some(shape.borrow().to_shape());
        }

        None
    }
}

#[pyclass(extends=Shape)]
#[derive(Debug, Clone)]
pub struct ShapeRect {
    #[pyo3(get, set)]
    pub width: f64,
    #[pyo3(get, set)]
    pub height: f64,
}

impl ShapeRect {
    pub fn to_shape(&self) -> ag::Shape {
        let &Self {width, height} = self;
        let half_extents = ag::Vec2::new(width/2.0, height/2.0);
        ag::Shape::Rect(ag::ShapeRect::new(half_extents))
    }
}

#[pymethods]
impl ShapeRect {
    #[new]
    //TODO(PyO3/pyo3#1025): These should be keyword-only arguments with no defaults
    #[args("*", width="0.0", height="0.0")]
    pub fn new(width: f64, height: f64) -> (Self, Shape) {
        (Self {width, height}, Shape::default())
    }
}
