use crate::core::{event::socket_events::ApplicationToSocket, util::size::Size};

/// Everything that can be delivered to an application, grouped by who sent it.
pub enum ApplicationEvent {
    FromSelf(ApplicationToSelf),
    FromSim(SimToApplication),
    FromSocket(SocketToApplication),
}

pub enum SimToApplication {
    Start,
    Stop,
}

pub enum ApplicationToSelf {}

pub enum SocketToApplication {
    ConnectionStatus(ConnectionStatus),
    /// How much room is available in the socket buffer to for
    /// new data to be sent.
    Writable {
        available: Size,
    },
    /// Answers every `ApplicationToSocket::Send`, short or not.
    Sent {
        accepted: Size,
    },
    Data(Vec<u8>),
    Error(SocketError),
}

pub enum ConnectionStatus {
    Connected,
    Disconnected,
}

pub enum SocketError {
    SendBufferOverflow { accepted_bytes: Size },
    FailedToConnect,
}

/// Everything an application can produce, grouped by who receives it.
pub enum ApplicationOutput {
    ToSocket(ApplicationToSocket),
    ToSelf(ApplicationToSelf),
}
