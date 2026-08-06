use std::collections::HashMap;

use crate::core::{
    application::{Application, ApplicationId},
    channel::{Channel, ChannelId},
    device::{Device, DeviceId},
    event_queue::EventQueue,
    node::{Node, NodeId},
    socket::{Socket, SocketId},
    util::time::SimTime,
};

#[derive(Default)]
pub struct Sim {
    now: SimTime,

    /// Application layer
    applications: HashMap<ApplicationId, Application>,

    /// Transport layer
    sockets: HashMap<SocketId, Socket>,

    /// Network layer, also represents individual nodes
    nodes: HashMap<NodeId, Node>,

    /// Link layer
    devices: HashMap<DeviceId, Device>,

    /// Physical layer
    channels: HashMap<ChannelId, Channel>,

    queue: EventQueue,
}

impl Sim {}
