use pyo3::prelude::*;
use pyo3::pyclass;
use pyo3::wrap_pyfunction;

// Notice the pyclass macro
#[pyclass]
struct NumberList {
    numbers: Vec<i32>,
}

impl NumberList {
    fn new() -> Self {
        Self {
            numbers: Vec::new(),
        }
    }

    fn add_number(&mut self, num: i32) {
        self.numbers.push(num);
    }

    fn len(&self) -> usize {
        self.numbers.len()
    }

    fn clear(&mut self) {
        self.numbers.clear();
    }
}

#[pymodule]
fn rust_list(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<NumberList>()?;
    m.add_wrapped(wrap_pyfunction!(add_number))?;
    m.add_wrapped(wrap_pyfunction!(len))?;
    m.add_wrapped(wrap_pyfunction!(clear))?;
    Ok(())
}

#[pyfunction]
fn add_number(list: &mut NumberList, num: i32) -> PyResult<()> {
    list.add_number(num);
    Ok(())
}

#[pyfunction]
fn len(list: &mut NumberList) -> PyResult<usize> {
    Ok(list.len())
}

#[pyfunction]
fn clear(list: &mut NumberList) -> PyResult<()> {
    list.clear();
    Ok(())
}

#[pymethods]
impl NumberList {
    #[new]
    fn new_obj() -> Self {
        Self::new()
    }

    fn add(&mut self, value: i32) {
        self.add_number(value);
    }

    fn length(&self) -> usize {
        self.len()
    }

    fn clear_list(&mut self) {
        self.clear()
    }
}

#[pymodule]
fn libownership_pyrust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<NumberList>()?;
    Ok(())
}
