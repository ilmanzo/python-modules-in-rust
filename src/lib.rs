use pyo3::prelude::*;

#[allow(dead_code)]
#[pymodule]
fn librusty(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(list_prod))?;
    Ok(())
}

/// calc the product of N numbers in a list.
#[pyfunction]
fn list_prod(a: Vec<isize>) -> PyResult<isize> {
    let mut prod: isize = 1;
    for i in a {
        prod *= i;
    }
    Ok(prod)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_product() {
        let input = vec![1, 2, 3, 4];
        let result = list_prod(input).unwrap_or(0);
        assert_eq!(result, 24);
    }
}
