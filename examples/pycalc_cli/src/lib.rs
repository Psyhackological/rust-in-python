/*
Calculator function to later import into a Python Fire CLI.
*/

use pyo3::exceptions::PyZeroDivisionError;
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: f64, b: f64) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// Formats adding two numbers as string
#[pyfunction]
fn add_as_string(a: f64, b: f64) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// Formats subtracting two numbers as string
#[pyfunction]
fn subtract_as_string(a: f64, b: f64) -> PyResult<String> {
    Ok((a - b).to_string())
}

/// Formats dividing two numbers as string
#[pyfunction]
fn divide_as_string(a: f64, b: f64) -> PyResult<String> {
    match b {
        0.0 => Err(PyZeroDivisionError::new_err("PyZeroDivisionError!")),
        _ => Ok((a / b).to_string()),
    }
}

/// Formats multiplying two numbers as string
#[pyfunction]
fn multiply_as_string(a: f64, b: f64) -> PyResult<String> {
    Ok((a * b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn libpycalc_cli(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(add_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(subtract_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(divide_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(multiply_as_string, m)?)?;
    Ok(())
}
