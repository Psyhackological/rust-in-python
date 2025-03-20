use pyo3::prelude::*;

/// Calculates an approximation of Pi using the Leibniz formula.
/// The more iterations, the more accurate the results.
#[pyfunction]
fn calculate_pi(iterations: usize) -> PyResult<f64> {
    let mut pi = 0.;
    for k in 0..iterations {
        pi += ((-1.0_f64).powi(k as i32) / (2 * k + 1) as f64) * 4.0;
    }
    Ok(pi)
}

/// A Python module implemented in Rust.
#[pymodule]
fn digits_pi(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_pi, m)?)?;
    Ok(())
}
