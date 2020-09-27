use pyo3::prelude::*;

use pyo3_mp::Process;

#[pyclass]
struct Integer {
    value: usize,
}

#[pymethods]
impl Integer {
    #[new]
    fn new(value: usize) -> Self {
        Self { value }
    }

    fn add_and_print(&mut self, value: usize) {
        println!("{} + {} = {}", self.value, value, self.value + value)
    }
}

#[test]
fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        let mut mp = Process::new(py)?;

        let object = Integer::new(42).into_py(py);

        // Spawn 10 processes.
        for i in 0..10 {
            let f = object.getattr(py, "add_and_print")?;
            mp.spawn(f, (i,), None)?;
        }

        mp.join()
    })
}
