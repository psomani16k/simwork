use crate::core::{application::ApplicationId, util::id::IdGenerator};

pub struct Socket {
    id: SocketId,
    application_id: ApplicationId,
}

pub struct SocketId(u64);

impl IdGenerator {
    pub fn new_socket_id(&mut self) -> SocketId {
        let id = self.get_id();
        SocketId(id)
    }
}
