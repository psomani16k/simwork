use std::cell::Cell;

#[derive(Debug, Default)]
pub struct IdGenerator(Cell<u64>);

impl IdGenerator {
    pub fn get_id(&self) -> u64 {
        let id = self.0.get();
        self.0.set(id + 1);
        id
    }

    pub fn new() -> Self {
        Default::default()
    }
}
