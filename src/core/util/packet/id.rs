use crate::core::util::id::IdGenerator;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PacketId(u64);

impl IdGenerator {
    pub fn new_packet_id(&self) -> PacketId {
        let id = self.get_id();
        PacketId(id)
    }
}
