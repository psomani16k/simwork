use crate::core::{
    device::id::DeviceId,
    event::{device_events::NodeToDevice, socket_events::NodeToSocket},
    socket::id::SocketId,
    util::{address::IpAddress, packet::Packet},
};

pub enum NodeEvent {
    FromSelf(NodeToSelf),
    FromSocket(SocketToNode),
    FromDevice(DeviceToNode),
}

pub enum NodeToSelf {}

pub enum SocketToNode {
    Send(Packet, IpAddress),
}

pub enum DeviceToNode {
    Data(Packet),
}

pub enum NodeOutput {
    ToSelf(NodeToSelf),
    ToSocket(NodeToSocket),
    ToDevice(NodeToDevice),
}
