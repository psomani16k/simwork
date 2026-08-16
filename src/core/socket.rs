use crate::core::{
    address::Endpoint,
    application::ApplicationId,
    event::{
        EventType,
        socket_events::{SocketEventData, SocketEventFromApplication, SocketEventFromNode},
    },
    node::NodeId,
    packet::Packet,
    util::{id::IdGenerator, time::SimTime},
};

pub struct Socket {
    id: SocketId,
    application: ApplicationId,
    node: NodeId,

    socket_impl: Box<dyn SocketImpl>,
}

impl Socket {
    pub fn connected_application(&self) -> ApplicationId {
        self.application
    }

    pub fn socket_ctx(&self, now: SimTime) -> SocketCtx {
        SocketCtx {
            node: self.node,
            id: self.id,
            application: self.application,
            now,
        }
    }

    pub fn handle_event(
        &mut self,
        data: SocketEventData,
        now: SimTime,
    ) -> Vec<(SimTime, EventType)> {
        match data {
            SocketEventData::FromApplication(events) => {
                self.handle_event_from_application(events, now)
            }
            SocketEventData::FromNode(events) => self.handle_event_from_node(events, now),
        }
    }

    fn handle_event_from_application(
        &mut self,
        data: SocketEventFromApplication,
        now: SimTime,
    ) -> Vec<(SimTime, EventType)> {
        let ctx = self.socket_ctx(now);
        match data {
            SocketEventFromApplication::Close => self.socket_impl.close(ctx),
            SocketEventFromApplication::Connect(endpoint) => {
                self.socket_impl.connect(ctx, endpoint)
            }
            SocketEventFromApplication::ReceivePacket(packet) => {
                self.socket_impl.receive_from_destination(ctx, packet)
            }
            SocketEventFromApplication::SendData(data) => {
                self.socket_impl.send_to_destination(ctx, data)
            }
        }
    }

    fn handle_event_from_node(
        &mut self,
        data: SocketEventFromNode,
        now: SimTime,
    ) -> Vec<(SimTime, EventType)> {
        let ctx = self.socket_ctx(now);
        match data {}
    }
}

pub trait SocketImpl {
    /// Called when the application wants to initiate a connection
    fn connect(&mut self, ctx: SocketCtx, endpoint: Endpoint) -> Vec<(SimTime, EventType)>;

    /// Called when application requests to send data to destination
    fn send_to_destination(&mut self, ctx: SocketCtx, data: Vec<u8>) -> Vec<(SimTime, EventType)>;

    /// Called when there is a packet coming from destination
    fn receive_from_destination(
        &mut self,
        ctx: SocketCtx,
        packet: Packet,
    ) -> Vec<(SimTime, EventType)>;

    /// Close socket
    fn close(&mut self, ctx: SocketCtx) -> Vec<(SimTime, EventType)>;
}

pub struct SocketCtx {
    pub node: NodeId,
    pub id: SocketId,
    pub application: ApplicationId,
    pub now: SimTime,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SocketId(u64);

impl IdGenerator {
    pub fn new_socket_id(&mut self) -> SocketId {
        let id = self.get_id();
        SocketId(id)
    }
}
