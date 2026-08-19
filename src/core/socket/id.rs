use crate::core::util::id::IdGenerator;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct SocketId(u64);

impl IdGenerator {
    pub fn new_socket_id(&mut self) -> SocketId {
        let id = self.get_id();
        SocketId(id)
    }
}
