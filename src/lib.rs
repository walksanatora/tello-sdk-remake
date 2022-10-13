
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
mod tello;
pub use tello::*;


pyo3::create_exception!(tello_sdk,TelloErr,PyException);

/// A Python module implemented in Rust.
#[pymodule]
fn tello_sdk(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<tello::State>()?;
    m.add_class::<tello::Tello>()?;
    m.add_class::<tello::Flip>()?;
    m.add("TelloErr",py.get_type::<TelloErr>())?;
    Ok(())
}