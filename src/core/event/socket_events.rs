use crate::core::{
    event::{application_events::SocketToApplication, node_events::SocketToNode},
    util::{address::Endpoint, packet::Packet},
};

/// Everything that can be delivered to a socket, grouped by who sent it.
pub enum SocketEvent {
    FromSelf(SocketToSelf),
    FromApplication(ApplicationToSocket),
    FromNode(NodeToSocket),
}

pub enum SocketToSelf {}

pub enum ApplicationToSocket {
    Connect(Endpoint),
    Close,
    Send(Vec<u8>),
    ReceivePacket(Packet),
}

pub enum NodeToSocket {
    Data(Packet),
}

/// Everything a socket can produce, grouped by who receives it.
pub enum SocketOutput {
    ToSelf(SocketToSelf),
    ToApplication(SocketToApplication),
    ToNode(SocketToNode),
}
