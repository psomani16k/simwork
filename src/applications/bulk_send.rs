use crate::core::{
    application::interface::ApplicationImpl,
    event::{
        application_events::{ApplicationOutput, ConnectionStatus, SocketError},
        socket_events::ApplicationToSocket,
    },
    sim::ctx::SimCtx,
    util::{address::Endpoint, duration::Duration, size::Size},
};

/// BulkSendApplication pushes 0's on the channel at the rate supported by the
/// underlaying stack. It assumes no processing delays.
pub struct BulkSendApplication {
    connection_status: ConnectionStatus,
    destination: Endpoint,
    sent: Size,
}

impl ApplicationImpl for BulkSendApplication {
    fn on_start(&mut self, _ctx: &SimCtx) -> Vec<(Duration, ApplicationOutput)> {
        vec![(
            Duration::ZERO,
            ApplicationOutput::ToSocket(ApplicationToSocket::Connect(self.destination)),
        )]
    }

    fn on_sendable(&mut self, _ctx: &SimCtx, buf: &mut [u8]) -> (Duration, Size) {
        let sendable = buf.len();
        (Duration::ZERO, Size::from_bytes(sendable as u32))
    }

    fn send_callback(&mut self, _ctx: &SimCtx, sent: Size) -> Vec<(Duration, ApplicationOutput)> {
        self.sent += sent;
        vec![]
    }

    fn on_socket_error(
        &mut self,
        _ctx: &SimCtx,
        _err: SocketError,
    ) -> Vec<(Duration, ApplicationOutput)> {
        vec![]
    }

    fn on_connection_status_update(
        &mut self,
        _ctx: &SimCtx,
        status: ConnectionStatus,
    ) -> Vec<(Duration, ApplicationOutput)> {
        match status {
            ConnectionStatus::Connected => {
                self.connection_status = ConnectionStatus::Connected;
                vec![(
                    Duration::ZERO,
                    ApplicationOutput::ToSocket(ApplicationToSocket::Send(Vec::from(
                        [0u8; 1_000_000],
                    ))),
                )]
            }
            ConnectionStatus::Disconnected => {
                self.connection_status = ConnectionStatus::Disconnected;
                vec![]
            }
        }
    }

    fn on_receive(&mut self, _ctx: &SimCtx, _data: Vec<u8>) -> Vec<(Duration, ApplicationOutput)> {
        vec![]
    }

    fn on_stop(&mut self, _ctx: &SimCtx) -> Vec<(Duration, ApplicationOutput)> {
        vec![(
            Duration::ZERO,
            ApplicationOutput::ToSocket(ApplicationToSocket::Close),
        )]
    }
}
