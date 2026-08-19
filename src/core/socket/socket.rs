use crate::core::{
    address::Port,
    application::id::ApplicationId,
    event::socket_events::{ApplicationToSocket, NodeToSocket, SocketEvent, SocketOutput},
    node::id::NodeId,
    socket::{ctx::SocketCtx, id::SocketId, interface::SocketImpl},
    util::{duration::Duration, time::SimTime},
};

pub struct Socket {
    id: SocketId,
    application: ApplicationId,
    node: NodeId,
    port: Port,

    socket_impl: Box<dyn SocketImpl>,
}

impl Socket {
    pub fn port(&self) -> Port {
        self.port
    }

    pub fn connected_application(&self) -> ApplicationId {
        self.application
    }

    pub fn socket_ctx(&self, now: SimTime) -> SocketCtx {
        SocketCtx {
            node: self.node,
            id: self.id,
            application: self.application,
            now,
            port: self.port(),
        }
    }

    pub fn handle_event(
        &mut self,
        data: SocketEvent,
        now: SimTime,
    ) -> Vec<(Duration, SocketOutput)> {
        let ctx = self.socket_ctx(now);
        match data {
            SocketEvent::FromApplication(events) => self.handle_event_from_application(events, now),
            SocketEvent::FromNode(events) => self.handle_event_from_node(events, now),
        }
    }

    fn handle_event_from_application(
        &mut self,
        data: ApplicationToSocket,
        now: SimTime,
    ) -> Vec<(Duration, SocketOutput)> {
        let ctx = self.socket_ctx(now);
        match data {
            ApplicationToSocket::Close => self.socket_impl.close(ctx),
            ApplicationToSocket::Connect(endpoint) => self.socket_impl.connect(ctx, endpoint),
            ApplicationToSocket::ReceivePacket(packet) => self.socket_impl.on_receive(ctx, packet),
            ApplicationToSocket::Send(data) => self.socket_impl.on_send(ctx, data),
        }
    }

    fn handle_event_from_node(
        &mut self,
        data: NodeToSocket,
        now: SimTime,
    ) -> Vec<(Duration, SocketOutput)> {
        let ctx = self.socket_ctx(now);
        match data {}
    }
}
