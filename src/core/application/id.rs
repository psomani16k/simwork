use crate::core::util::id::IdGenerator;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct ApplicationId(u64);

impl IdGenerator {
    pub fn new_application_id(&mut self) -> ApplicationId {
        let id = self.get_id();
        ApplicationId(id)
    }
}
