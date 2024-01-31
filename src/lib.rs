use crate::_pareto_sort::pareto_sort_rs;
use pyo3::prelude::*;
mod _pareto_sort;

#[pyfunction]
fn pareto_sort(points: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    pareto_sort_rs(points)
}

#[pymodule]
fn predictables_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pareto_sort, m)?)?;
    Ok(())
}
