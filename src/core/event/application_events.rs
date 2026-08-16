use crate::core::util::size::Size;

/// Events that an application can accept

pub enum ApplicationEventData {
    FromSelf(),
    FromSim(ApplicationEventFromSim),
    FromSocket(ApplicationEventFromSocket),
}

pub enum ApplicationEventFromSim {
    Start,
    Stop,
}

pub enum ApplicationEventFromSocket {
    ConnectionStatus(ConnectionStatus),
    Writable { available: Size }, // a credit to spend, not a promise
    Sent { accepted: Size },      // answers every Send, short or not
    Data(Vec<u8>),
    Error(SocketErr),
}

pub enum ConnectionStatus {
    Connected,
    Disconnected,
}

pub enum SocketErr {
    SendBufferOverflow { accepted_bytes: Size },
    FailedToConnect,
}
