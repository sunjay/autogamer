use autogamer::foo;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pymodule]
fn autogamer_bindings(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(run_foo))?;

    Ok(())
}

#[pyfunction]
/// Runs the foo function
pub fn run_foo() {
    foo()
}
