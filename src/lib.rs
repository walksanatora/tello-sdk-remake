use pyo3::prelude::*;
mod tello;

/// A Python module implemented in Rust.
#[pymodule]
fn tello_sdk(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<tello::State>()?;
    m.add_class::<tello::Tello>()?;
    m.add_class::<tello::Flip>()?;
    Ok(())
}