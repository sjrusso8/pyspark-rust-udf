use pyo3::prelude::*;
use statrs::distribution::{ContinuousCDF, Normal};
use statrs::statistics::Statistics;


/// Adds one and returns a float.
#[pyfunction]
fn plus_one_rs(a: f64) -> PyResult<f64> {
    Ok(a + 1.0)
}

/// Returns the CDF of the standard normal distribution.
#[pyfunction]
fn cdf_rs(a: f64) -> PyResult<f64> {
    Ok(Normal::new(0.0, 1.0).unwrap().cdf(a))
}

/// Subtract mean
#[pyfunction]
fn subtract_mean_rs(a: Vec<f64>) -> PyResult<Vec<f64>> {
    let mean = a.clone().mean();
    Ok(a.iter().map(|x| *x - mean).collect())
}


/// A Python module implemented in Rust.
#[pymodule]
fn rust_udf(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(plus_one_rs, m)?)?;
    m.add_function(wrap_pyfunction!(cdf_rs, m)?)?;
    m.add_function(wrap_pyfunction!(subtract_mean_rs, m)?)?;
    Ok(())
}