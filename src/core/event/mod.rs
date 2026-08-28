pub mod application_events;
pub mod channel_events;
pub mod device_events;
pub mod entity;
pub mod node_events;
pub mod socket_events;

use crate::core::{
    application::id::ApplicationId,
    channel::id::ChannelId,
    device::id::DeviceId,
    event::{
        application_events::ApplicationEvent, channel_events::ChannelEvent,
        device_events::DeviceEvent, node_events::NodeEvent, socket_events::SocketEvent,
    },
    node::id::NodeId,
    socket::id::SocketId,
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

    pub fn is_cancelled(&self) -> bool {
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
    ToApplication(ApplicationId, ApplicationEvent),
    ToSocket(SocketId, SocketEvent),
    ToNode(NodeId, NodeEvent),
    ToDevice(DeviceId, DeviceEvent),
    ToChannel(ChannelId, ChannelEvent),
}
