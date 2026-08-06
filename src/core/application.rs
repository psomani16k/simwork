use crate::core::{packet::Packet, socket::SocketId, util::id::IdGenerator};

pub struct Application {
    id: ApplicationId,
    socket_id: SocketId,

    application_impl: Box<dyn ApplicationImpl>,
}

pub trait ApplicationImpl {
    fn notify_sent(&self, sock: SocketId);
    fn receive_packet(&self, packet: Packet);
}

pub struct ApplicationId(u64);

impl IdGenerator {
    pub fn new_application_id(&mut self) -> ApplicationId {
        let id = self.get_id();
        ApplicationId(id)
    }
}
