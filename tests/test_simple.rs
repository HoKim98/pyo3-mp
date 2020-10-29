use std::thread::sleep;
use std::time::Duration;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use pyo3_mp::Process;

/// A Python function implemented in Rust.
#[pyfunction]
fn foo(_py: Python, i: usize) -> PyResult<()> {
    println!("hello, number {}!", i);
    // This may be worked on each process!
    sleep(Duration::from_secs(1));
    println!("goodbye, number {}!", i);
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
        assert_eq!(mp.is_running(), false);

        // Spawn 10 processes.
        for i in 0..10 {
            mp.spawn(&f, (i,), None)?;
        }
        assert_eq!(mp.is_running(), true);

        mp.join()?;
        assert_eq!(mp.is_running(), false);

        Ok(())
    })
}
