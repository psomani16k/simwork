use crate::core::{
    application::ApplicationId,
    channel::ChannelId,
    device::DeviceId,
    node::NodeId,
    socket::SocketId,
    util::{id::IdGenerator, time::SimTime},
};

pub struct Event {
    id: EventId,
    timestamp: SimTime,
    is_cancelled: bool,
    pub event_type: EventType,
}

impl Event {
    pub fn new(id: EventId, timestamp: SimTime, event_type: EventType) -> Self {
        Event {
            id,
            timestamp,
            is_cancelled: false,
            event_type,
        }
    }

    pub fn get_id(&self) -> EventId {
        self.id
    }

    pub fn get_timestamp(&self) -> SimTime {
        self.timestamp
    }

    pub fn cancel(&mut self) {
        self.is_cancelled = true;
    }

    pub fn is_cancelled(&self) -> bool {
        self.is_cancelled
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
    ApplicationStart(ApplicationId),
    ApplicationStop(ApplicationId),
    ToApplication(Entity, ApplicationId, ApplicationEventData),
    ToSocket(Entity, SocketId, SocketEventData),
    ToNode(Entity, NodeId, NodeEventData),
    ToDevice(Entity, DeviceId, DeviceEventData),
    ToChannel(Entity, ChannelId, ChannelEventData),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Entity {
    Application(ApplicationId),
    Socket(SocketId),
    Node(NodeId),
    Device(DeviceId),
    Channel(ChannelId),
    Sim,
}

pub enum ApplicationEventData {}

pub enum SocketEventData {}

pub enum NodeEventData {}

pub enum DeviceEventData {}

pub enum ChannelEventData {}
