use std::collections::HashMap;
use std::ffi::CString;
use std::fmt::Display;

use pyo3::Bound;
use pyo3::Py;
use pyo3::PyResult;
use pyo3::pyclass;
use pyo3::pyfunction;
use pyo3::pymethods;
use pyo3::types::PyAnyMethods;
use pyo3::types::PyDict;
use pyo3::types::PyModuleMethods;
use pyo3::{Python, types::PyModule, wrap_pyfunction};
use rand::Rng;
use rand::distr::Alphanumeric;
use rand::distr::SampleString;
use rand::rng;
/// Define the Person struct as a Python class.
#[pyclass(dict, eq, str)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Person {
    /// The person's name.
    #[pyo3(get)]
    pub name: String,
    /// The person's age.
    #[pyo3(get)]
    pub age: u32,

    #[pyo3(get, set)]
    pub children: Vec<Person>,
}

impl Person {
    fn to_dict_with_py<'a>(&'a self, py: Python<'a>) -> PyResult<Bound<'a, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item("name", self.name.clone())?;
        dict.set_item("age", self.age.to_string())?;
        dict.set_item(
            "children",
            self.children
                .iter()
                .map(|c| c.to_dict_with_py(py))
                .collect::<PyResult<Vec<_>>>()?,
        )?;

        //dbg!(&dict);
        Ok(dict)
    }
}
#[pymethods]
impl Person {
    fn add_child(&mut self, child: Person) {
        self.children.push(child);
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self)
    }

    fn to_dict(&self) -> PyResult<Py<PyDict>> {
        Python::with_gil(|py| {
            let a = self.to_dict_with_py(py)?.into();
            Ok(a)
        })
    }
}

impl Display for Person {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Person(name={}, age={}, children={:?})",
            self.name, self.age, self.children
        )
    }
}

/// Create a Person with a random name and age
#[pyfunction]
pub fn create_random_person() -> Person {
    //let mut rng = rng();

    // Generate a random name (3-10 characters)
    //let name_length = rng.random_range(3..=10);
    //let name = Alphanumeric.sample_string(&mut rng, name_length);

    // Generate a random age (1-99)
    //let age = rng.random_range(1..=99);

    Person {
        name: "John".to_string(),
        age: 30,
        children: vec![],
    }
}

/// Create a deeply nested Person structure with random children
///
/// # Arguments
/// * `depth` - Maximum depth of the person hierarchy
/// * `max_children` - Maximum number of children at each level
#[pyfunction]
pub fn create_nested_person(depth: usize, max_children: usize) -> Person {
    let mut root = create_random_person();

    fn add_children(person: &mut Person, current_depth: usize, max_children: usize) {
        if current_depth == 0 {
            return;
        }

        // Add random number of children (1 to max_children)
        let mut rng = rng();
        let num_children = rng.random_range(1..=max_children);

        for _ in 0..num_children {
            let mut child = create_random_person();
            add_children(&mut child, current_depth - 1, max_children);
            person.add_child(child);
        }
    }

    add_children(&mut root, depth, max_children);
    root
}

/// A pyfunction that acts as a constructor for Person.
#[pyfunction]
pub fn new_person(name: String, age: u32) -> Person {
    Person {
        name,
        age,
        children: vec![],
    }
}

fn main() -> anyhow::Result<()> {
    let python_file_path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/python/lib.py");

    let code = CString::new(std::fs::read_to_string(python_file_path)?)?;
    let file_name = CString::new("lib.py")?;
    let module_name = CString::new("my_lib")?;

    Python::with_gil(|py| {
        let my_module = PyModule::new(py, "py03_pydantic_ormsgpack_experiment")?;

        my_module.add_class::<Person>()?;
        my_module.add_function(wrap_pyfunction!(new_person, &my_module)?)?;
        my_module.add_function(wrap_pyfunction!(create_random_person, &my_module)?)?;
        my_module.add_function(wrap_pyfunction!(create_nested_person, &my_module)?)?;

        dbg!(&my_module);

        // Import and get sys.modules
        let sys = PyModule::import(py, "sys")?;
        let py_modules: Bound<'_, PyDict> = sys.getattr("modules")?.downcast_into().unwrap();

        // Insert foo into sys.modules
        py_modules.set_item("py03_pydantic_ormsgpack_experiment", my_module)?;

        let py_module = PyModule::from_code(
            py,
            code.as_c_str(),
            file_name.as_c_str(),
            module_name.as_c_str(),
        )?;

        dbg!(&py_module);

        py_module.getattr("main")?.call0()?;

        Ok::<(), anyhow::Error>(())
    })?;

    Ok(())
}
