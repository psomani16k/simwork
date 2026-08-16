use crate::core::{
    address::Endpoint,
    application::{ApplicationCtx, ApplicationImpl},
    event::{
        application_events::{ConnectionStatus, SocketErr},
        socket_events::SocketEventFromApplication,
    },
    util::{duration::Duration, size::Size},
};

/// BulkSendApplication pushes 0's on the channel at the rate supported by the
/// underlaying stack. It assumes no processing delays.
pub struct BulkSendApplication {
    connection_status: ConnectionStatus,
    destination: Endpoint,
    sending: bool,
}

impl BulkSendApplication {}

impl ApplicationImpl for BulkSendApplication {
    fn start(&mut self, _ctx: ApplicationCtx) -> Vec<(Duration, SocketEventFromApplication)> {
        vec![(
            Duration::ZERO,
            SocketEventFromApplication::Connect(self.destination),
        )]
    }

    fn pull_data(&mut self, _ctx: ApplicationCtx, buf: &mut [u8]) -> (Duration, Size) {
        if !self.sending {
            return (Duration::ZERO, Size::ZERO);
        }
        (Duration::ZERO, Size::from_bytes(buf.len() as u32))
    }

    fn send_callback(
        &mut self,
        ctx: ApplicationCtx,
        sent: Size,
    ) -> Vec<(Duration, SocketEventFromApplication)> {
        todo!()
    }

    fn socket_error(
        &mut self,
        _ctx: ApplicationCtx,
        err: SocketErr,
    ) -> Vec<(Duration, SocketEventFromApplication)> {
        match err {
            SocketErr::SendBufferOverflow { accepted_bytes: _ } => {}
            SocketErr::FailedToConnect => {}
        };
        vec![]
    }

    fn connection_status_update(
        &mut self,
        _ctx: ApplicationCtx,
        status: ConnectionStatus,
    ) -> Vec<(Duration, SocketEventFromApplication)> {
        match status {
            ConnectionStatus::Connected => self.connection_status = ConnectionStatus::Connected,
            ConnectionStatus::Disconnected => {
                self.connection_status = ConnectionStatus::Disconnected
            }
        };
        vec![]
    }

    fn receive_data(
        &mut self,
        _ctx: ApplicationCtx,
        _data: Vec<u8>,
    ) -> Vec<(Duration, SocketEventFromApplication)> {
        vec![]
    }

    fn stop(&mut self, _ctx: ApplicationCtx) -> Vec<(Duration, SocketEventFromApplication)> {
        vec![(Duration::ZERO, SocketEventFromApplication::Close)]
    }
}
