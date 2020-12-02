use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use pyo3::wrap_pyfunction;

use bincode::{deserialize, serialize, serialize_into};
use serde::{Deserialize, Serialize};

use pyo3_mp::Process;

#[pyclass]
#[derive(Serialize, Deserialize)]
struct Integer {
    #[pyo3(get)]
    value: usize,
}

#[pymethods]
impl Integer {
    #[new]
    fn new(value: usize) -> Self {
        Self { value }
    }

    fn increse(&mut self) {
        self.value += 1;
    }

    #[classmethod]
    unsafe fn load(source: &PyAny) -> PyResult<Self> {
        let source: &PyByteArray = source.extract()?;
        Ok(deserialize(source.as_bytes()).unwrap())
    }

    fn store<'a>(&self, py: Python<'a>) -> &'a PyByteArray {
        PyByteArray::new(py, &serialize(self).unwrap())
    }

    unsafe fn store_into(&self, target: &PyAny) -> PyResult<()> {
        let target: &PyByteArray = target.extract()?;
        serialize_into(target.as_bytes_mut(), self).unwrap();
        Ok(())
    }
}

#[pyfunction]
unsafe fn increase_integer(py: Python, args: &PyAny, _kwargs: &PyAny) -> PyResult<()> {
    let mut integer = Integer::load(args.get_item(0)?)?;
    integer.increse();
    args.set_item(0, integer.store(py))
}

/// Converts the pyfunction into python object.
fn build_fn(py: Python) -> PyResult<Py<PyAny>> {
    Ok(wrap_pyfunction!(increase_integer)(py)?.into_py(py))
}

#[test]
fn main() -> Result<(), ()> {
    Python::with_gil(|py| {
        let mut mp = Process::new(py)?;

        let object = Integer::new(42);
        let object = object.store(py);

        // Spawn a process.
        let f = build_fn(py)?;
        let (args, _kwargs) = mp.spawn_mut(f, (object,), None)?;

        mp.join()?;

        let object = unsafe { Integer::load(args.as_ref(py).get_item(0)?)? };
        assert_eq!(object.value, 43);

        Ok(())
    })
    .map_err(|e: PyErr| Python::with_gil(|py| e.print_and_set_sys_last_vars(py)))
}
