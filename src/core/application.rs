use crate::core::{
    event::{
        EventType,
        application_events::{
            ApplicationEventData, ApplicationEventFromSim, ApplicationEventFromSocket,
            ConnectionStatus, SocketErr,
        },
        entity::Entity,
        socket_events::{SocketEventData, SocketEventFromApplication},
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
        data: ApplicationEventData,
        now: SimTime,
    ) -> Vec<(SimTime, EventType)> {
        let event_data = match data {
            ApplicationEventData::FromSim(event) => self.handle_event_from_sim(event, now),
            ApplicationEventData::FromSocket(event) => self.handle_event_from_socket(event, now),
            ApplicationEventData::FromSelf() => todo!(),
        };

        let events = event_data
            .into_iter()
            .map(|(delay, data)| -> (SimTime, EventType) {
                let ts = now + delay;
                let event = EventType::ToSocket(
                    self.connected_socket(),
                    SocketEventData::FromApplication(data),
                );
                (ts, event)
            })
            .collect();
        events
    }

    fn handle_event_from_sim(
        &mut self,
        data: ApplicationEventFromSim,
        now: SimTime,
    ) -> Vec<(Duration, SocketEventFromApplication)> {
        let ctx = self.application_ctx(now);
        match data {
            ApplicationEventFromSim::Start => self.application_impl.start(ctx),
            ApplicationEventFromSim::Stop => self.application_impl.stop(ctx),
        }
    }

    fn handle_event_from_socket(
        &mut self,
        data: ApplicationEventFromSocket,
        now: SimTime,
    ) -> Vec<(Duration, SocketEventFromApplication)> {
        let ctx = self.application_ctx(now);
        match data {
            ApplicationEventFromSocket::ConnectionStatus(status) => {
                self.application_impl.connection_status_update(ctx, status)
            }
            ApplicationEventFromSocket::Data(data) => self.application_impl.receive_data(ctx, data),
            ApplicationEventFromSocket::Error(err) => self.application_impl.socket_error(ctx, err),
            ApplicationEventFromSocket::Sent { accepted } => {
                self.application_impl.send_callback(ctx, accepted)
            }
            ApplicationEventFromSocket::Writable { available } => {
                let mut data: Vec<u8> = vec![0u8; available.as_bytes() as usize];
                let (delay, filled) = self.application_impl.pull_data(ctx, &mut data);
                data.truncate(filled.as_bytes() as usize);
                let event_data = SocketEventFromApplication::SendData(data);
                vec![(delay, event_data)]
            }
        }
    }
}

pub enum EventsFromApplication {
    ToSocket(SocketEventFromApplication),
    ToApplication(ApplicationEventData)
}

pub trait ApplicationImpl {
    /// called when the application is started
    fn start(&mut self, ctx: ApplicationCtx) -> Vec<(Duration, SocketEventFromApplication)>;

    /// Socket layer requesting for more data if available
    fn pull_data(&mut self, ctx: ApplicationCtx, buf: &mut [u8]) -> (Duration, Size);

    /// Called to inform application of how many bytes were sent in the last send request
    fn send_callback(
        &mut self,
        ctx: ApplicationCtx,
        sent: Size,
    ) -> Vec<(Duration, SocketEventFromApplication)>;

    /// Erros thrown by socket arrive here
    fn socket_error(
        &mut self,
        ctx: ApplicationCtx,
        err: SocketErr,
    ) -> Vec<(Duration, SocketEventFromApplication)>;

    /// Called when there is an update from the socket regarding the connection status
    fn connection_status_update(
        &mut self,
        ctx: ApplicationCtx,
        status: ConnectionStatus,
    ) -> Vec<(Duration, SocketEventFromApplication)>;

    /// Called when socket has data to give to the application
    fn receive_data(
        &mut self,
        ctx: ApplicationCtx,
        data: Vec<u8>,
    ) -> Vec<(Duration, SocketEventFromApplication)>;

    /// called when the application is to be stopped
    fn stop(&mut self, ctx: ApplicationCtx) -> Vec<(Duration, SocketEventFromApplication)>;
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
