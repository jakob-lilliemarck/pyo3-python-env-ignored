use pyo3::prelude::*;
use std::env;

pub fn ping() {
    Python::with_gil(|py| {
        let sys = PyModule::import(py, "sys").unwrap();

        let py_path: String = sys.getattr("executable").unwrap().extract().unwrap();
        let py_version: String = sys.getattr("version").unwrap().extract().unwrap();

        println!("executable: {}\nversion: {}", py_path, py_version);
    });
}

fn main() {
    dotenv::dotenv().ok();

    let venv_bin_relative_path = env::var("VENV_BIN").expect("VENV_BIN relative path must be set");

    let current_dir = env::current_dir().expect("Failed to get current directory");
    
    let venv_bin_path = current_dir.join(venv_bin_relative_path);

    let venv_bin_path_str = venv_bin_path.to_str().expect("Failed to convert path to string");

    let current_path = env::var("PATH").unwrap_or_else(|_| String::new());

    let new_path = format!("{}:{}", venv_bin_path_str, current_path);

    unsafe {
        env::set_var("PATH", &new_path);
    }

    ping();
}
