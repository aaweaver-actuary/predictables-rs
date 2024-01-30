use pyo3::prelude::*;
use rayon::prelude::*;

fn is_dominated(a: &[f64], b: &[f64]) -> bool {
    let a_ge_b = a.iter().zip(b.iter()).all(|(ai, bi)| ai >= bi);
    let a_gt_b = a.iter().zip(b.iter()).any(|(ai, bi)| ai > bi);
    a_ge_b && a_gt_b
}

fn pareto_sort_rs(points: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    points
        .par_iter()
        .filter(|a| !points.par_iter().any(|b| is_dominated(a, b)))
        .cloned()
        .collect()
}

#[pyfunction]
fn pareto_sort_py(points: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    pareto_sort_rs(points)
}

#[pymodule]
fn pareto_sort(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pareto_sort_py, m)?)?;
    Ok(())
}
