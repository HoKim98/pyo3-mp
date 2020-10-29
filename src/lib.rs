use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyDict, PyTuple};

pub struct Process<'a> {
    py: Python<'a>,
    py_process: &'a PyAny,

    pool: Vec<&'a PyAny>,
}

impl<'a> Process<'a> {
    pub fn new(py: Python<'a>) -> PyResult<Self> {
        let multiprocessing = py.import("multiprocessing")?;
        let py_process = multiprocessing.get("Process")?;

        Ok(Self {
            py,
            py_process,
            pool: vec![],
        })
    }

    pub fn is_running(&self) -> bool {
        !self.pool.is_empty()
    }

    pub fn spawn(
        &mut self,
        f: impl IntoPy<Py<PyAny>>,
        args: impl IntoPy<Py<PyTuple>>,
        kwargs: Option<&PyDict>,
    ) -> PyResult<&PyAny> {
        let f = f.into_py(self.py);
        let f_args = args.into_py(self.py).into_py(self.py);
        let f_kwargs = kwargs
            .or_else(|| Some(PyDict::new(self.py)))
            .into_py(self.py);

        let kwargs = [("target", f), ("args", f_args), ("kwargs", f_kwargs)].into_py_dict(self.py);

        let p = self.py_process.call((), Some(kwargs))?;
        p.call_method0("start")?;
        self.pool.push(p);
        Ok(p)
    }

    pub fn join(&mut self) -> PyResult<()> {
        for p in &self.pool {
            p.call_method0("join")?;
        }
        self.pool.clear();
        Ok(())
    }
}

impl<'a> Drop for Process<'a> {
    fn drop(&mut self) {
        self.join().unwrap();
    }
}
