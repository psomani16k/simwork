use crate::core::{
    event::{ApplicationEventData, Entity},
    packet::Packet,
    socket::SocketId,
    util::id::IdGenerator,
};

pub struct Application {
    id: ApplicationId,
    socket_id: SocketId,

    application_impl: Box<dyn ApplicationImpl>,
}

impl Application {
    pub fn start(&mut self) {
        self.application_impl.start();
    }

    pub fn stop(&mut self) {
        self.application_impl.stop();
    }

    pub fn handle_event(&mut self, from: Entity, data: ApplicationEventData) {}
}

pub trait ApplicationImpl {
    fn start(&mut self);
    fn stop(&mut self);
    fn notify_sent(&self, sock: SocketId);
    fn receive_packet(&self, packet: Packet);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct ApplicationId(u64);

impl IdGenerator {
    pub fn new_application_id(&mut self) -> ApplicationId {
        let id = self.get_id();
        ApplicationId(id)
    }
}
