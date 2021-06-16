use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crossbeam_channel::unbounded;
use std::thread;



fn runworkers(
    size: usize,
    results_tx: crossbeam_channel::Sender<Vec<u8>>,
) -> Vec<std::thread::JoinHandle<()>> {
    let mut children = vec![];
    for _ in 0..3 {
        let results = results_tx.clone();
        children.push(thread::spawn(move || {
            let mut big_vec: Vec<u8> = Vec::with_capacity(size);

            for _ in 0..size {
                big_vec.push(0);
            }

            results.send(big_vec).expect("send result");
        }));
    }
    children
}
#[pyfunction]
fn proba(py: Python, size: usize) -> Vec<Vec<u8>> {
    let (results_tx, results_rx) = unbounded();
    py.allow_threads(move || {
        let children = runworkers(size, results_tx);
        for child in children {
            let _ = child.join();
        }

        let results: Vec<Vec<u8>> = results_rx.iter().collect();
        results
    })
}

#[pymodule]
fn py_wrapper(py: Python, m: &PyModule) -> PyResult<()> {
    
    m.add_wrapped(wrap_pyfunction!(proba))?;

    Ok(())
}
