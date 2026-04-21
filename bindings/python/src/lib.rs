use pyo3::{exceptions::PyRuntimeError, prelude::*};

fn map_core_error(error: nextral::CoreError) -> PyErr {
    PyRuntimeError::new_err(error.to_string())
}

#[pyfunction]
fn lexical_score(text: String, query: String) -> PyResult<f32> {
    nextral::scoring::try_lexical_score(&text, &query).map_err(map_core_error)
}

#[pymodule]
fn _nextral(_py: Python<'_>, module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(lexical_score, module)?)?;
    Ok(())
}
