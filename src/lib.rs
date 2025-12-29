use pyo3::prelude::*;
use pyo3::types::PyList;
use rand::prelude::*;
use rand::distributions::Uniform;
use rayon::prelude::*;

thread_local! {
    static RNG: std::cell::RefCell<StdRng> = std::cell::RefCell::new(StdRng::from_entropy());
}

#[pyfunction]
fn set_seed(seed: u64) {
    RNG.with(|rng| {
        *rng.borrow_mut() = StdRng::seed_from_u64(seed);
    });
}

#[pyfunction]
fn randfloat() -> f64 {
    RNG.with(|rng| rng.borrow_mut().gen())
}

#[pyfunction]
fn randint(min: i32, max: i32) -> i32 {
    RNG.with(|rng| rng.borrow_mut().gen_range(min..=max))
}

#[pyfunction]
fn uniform(min: f64, max: f64) -> f64 {
    RNG.with(|rng| rng.borrow_mut().sample(Uniform::new(min, max)))
}

#[pyfunction]
fn choice(py: Python, lst: &PyList) -> PyResult<PyObject> {
    let idx = RNG.with(|rng| rng.borrow_mut().gen_range(0..lst.len()));
    let item = lst.get_item(idx)?;  // unwrap PyResult<&PyAny>
    Ok(item.into_py(py))            // convert &PyAny -> PyObject
}

#[pyfunction]
fn sample(py: Python, lst: &PyList, k: usize) -> PyResult<Vec<PyObject>> {
    let mut indices: Vec<usize> = (0..lst.len()).collect();
    RNG.with(|rng| indices.shuffle(&mut *rng.borrow_mut()));
    indices
        .into_iter()
        .take(k)
        .map(|i| {
            let item = lst.get_item(i)?;  // unwrap PyResult<&PyAny>
            Ok(item.into_py(py))          // convert to PyObject
        })
        .collect()
}

#[pyfunction]
fn shuffle(py: Python, lst: &PyList) {
    let mut items: Vec<PyObject> = lst.iter().map(|x| x.into_py(py)).collect();
    RNG.with(|rng| items.shuffle(&mut *rng.borrow_mut()));
    for (i, val) in items.into_iter().enumerate() {
        lst.set_item(i, val).unwrap();
    }
}

#[pyfunction]
fn randint_array(min: i32, max: i32, size: usize) -> Vec<i32> {
    (0..size).into_par_iter().map(|_| thread_rng().gen_range(min..=max)).collect()
}

#[pyfunction]
fn random_array(size: usize) -> Vec<f64> {
    (0..size).into_par_iter().map(|_| thread_rng().gen::<f64>()).collect()
}

#[pymodule]
fn pyrand(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(set_seed, m)?)?;
    m.add_function(wrap_pyfunction!(randfloat, m)?)?;
    m.add_function(wrap_pyfunction!(randint, m)?)?;
    m.add_function(wrap_pyfunction!(uniform, m)?)?;
    m.add_function(wrap_pyfunction!(choice, m)?)?;
    m.add_function(wrap_pyfunction!(sample, m)?)?;
    m.add_function(wrap_pyfunction!(shuffle, m)?)?;
    m.add_function(wrap_pyfunction!(randint_array, m)?)?;
    m.add_function(wrap_pyfunction!(random_array, m)?)?;
    Ok(())
}
