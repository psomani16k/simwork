use crate::core::{
    address::Endpoint,
    application::{ApplicationCtx, ApplicationImpl},
    event::{
        application_events::{ConnectionStatus, SocketError},
        socket_events::ApplicationToSocket,
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
