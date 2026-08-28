use crate::core::{
    application::id::ApplicationId,
    event::{
        EventType,
        application_events::ApplicationEvent,
        node_events::NodeEvent,
        socket_events::{ApplicationToSocket, NodeToSocket, SocketEvent, SocketOutput},
    },
    node::id::NodeId,
    sim::ctx::SimCtx,
    socket::{ctx::SocketCtx, id::SocketId, interface::SocketImpl},
    util::{address::Port, duration::Duration, time::SimTime},
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

    pub fn handle_event(&mut self, ctx: &SimCtx, event: SocketEvent) -> Vec<(SimTime, EventType)> {
        let event_data = match event {
            SocketEvent::FromApplication(events) => self.handle_event_from_application(ctx, events),
            SocketEvent::FromNode(events) => self.handle_event_from_node(ctx, events),
        };

        let events = event_data
            .into_iter()
            .map(|(delay, data)| -> (SimTime, EventType) {
                let ts = ctx.now + delay;
                let event = match data {
                    SocketOutput::ToSelf(to_self) => {
                        EventType::ToSocket(self.id, SocketEvent::FromSelf(to_self))
                    }
                    SocketOutput::ToApplication(socket_to_application) => EventType::ToApplication(
                        self.application,
                        ApplicationEvent::FromSocket(socket_to_application),
                    ),
                    SocketOutput::ToNode(socket_to_node) => {
                        EventType::ToNode(self.node, NodeEvent::FromSocket(socket_to_node))
                    }
                };
                (ts, event)
            })
            .collect();
        events
    }

    fn handle_event_from_application(
        &mut self,
        ctx: &SimCtx,
        data: ApplicationToSocket,
    ) -> Vec<(Duration, SocketOutput)> {
        match data {
            ApplicationToSocket::Close => self.socket_impl.close(ctx),
            ApplicationToSocket::Connect(endpoint) => self.socket_impl.connect(ctx, endpoint),
            ApplicationToSocket::ReceivePacket(packet) => self.socket_impl.on_receive(ctx, packet),
            ApplicationToSocket::Send(data) => self.socket_impl.on_send(ctx, data),
        }
    }

    fn handle_event_from_node(
        &mut self,
        ctx: &SimCtx,
        data: NodeToSocket,
    ) -> Vec<(Duration, SocketOutput)> {
        match data {
            NodeToSocket::Data(packet) => self.socket_impl.on_receive(ctx, packet),
        }
    }
}
