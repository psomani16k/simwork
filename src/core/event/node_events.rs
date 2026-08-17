use crate::core::event::{device_events::NodeToDevice, socket_events::NodeToSocket};

pub enum NodeEvent {
    FromSelf(NodeToSelf),
    FromSocket(SocketToNode),
    FromDevice(DeviceToNode),
}

pub enum NodeToSelf {}

pub enum SocketToNode {}

pub enum DeviceToNode {}

pub enum NodeOutput {
    ToSelf(NodeToSelf),
    ToSocket(NodeToSocket),
    ToDevice(NodeToDevice),
}
