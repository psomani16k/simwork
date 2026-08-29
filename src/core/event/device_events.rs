use crate::core::{
    event::{channel_events::DeviceToChannel, node_events::DeviceToNode},
    util::{
        address::IpAddress,
        packet::{Packet, header::Header},
    },
};

pub enum DeviceEvent {
    FromSelf(DeviceToSelf),
    FromNode(NodeToDevice),
    FromChannel(ChannelToDevice),
}

pub enum DeviceToSelf {}

pub enum NodeToDevice {
    Send(Packet),
}

pub enum ChannelToDevice {
    Data(Packet),
    TransmissionComplete,
    ChannelBusy,
}

impl NodeToDevice {
    pub fn source_ip_address(&self) -> Option<IpAddress> {
        match self {
            NodeToDevice::Send(packet) => match packet.peek_header() {
                Header::IPv4(ipv4_header) => Some(IpAddress::Ipv4(ipv4_header.source_addr())),
                Header::IPv6(ipv6_header) => Some(IpAddress::Ipv6(ipv6_header.source_addr())),
                _ => None,
            },
        }
    }
}

pub enum DeviceOutput {
    ToSelf(DeviceToSelf),
    ToNode(DeviceToNode),
    ToChannel(DeviceToChannel),
}
