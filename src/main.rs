use pyo3::prelude::*;

pub fn ping() {
    Python::with_gil(|py| {
        let sys = PyModule::import(py, "sys").unwrap();

        let py_path: String = sys.getattr("executable").unwrap().extract().unwrap();
        let py_version: String = sys.getattr("version").unwrap().extract().unwrap();

        println!("path: {}\nversion: {}", py_path, py_version);
    });
}

fn main() {
    ping();
}
