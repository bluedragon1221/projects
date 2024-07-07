use inquire::{self, Select};
use std::collections::HashMap;

use pyo3::{prelude::*, types::PyType};

// --- Object
#[pyclass]
#[derive(Clone)]
pub enum Object {
    Weapon { name: String },
    Item { name: String },
}

#[pymethods]
impl Object {
    fn get_name(&self) -> &str {
        match self {
            Object::Weapon { name: a } => a,
            Object::Item { name: a } => a,
        }
    }

    #[classmethod]
    fn new_weapon(_cls: &Bound<'_, PyType>, name: &str) -> Self {
        Object::Weapon { name: name.into() }
    }

    #[classmethod]
    fn new_item(_cls: &Bound<'_, PyType>, name: &str) -> Self {
        Object::Item { name: name.into() }
    }
}

// --- Container
#[pyclass]
#[derive(Clone)]
pub struct Container {
    name: String,
    items: Vec<Object>,
}

#[pyclass]
struct ContainerIter {
    inner: std::vec::IntoIter<Object>,
}

#[pymethods]
impl ContainerIter {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<Object> {
        slf.inner.next()
    }
}

#[pymethods]
impl Container {
    #[new]
    fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            items: Vec::new(),
        }
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyResult<Py<ContainerIter>> {
        let iter = ContainerIter {
            inner: slf.items.clone().into_iter(),
        };

        Py::new(slf.py(), iter)
    }

    // fn txt_list(&self) -> Vec<&str> {
    //     self.items.iter().map(|x| x.get_name()).collect()
    // }

    // fn find(&self, item_name: &str) -> Option<&Item> {
    //     self.items.iter().filter(|x| x.get_name() == item_name).nth(0)
    // }

    fn remove(&mut self, item_name: &str) {
        self.items.retain(|x| x.get_name() != item_name)
    }

    fn add(&mut self, item: Object) {
        self.items.push(item)
    }

    fn extend(&mut self, items: Vec<Object>) {
        for i in items {
            self.add(i)
        }
    }
}

// --- Player
#[pyclass]
struct Player {
    #[pyo3(get, set)]
    name: String,
    inventory: Container,
    #[pyo3(get, set)]
    gold: i32,
}

#[pymethods]
impl Player {
    #[new]
    fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            gold: 0,
            inventory: Container::new("inventory"),
        }
    }

    fn get_inventory(&self) -> Container {
        self.inventory.clone()
    }

    fn give_item(&mut self, item: Object) {
        self.inventory.add(item)
    }

    fn remove_item(&mut self, item: &str) {
        self.inventory.remove(item)
    }

    fn give_gold(&mut self, amount: i32) {
        self.gold += amount
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn ad(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Object>()?;
    m.add_class::<Container>()?;
    m.add_class::<Player>()?;
    Ok(())
}
