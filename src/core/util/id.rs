#[derive(Default)]
pub struct IdGenerator(u64);

impl IdGenerator {
    pub fn get_id(&mut self) -> u64 {
        let id = self.0;
        self.0 += 1;
        id
    }

    pub fn new() -> Self {
        Default::default()
    }
}
