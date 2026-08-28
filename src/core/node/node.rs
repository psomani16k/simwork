use std::{collections::HashMap, vec};

use crate::core::{
    device::id::DeviceId,
    event::node_events::{DeviceToNode, NodeEvent, NodeOutput, NodeToSelf, SocketToNode},
    node::{ctx::NodeCtx, id::NodeId, interface::NodeImpl},
    socket::id::SocketId,
    util::{
        address::{IpAddress, Port},
        duration::Duration,
        time::SimTime,
    },
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

    pub fn handle_event(&mut self, event: NodeEvent, now: SimTime) -> Vec<(Duration, NodeOutput)> {
        match event {
            NodeEvent::FromSelf(from_self) => self.handle_event_from_self(from_self, now),
            NodeEvent::FromSocket(from_node) => self.handle_event_from_socket(from_node, now),
            NodeEvent::FromDevice(from_device) => self.handle_event_from_device(from_device, now),
        }
    }

    fn handle_event_from_self(
        &mut self,
        event: NodeToSelf,
        now: SimTime,
    ) -> Vec<(Duration, NodeOutput)> {
        let ctx = NodeCtx::new(now);
        match event {}
        vec![]
    }

    fn handle_event_from_socket(
        &mut self,
        event: SocketToNode,
        now: SimTime,
    ) -> Vec<(Duration, NodeOutput)> {
        let ctx = NodeCtx::new(now);
        match event {
            SocketToNode::Send(packet, dst_ip) => {
                self.node_impl.on_packet_from_socket(ctx, packet, dst_ip)
            }
        }
    }

    fn handle_event_from_device(
        &mut self,
        event: DeviceToNode,
        now: SimTime,
    ) -> Vec<(Duration, NodeOutput)> {
        let ctx = NodeCtx::new(now);
        match event {
            DeviceToNode::Data(packet) => todo!(),
        }
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
