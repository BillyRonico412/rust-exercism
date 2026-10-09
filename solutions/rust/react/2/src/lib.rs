use std::collections::{HashMap, HashSet};

use uuid::Uuid;

/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellId(uuid::Uuid);

impl InputCellId {
    fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellId(uuid::Uuid);

impl ComputeCellId {
    fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CallbackId(uuid::Uuid);

impl CallbackId {
    fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

struct InputCell<T> {
    value: T,
    conpute_consumers: HashSet<ComputeCellId>,
}

struct ComputeCell<T> {
    dependencies: Vec<CellId>,
    compute_func: fn(&[T]) -> T,
    value: T,
    compute_consumers: HashSet<ComputeCellId>,
    callback_consumers: HashSet<CallbackId>,
}

struct Callback<'a, T> {
    dependency: ComputeCellId,
    func: Box<dyn FnMut(T) + 'a>,
    last_value: T,
}

pub struct Reactor<'a, T> {
    input_cells: HashMap<InputCellId, InputCell<T>>,
    compute_cells: HashMap<ComputeCellId, ComputeCell<T>>,
    callbacks: HashMap<CallbackId, Callback<'a, T>>,
}

impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Self {
            input_cells: HashMap::new(),
            compute_cells: HashMap::new(),
            callbacks: HashMap::new(),
        }
    }

    pub fn create_input(&mut self, initial: T) -> InputCellId {
        let input_cell_id = InputCellId::new();
        self.input_cells.insert(
            input_cell_id,
            InputCell {
                conpute_consumers: HashSet::new(),
                value: initial,
            },
        );
        input_cell_id
    }

    pub fn create_compute(
        &mut self,
        dependencies: &[CellId],
        compute_func: fn(&[T]) -> T,
    ) -> Result<ComputeCellId, CellId> {
        let compute_cell_id = ComputeCellId::new();
        let value = self.compute(dependencies, compute_func)?;

        dependencies.iter().for_each(|id| {
            match id {
                CellId::Input(id) => {
                    self.input_cells.entry(*id).and_modify(|input_cell| {
                        input_cell.conpute_consumers.insert(compute_cell_id);
                    });
                }
                CellId::Compute(id) => {
                    self.compute_cells.entry(*id).and_modify(|compute_cell| {
                        compute_cell.compute_consumers.insert(compute_cell_id);
                    });
                }
            };
        });

        self.compute_cells.insert(
            compute_cell_id,
            ComputeCell {
                dependencies: dependencies.iter().copied().collect(),
                compute_func,
                value,
                compute_consumers: HashSet::new(),
                callback_consumers: HashSet::new(),
            },
        );

        Ok(compute_cell_id)
    }

    fn compute(&self, dependencies: &[CellId], compute_func: fn(&[T]) -> T) -> Result<T, CellId> {
        let mut dependencies_values = vec![];
        for &d in dependencies {
            let Some(v) = self.value(d) else {
                return Err(d);
            };
            dependencies_values.push(v);
        }
        Ok(compute_func(&dependencies_values))
    }

    pub fn value(&self, id: CellId) -> Option<T> {
        match id {
            CellId::Input(id) => self.input_cells.get(&id).map(|input_cell| input_cell.value),
            CellId::Compute(id) => self
                .compute_cells
                .get(&id)
                .map(|compute_cell| compute_cell.value),
        }
    }

    pub fn set_value(&mut self, id: InputCellId, new_value: T) -> bool {
        let Some(input_cell) = self.input_cells.get_mut(&id) else {
            return false;
        };
        input_cell.value = new_value;
        let compute_consumers = input_cell.conpute_consumers.clone();
        let mut callbacks_ids = HashSet::new();
        self.diffuse_change(compute_consumers, &mut callbacks_ids);
        callbacks_ids.iter().for_each(|callback_id| {
            let Some(callback) = self.callbacks.get_mut(callback_id) else {
                return;
            };
            let Some(compute_cell) = self.compute_cells.get(&callback.dependency) else {
                return;
            };
            if callback.last_value != compute_cell.value {
                (callback.func)(compute_cell.value);
                callback.last_value = compute_cell.value;
            }
        });
        true
    }

    fn diffuse_change(
        &mut self,
        conpute_consumers: HashSet<ComputeCellId>,
        callback_ids: &mut HashSet<CallbackId>,
    ) {
        for compute_cell_id in conpute_consumers.iter() {
            let Some(compute_cell) = self.compute_cells.get(&compute_cell_id) else {
                continue;
            };
            let Ok(value) = self.compute(&compute_cell.dependencies, compute_cell.compute_func)
            else {
                continue;
            };
            self.compute_cells
                .entry(*compute_cell_id)
                .and_modify(|compute_cell| compute_cell.value = value);

            let Some(compute_cell) = self.compute_cells.get(&compute_cell_id) else {
                continue;
            };
            compute_cell
                .callback_consumers
                .iter()
                .for_each(|&callback_id| {
                    callback_ids.insert(callback_id);
                });

            self.diffuse_change(compute_cell.compute_consumers.clone(), callback_ids);
        }
    }

    pub fn add_callback<F: FnMut(T) + 'a>(
        &mut self,
        compute_cell_id: ComputeCellId,
        callback: F,
    ) -> Option<CallbackId> {
        let callback_id = CallbackId::new();

        let compute_cell = self.compute_cells.get_mut(&compute_cell_id)?;
        compute_cell.callback_consumers.insert(callback_id);

        let callback = Callback {
            dependency: compute_cell_id,
            func: Box::new(callback),
            last_value: compute_cell.value,
        };

        self.callbacks.insert(callback_id, callback);
        Some(callback_id)
    }

    pub fn remove_callback(
        &mut self,
        compute_cell_id: ComputeCellId,
        callback_id: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        let compute_cell = self
            .compute_cells
            .get_mut(&compute_cell_id)
            .ok_or(RemoveCallbackError::NonexistentCell)?;

        if !compute_cell.callback_consumers.remove(&callback_id) {
            return Err(RemoveCallbackError::NonexistentCallback);
        }
        Ok(())
    }
}
