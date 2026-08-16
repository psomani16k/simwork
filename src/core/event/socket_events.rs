use crate::core::{address::Endpoint, packet::Packet};

pub enum SocketEventData {
    FromApplication(SocketEventFromApplication),
    FromNode(SocketEventFromNode),
}

pub enum SocketEventFromApplication {
    Connect(Endpoint),
    Close,
    SendData(Vec<u8>),
    ReceivePacket(Packet),
}

pub enum SocketEventFromNode {}
