use std::collections::HashMap;

use crate::core::{
    application::{application::Application, id::ApplicationId},
    channel::{channel::Channel, id::ChannelId},
    device::{device::Device, id::DeviceId},
    event::{Event, EventId, EventType},
    node::{id::NodeId, node::Node},
    sim::ctx::SimCtx,
    socket::{id::SocketId, socket::Socket},
    util::{event_queue::EventQueue, time::SimTime},
};

#[derive(Default)]
pub struct Sim {
    now: SimTime,

    ctx: SimCtx,

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

impl Sim {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cancel(&mut self, event: EventId) {
        self.queue.cancel_event(event);
    }

    pub fn now(&self) -> SimTime {
        self.now
    }

    pub fn run(&mut self) {
        while let Some(event) = self.queue.pop_event() {
            if event.is_cancelled() || self.now.is_after(event.timestamp()) {
                continue;
            }
            self.now = event.timestamp();
            self.handle_event(event);
        }
    }

    fn handle_event(&mut self, event: Event) {
        let event_type = event.event_type;
        match event_type {
            EventType::ToApplication(app_id, event) => {
                let app = self.applications.get_mut(&app_id).expect("Exists");
                let new_events = app.handle_event(&self.ctx, event);
                let _ = self.schedule_raw_events(new_events);
            }
            EventType::ToSocket(socket_id, event) => {
                let socket = self.sockets.get_mut(&socket_id).expect("Exists");
                let new_events = socket.handle_event(&self.ctx, event);
                let _ = self.schedule_raw_events(new_events);
            }
            EventType::ToNode(node_id, event) => {
                let node = self.nodes.get_mut(&node_id).expect("Exists");
                let new_events = node.handle_event(&self.ctx, event);
                let _ = self.schedule_raw_events(new_events);
            }
            EventType::ToDevice(device_id, event) => {
                let device = self.devices.get_mut(&device_id).expect("Exists");
                let new_events = device.handle_event(&self.ctx, event);
                let _ = self.schedule_raw_events(new_events);
            }
            EventType::ToChannel(channel_id, event) => {
                let channel = self.channels.get_mut(&channel_id).expect("Exists");
                let new_events = channel.handle_event(&self.ctx, event);
                let _ = self.schedule_raw_events(new_events);
            }
        }
    }

    fn schedule_raw_events(&mut self, raw_events: Vec<(SimTime, EventType)>) -> Vec<EventId> {
        raw_events
            .into_iter()
            .map(|(at, event_type)| -> EventId { self.schedule(at, event_type) })
            .collect()
    }

    pub fn schedule(&mut self, at: SimTime, event_type: EventType) -> EventId {
        let id = self.ctx.id_generator.new_event_id();
        self.queue.push_event(Event::new(id, at, event_type));
        id
    }
}
