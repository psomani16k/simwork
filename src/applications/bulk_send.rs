use crate::core::{address::Endpoint, event::application_events::ConnectionStatus};

/// BulkSendApplication pushes 0's on the channel at the rate supported by the
/// underlaying stack. It assumes no processing delays.
pub struct BulkSendApplication {
    connection_status: ConnectionStatus,
    destination: Endpoint,
    sending: bool,
}

impl BulkSendApplication {}
