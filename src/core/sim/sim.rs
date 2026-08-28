use std::collections::HashMap;

use crate::core::{
    application::{application::Application, id::ApplicationId},
    channel::{channel::Channel, id::ChannelId},
    device::{device::Device, id::DeviceId},
    event::{Event, EventId, EventType},
    node::{id::NodeId, node::Node},
    socket::{id::SocketId, socket::Socket},
    util::{event_queue::EventQueue, id::IdGenerator, time::SimTime},
};

#[derive(Default)]
pub struct Sim {
    now: SimTime,

    id_generator: IdGenerator,

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
            EventType::ToApplication(app_id, data) => {
                let now = self.now();
                let app = self.applications.get_mut(&app_id).expect("Exists");
                let new_events = app.handle_event(data, now);
                let _ = self.schedule_raw_events(new_events);
            }
            EventType::ToSocket(_socket_id, _data) => todo!(),
            EventType::ToNode(_node_id, _data) => todo!(),
            EventType::ToDevice(_device_id, _data) => todo!(),
            EventType::ToChannel(_channel_id, _data) => todo!(),
        }
    }

    fn schedule_raw_events(&mut self, raw_events: Vec<(SimTime, EventType)>) -> Vec<EventId> {
        raw_events
            .into_iter()
            .map(|(at, event_type)| -> EventId { self.schedule(at, event_type) })
            .collect()
    }

    pub fn schedule(&mut self, at: SimTime, event_type: EventType) -> EventId {
        let id = self.id_generator.new_event_id();
        self.queue.push_event(Event::new(id, at, event_type));
        id
    }
}
