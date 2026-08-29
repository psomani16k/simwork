use crate::core::{
    event::{application_events::SocketToApplication, node_events::SocketToNode},
    util::{
        address::{Endpoint, Port},
        packet::{Packet, header::Header},
    },
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

impl NodeToSocket {
    pub fn destination_port(&self) -> Option<Port> {
        match self {
            NodeToSocket::Data(packet) => match packet.peek_header() {
                Header::TCP(tcp_header) => Some(tcp_header.destination_port()),
                Header::UDP(udp_header) => Some(udp_header.destination_port()),
                _ => None,
            },
        }
    }
}

/// Everything a socket can produce, grouped by who receives it.
pub enum SocketOutput {
    ToSelf(SocketToSelf),
    ToApplication(SocketToApplication),
    ToNode(SocketToNode),
}
