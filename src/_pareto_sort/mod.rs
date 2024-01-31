use rayon::prelude::*;
use std::cmp::Ordering;

/// Returns true if a is dominated by b
/// a is dominated by b if all elements of a are greater than or equal to the corresponding elements of b
/// and at least one element of a is strictly greater than the corresponding element of b
/// This is used to sort multi-objective optimization results, using a pareto front
///
/// # Arguments
///
/// * `a` - A slice of f64
/// * `b` - A slice of f64
///
/// # Example
///
/// ```rust
/// let a = vec![1.0, 2.0, 3.0];
/// let b = vec![1.0, 2.0, 4.0];
/// let result = is_dominated(a.as_slice(), b.as_slice());
/// assert_eq!(result, false);
/// ```
///
/// ```rust
/// let a = vec![1.0, 2.0, 4.0];
/// let b = vec![1.0, 2.0, 3.0];
/// let result = is_dominated(a.as_slice(), b.as_slice());
/// assert_eq!(result, true);
/// ```
/// # Returns
///
/// A boolean value representing whether a is dominated by b
///
/// # Panics
///
/// This function will panic if the length of a and b are not equal
///
/// # Safety
///
/// This function is safe to use
///
/// # Performance
///
/// This function is very fast and can be used in performance critical applications
///
/// # Note
///
/// This function is used to sort multi-objective optimization results
/// according to a pareto-optimal front
///
fn is_dominated(a: &[f64], b: &[f64]) -> bool {
    let a_ge_b: bool = a.iter().zip(b.iter()).all(|(ai, bi)| ai >= bi);
    let a_gt_b: bool = a.iter().zip(b.iter()).any(|(ai, bi)| ai > bi);
    a_ge_b && a_gt_b
}

/// Returns the ordering of a and b according to the domination order
/// a is greater than b if a is dominated by b
/// a is less than b if b is dominated by a
/// a is equal to b if neither a nor b dominates the other
/// This is used to sort multi-objective optimization results, using a pareto front
///
/// # Arguments
///
/// * `a` - A slice of f64
/// * `b` - A slice of f64
///
/// # Example
///
/// ```rust
/// let a = vec![1.0, 2.0, 3.0];
/// let b = vec![1.0, 2.0, 4.0];
/// let result = domination_order(a.as_slice(), b.as_slice());
/// println!("{}", result);
/// ```
///
/// > Less
///
/// # Returns
///
/// An Ordering enum value representing the domination order of a and b
///
/// # Panics
///
/// This function will panic if the length of a and b are not equal
///
/// # Safety
///
/// This function is safe to use
///
/// # Performance
///
/// This function is very fast and can be used in performance critical applications
///
/// # Note
///
/// This function is used to sort multi-objective optimization results
/// according to a pareto-optimal front
fn domination_order(a: &[f64], b: &[f64]) -> Ordering {
    if is_dominated(a, b) {
        Ordering::Greater
    } else if is_dominated(b, a) {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

/// Returns the pareto-optimal ordering of a set of points
/// This is used to sort multi-objective optimization results,
/// using a pareto-optimal condition
///
/// # Arguments
///
/// * `points` - A vector of vectors of f64. Each vector will be a point in the multi-objective optimization space.
///
/// # Example
///
/// ```rust
/// let points = vec![
///    vec![1.0, 2.0, 3.0],
///    vec![1.0, 3.0, 4.0],
///    vec![1.0, 3.0, 5.0]
/// ];
/// let result = pareto_sort_rs(points);
/// let expected = vec![
///   vec![1.0, 3.0, 5.0],
///   vec![1.0, 3.0, 4.0],
///   vec![1.0, 2.0, 3.0]
/// ];
/// assert_eq!(result, expected);
/// ```
///
/// # Returns
///
/// A vector of vectors of f64. Each vector will be a point in the multi-objective optimization space,
/// sorted according to the pareto-optimal condition
///
/// # Panics
///
/// This function will panic if the length of any of the vectors in points are not equal
///
/// # Safety
///
/// This function is safe to use
///
/// # Performance
///
/// This function is very fast and can be used in performance critical applications
///
pub fn pareto_sort_rs(points: &[&[f64]]) -> Vec<Vec<f64>> {
    let mut pareto_front: Vec<Vec<f64>> = Vec::new();
    for point in points.iter() {
        let mut dominated = false;
        pareto_front.retain(|p| match domination_order(*p, *point) {
            Ordering::Less => {
                dominated = true;
                false
            }
            Ordering::Greater => true,
            Ordering::Equal => {
                dominated = true;
                false
            }
        });
        if !dominated {
            pareto_front.push(point.to_vec());
        }
    }
    pareto_front
}

#[cfg(test)]

mod tests {

    #[test]
    fn test_is_dominated_eq_false() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![1.0, 2.0, 4.0];
        let result = super::is_dominated(a.as_slice(), b.as_slice());
        assert_eq!(result, false);
    }

    #[test]
    fn test_is_dominated_eq_true() {
        let a = vec![1.0, 2.0, 4.0];
        let b = vec![1.0, 2.0, 3.0];
        let result = super::is_dominated(a.as_slice(), b.as_slice());
        assert_eq!(result, true);
    }

    #[test]
    fn test_domination_order_eq_less() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![1.0, 2.0, 4.0];
        let result = super::domination_order(a.as_slice(), b.as_slice());
        assert_eq!(result, std::cmp::Ordering::Less);
    }

    #[test]
    fn test_domination_order_eq_greater() {
        let a = vec![1.0, 2.0, 4.0];
        let b = vec![1.0, 2.0, 3.0];
        let result = super::domination_order(a.as_slice(), b.as_slice());
        assert_eq!(result, std::cmp::Ordering::Greater);
    }

    #[test]
    fn test_domination_order_eq_equal_because_actually_equal() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![1.0, 2.0, 3.0];
        let result = super::domination_order(a.as_slice(), b.as_slice());
        assert_eq!(result, std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_domination_order_eq_equal_because_no_pareto_optimal_front() {
        let a = vec![1.0, 3.0, 2.0];
        let b = vec![1.0, 2.0, 3.0];
        let result = super::domination_order(a.as_slice(), b.as_slice());
        assert_eq!(result, std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_pareto_sort() {
        let points = vec![
            vec![1.0, 1.0, 3.0],
            vec![1.0, 1.0, 4.0],
            vec![1.0, 1.0, 5.0],
            vec![1.0, 1.0, 6.0],
            vec![1.0, 1.0, 7.0],
            vec![1.0, 1.0, 8.0],
            vec![1.0, 2.0, 8.0],
            vec![1.0, 2.0, 9.0],
            vec![1.0, 2.0, 10.0],
        ];
        let result = super::pareto_sort_rs(&points);
        assert_eq!(result.len(), points.len());

        assert_eq!(result[0], vec![1.0, 2.0, 10.0]);
    }
}
