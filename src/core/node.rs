use crate::core::{device::DeviceId, socket::SocketId, util::id::IdGenerator};

pub struct Node {
    id: NodeId,
    sockets: Vec<SocketId>,
    devices: Vec<DeviceId>,
}

impl Node {
    pub fn get_id(&self) -> NodeId {
        self.id
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(u64);

impl IdGenerator {
    pub fn new_node_id(&mut self) -> NodeId {
        let id = self.get_id();
        NodeId(id)
    }
}
