pub mod application_events;
pub mod entity;
pub mod node_events;
pub mod socket_events;

use crate::core::{
    application::ApplicationId,
    channel::ChannelId,
    device::DeviceId,
    event::{
        application_events::ApplicationEventData, entity::Entity, node_events::NodeEventData,
        socket_events::SocketEventData,
    },
    node::NodeId,
    socket::SocketId,
    util::{id::IdGenerator, time::SimTime},
};

pub struct Event {
    id: EventId,
    timestamp: SimTime,
    cancelled: bool,
    pub event_type: EventType,
}

impl Event {
    pub fn new(id: EventId, timestamp: SimTime, event_type: EventType) -> Self {
        Event {
            id,
            timestamp,
            cancelled: false,
            event_type,
        }
    }

    pub fn id(&self) -> EventId {
        self.id
    }

    pub fn timestamp(&self) -> SimTime {
        self.timestamp
    }

    pub fn cancel(&mut self) {
        self.cancelled = true;
    }

    pub fn cancelled(&self) -> bool {
        self.cancelled
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub struct EventId(u64);

impl IdGenerator {
    pub fn new_event_id(&mut self) -> EventId {
        let id = self.get_id();
        EventId(id)
    }
}

pub enum EventType {
    ToApplication(ApplicationId, ApplicationEventData),
    ToSocket(SocketId, SocketEventData),
    ToNode(NodeId, NodeEventData),
    ToDevice(DeviceId, DeviceEventData),
    ToChannel(ChannelId, ChannelEventData),
}

pub enum DeviceEventData {}

pub enum ChannelEventData {}
