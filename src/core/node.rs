use std::collections::HashSet;

use crate::core::{
    device::DeviceId,
    socket::SocketId,
    util::{id::IdGenerator, time::SimTime},
};

pub struct Node {
    id: NodeId,
    sockets: HashSet<SocketId>,
    devices: HashSet<DeviceId>,
}

impl Node {
    pub fn get_id(&self) -> NodeId {
        self.id
    }
}

pub trait NodeImpl {}

pub struct NodeCtx {
    pub now: SimTime,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct NodeId(u64);

impl IdGenerator {
    pub fn new_node_id(&mut self) -> NodeId {
        let id = self.get_id();
        NodeId(id)
    }
}
