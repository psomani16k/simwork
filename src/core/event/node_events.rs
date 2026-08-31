use crate::core::{
    event::{device_events::NodeToDevice, socket_events::NodeToSocket},
    util::{
        address::{IpAddress, MacAddress, Port},
        packet::Packet,
    },
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
    ToSocket(NodeToSocket, Port),
    ToDevice(
        NodeToDevice,
        MacAddress, // mac of device this event addresses
    ),
}
