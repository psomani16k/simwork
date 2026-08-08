use std::collections::HashMap;

use crate::core::{
    application::{Application, ApplicationId},
    channel::{Channel, ChannelId},
    device::{Device, DeviceId},
    event::{Event, EventId, EventType},
    event_queue::EventQueue,
    node::{Node, NodeId},
    socket::{Socket, SocketId},
    util::{id::IdGenerator, time::SimTime},
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

    pub fn schedule(&mut self, at: SimTime, event_type: EventType) -> EventId {
        let id = self.id_generator.new_event_id();
        self.queue.push_event(Event::new(id, at, event_type));
        id
    }

    pub fn cancel(&mut self, event: EventId) {
        self.queue.cancel_event(event);
    }

    pub fn now(&self) -> SimTime {
        self.now
    }

    pub fn run(&mut self) {
        while let Some(event) = self.queue.pop_event() {
            self.now = event.get_timestamp();
            if event.is_cancelled() {
                continue;
            }
            self.handle_event(event.event_type);
        }
    }

    fn handle_event(&mut self, event: EventType) {
        match event {
            EventType::ApplicationStart(app_id) => {
                self.applications.get_mut(&app_id).expect("Exists").start()
            }
            EventType::ApplicationStop(app_id) => {
                self.applications.get_mut(&app_id).expect("Exists").stop()
            }
            EventType::ToApplication(from, app_id, data) => {
                self.applications
                    .get_mut(&app_id)
                    .expect("Exists")
                    .handle_event(from, data);
            }
            EventType::ToSocket(from, _socket_id, _data) => todo!(),
            EventType::ToNode(from, _node_id, _data) => todo!(),
            EventType::ToDevice(from, _device_id, _data) => todo!(),
            EventType::ToChannel(from, _channel_id, _data) => todo!(),
        }
    }
}
