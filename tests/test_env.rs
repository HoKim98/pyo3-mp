use std::env;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use pyo3_mp::Process;

/// A Python function implemented in Rust.
#[pyfunction]
fn foo(_py: Python) -> PyResult<()> {
    // This may be applied on each process!
    let i = env::var("PYO3_MP_INDEX").unwrap();
    println!("hello, index {}!", i);
    Ok(())
}

/// Converts the pyfunction into python object.
fn build_foo(py: Python) -> PyResult<Py<PyAny>> {
    Ok(wrap_pyfunction!(foo)(py)?.into_py(py))
}

#[test]
fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        // Let's get a sample python function.
        let f = build_foo(py)?;

        let mut mp = Process::new(py)?;

        // Spawn 10 processes.
        for i in 0..10 {
            // The arguments can be passed by environment variables.
            env::set_var("PYO3_MP_INDEX", format!("{}", i));
            mp.spawn(&f, (), None)?;
        }

        mp.join()
    })
}
