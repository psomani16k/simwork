use crate::core::util::id::IdGenerator;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DeviceId(u64);

impl IdGenerator {
    pub fn new_device_id(&self) -> DeviceId {
        let id = self.get_id();
        DeviceId(id)
    }
}
