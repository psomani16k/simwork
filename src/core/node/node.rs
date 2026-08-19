use std::collections::HashMap;

use crate::core::{
    address::{IpAddress, Port},
    device::id::DeviceId,
    node::{id::NodeId, interface::NodeImpl},
    socket::id::SocketId,
};

pub struct Node {
    id: NodeId,
    sockets: HashMap<SocketId, SocketData>,
    devices: HashMap<DeviceId, DeviceData>,

    node_impl: Box<dyn NodeImpl>,
}

impl Node {
    pub fn get_id(&self) -> NodeId {
        self.id
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct SocketData {
    pub id: SocketId,
    pub port: Port,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct DeviceData {
    pub id: DeviceId,
    pub ip_addr: IpAddress,
}
