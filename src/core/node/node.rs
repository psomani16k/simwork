use std::{collections::HashMap, vec};

use crate::core::{
    device::id::DeviceId,
    event::{
        EventType,
        device_events::DeviceEvent,
        node_events::{DeviceToNode, NodeEvent, NodeOutput, NodeToSelf, SocketToNode},
        socket_events::SocketEvent,
    },
    node::{id::NodeId, interface::NodeImpl},
    sim::ctx::SimCtx,
    socket::id::SocketId,
    util::{
        address::{IpAddress, Port},
        duration::Duration,
        time::SimTime,
    },
};

pub struct Node {
    id: NodeId,
    sockets: HashMap<Port, SocketId>,
    devices: HashMap<IpAddress, DeviceId>,

    node_impl: Box<dyn NodeImpl>,
}

impl Node {
    pub fn get_id(&self) -> NodeId {
        self.id
    }

    pub fn handle_event(&mut self, ctx: &SimCtx, event: NodeEvent) -> Vec<(SimTime, EventType)> {
        let event_data = match event {
            NodeEvent::FromSelf(from_self) => self.handle_event_from_self(ctx, from_self),
            NodeEvent::FromSocket(from_socket) => self.handle_event_from_socket(ctx, from_socket),
            NodeEvent::FromDevice(from_device) => self.handle_event_from_device(ctx, from_device),
        };

        let events = event_data
            .into_iter()
            .filter_map(|(delay, data)| {
                let event = match data {
                    NodeOutput::ToSelf(node_to_self) => {
                        EventType::ToNode(self.id, NodeEvent::FromSelf(node_to_self))
                    }
                    NodeOutput::ToSocket(node_to_socket) => {
                        let port = node_to_socket.destination_port()?;
                        let socket_id = *self.sockets.get(&port)?;
                        EventType::ToSocket(socket_id, SocketEvent::FromNode(node_to_socket))
                    }
                    NodeOutput::ToDevice(node_to_device) => {
                        let src_addr = node_to_device.source_ip_address()?;
                        let device_id = *self.devices.get(&src_addr)?;
                        EventType::ToDevice(device_id, DeviceEvent::FromNode(node_to_device))
                    }
                };
                Some((ctx.now + delay, event))
            })
            .collect();
        events
    }

    fn handle_event_from_self(
        &mut self,
        ctx: &SimCtx,
        event: NodeToSelf,
    ) -> Vec<(Duration, NodeOutput)> {
        match event {}
        vec![]
    }

    fn handle_event_from_socket(
        &mut self,
        ctx: &SimCtx,
        event: SocketToNode,
    ) -> Vec<(Duration, NodeOutput)> {
        match event {
            SocketToNode::Send(packet, dst_ip) => {
                self.node_impl.on_packet_from_socket(ctx, packet, dst_ip)
            }
        }
    }

    fn handle_event_from_device(
        &mut self,
        ctx: &SimCtx,
        event: DeviceToNode,
    ) -> Vec<(Duration, NodeOutput)> {
        match event {
            DeviceToNode::Data(packet) => self.node_impl.on_packet_from_device(ctx, packet),
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
