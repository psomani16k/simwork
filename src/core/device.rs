use crate::core::{channel::ChannelId, node::NodeId, util::id::IdGenerator};

pub struct Device {
    id: DeviceId,
    node: NodeId,
    channel: ChannelId,
}

#[derive(Clone, Copy)]
pub struct DeviceId(u64);

impl IdGenerator {
    pub fn new_device_id(&mut self) -> DeviceId {
        let id = self.get_id();
        DeviceId(id)
    }
}
