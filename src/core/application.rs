use crate::core::{
    event::{
        EventType,
        application_events::{
            ApplicationEvent, ApplicationOutput, ApplicationToSelf, ConnectionStatus,
            SimToApplication, SocketError, SocketToApplication,
        },
        socket_events::{ApplicationToSocket, SocketEvent},
    },
    socket::SocketId,
    util::{duration::Duration, id::IdGenerator, size::Size, time::SimTime},
};

pub struct Application {
    id: ApplicationId,
    socket: SocketId,

    application_impl: Box<dyn ApplicationImpl>,
}

impl Application {
    pub fn connected_socket(&self) -> SocketId {
        self.socket
    }

    pub fn id(&self) -> ApplicationId {
        self.id
    }

    pub fn application_ctx(&self, now: SimTime) -> ApplicationCtx {
        ApplicationCtx { now }
    }

    pub fn handle_event(
        &mut self,
        data: ApplicationEvent,
        now: SimTime,
    ) -> Vec<(SimTime, EventType)> {
        let event_data = match data {
            ApplicationEvent::FromSim(event) => self.handle_event_from_sim(event, now),
            ApplicationEvent::FromSocket(event) => self.handle_event_from_socket(event, now),
            ApplicationEvent::FromSelf(event) => self.handle_event_from_application(event, now),
        };

        let events = event_data
            .into_iter()
            .map(|(delay, data)| -> (SimTime, EventType) {
                let ts = now + delay;
                let event = match data {
                    ApplicationOutput::ToSocket(socket_event) => EventType::ToSocket(
                        self.connected_socket(),
                        SocketEvent::FromApplication(socket_event),
                    ),
                };
                (ts, event)
            })
            .collect();
        events
    }

    fn handle_event_from_application(
        &mut self,
        data: ApplicationToSelf,
        now: SimTime,
    ) -> Vec<(Duration, ApplicationOutput)> {
        let _ctx = self.application_ctx(now);
        match data {}
    }

    fn handle_event_from_sim(
        &mut self,
        data: SimToApplication,
        now: SimTime,
    ) -> Vec<(Duration, ApplicationOutput)> {
        let ctx = self.application_ctx(now);
        match data {
            SimToApplication::Start => self.application_impl.on_start(ctx),
            SimToApplication::Stop => self.application_impl.on_stop(ctx),
        }
    }

    fn handle_event_from_socket(
        &mut self,
        data: SocketToApplication,
        now: SimTime,
    ) -> Vec<(Duration, ApplicationOutput)> {
        let ctx = self.application_ctx(now);
        match data {
            SocketToApplication::ConnectionStatus(status) => {
                self.application_impl.on_connection_status_update(ctx, status)
            }
            SocketToApplication::Data(data) => self.application_impl.on_receive(ctx, data),
            SocketToApplication::Error(err) => self.application_impl.on_socket_error(ctx, err),
            SocketToApplication::Sent { accepted } => {
                self.application_impl.send_callback(ctx, accepted)
            }
            SocketToApplication::Writable { available } => {
                let mut data: Vec<u8> = vec![0u8; available.as_bytes() as usize];
                let (delay, filled) = self.application_impl.on_sendable(ctx, &mut data);
                data.truncate(filled.as_bytes() as usize);
                let event_data = ApplicationOutput::ToSocket(ApplicationToSocket::Send(data));
                vec![(delay, event_data)]
            }
        }
    }
}

pub trait ApplicationImpl {
    /// called when the application is started
    fn on_start(&mut self, ctx: ApplicationCtx) -> Vec<(Duration, ApplicationOutput)>;

    /// Socket layer requesting for more data if available
    fn on_sendable(&mut self, ctx: ApplicationCtx, buf: &mut [u8]) -> (Duration, Size);

    /// Called to inform application of how many bytes were sent in the last send request
    fn send_callback(
        &mut self,
        ctx: ApplicationCtx,
        sent: Size,
    ) -> Vec<(Duration, ApplicationOutput)>;

    /// Erros thrown by socket arrive here
    fn on_socket_error(
        &mut self,
        ctx: ApplicationCtx,
        err: SocketError,
    ) -> Vec<(Duration, ApplicationOutput)>;

    /// Called when there is an update from the socket regarding the connection status
    fn on_connection_status_update(
        &mut self,
        ctx: ApplicationCtx,
        status: ConnectionStatus,
    ) -> Vec<(Duration, ApplicationOutput)>;

    /// Called when socket has data to give to the application
    fn on_receive(
        &mut self,
        ctx: ApplicationCtx,
        data: Vec<u8>,
    ) -> Vec<(Duration, ApplicationOutput)>;

    /// called when the application is to be stopped
    fn on_stop(&mut self, ctx: ApplicationCtx) -> Vec<(Duration, ApplicationOutput)>;
}

pub struct ApplicationCtx {
    pub now: SimTime,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct ApplicationId(u64);

impl IdGenerator {
    pub fn new_application_id(&mut self) -> ApplicationId {
        let id = self.get_id();
        ApplicationId(id)
    }
}
