static ID_GENERATOR: IdGenerator = IdGenerator(0);

pub struct IdGenerator(u64);

impl IdGenerator {
    pub fn get_id(&mut self) -> u64 {
        let id = self.0;
        self.0 += 1;
        id
    }
}
