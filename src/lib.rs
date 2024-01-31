use crate::_pareto_sort::pareto_sort_rs;
use pyo3::prelude::*;
mod _pareto_sort;

#[pyfunction]
fn pareto_sort(points: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    pareto_sort_rs(&points)
}

#[pymodule]
fn predictables_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pareto_sort, m)?)?;
    Ok(())
}

#[cfg(test)]

mod tests {

    #[test]
    fn test_pareto_sort() {
        let points = vec![
            vec![1.0, 2.0, 3.0],
            vec![1.0, 2.0, 4.0],
            vec![1.0, 2.0, 5.0],
            vec![1.0, 2.0, 6.0],
            vec![1.0, 2.0, 7.0],
            vec![1.0, 2.0, 8.0],
            vec![1.0, 2.0, 9.0],
            vec![1.0, 2.0, 10.0],
            vec![1.0, 2.0, 11.0],
            vec![1.0, 2.0, 12.0],
            vec![1.0, 2.0, 13.0],
            vec![1.0, 2.0, 14.0],
            vec![1.0, 2.0, 15.0],
            vec![1.0, 2.0, 16.0],
            vec![1.0, 2.0, 17.0],
            vec![1.0, 2.0, 18.0],
            vec![1.0, 2.0, 19.0],
            vec![1.0, 2.0, 20.0],
            vec![1.0, 2.0, 21.0],
            vec![1.0, 2.0, 22.0],
            vec![1.0, 2.0, 23.0],
            vec![1.0, 2.0, 24.0],
            vec![1.0, 2.0, 25.0],
            vec![1.0, 2.0, 26.0],
            vec![1.0, 2.0, 27.0],
            vec![1.0, 2.0, 28.0],
            vec![1.0, 2.0, 29.0],
        ];
        let result = super::pareto_sort_rs(&points);
        assert_eq!(result.len(), 1);

        assert_eq!(result[0], vec![1.0, 2.0, 29.0]);
    }
}
