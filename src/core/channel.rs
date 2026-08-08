use crate::core::{device::DeviceId, util::id::IdGenerator};

pub struct Channel {
    id: ChannelId,
    head: DeviceId,
    tail: DeviceId,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ChannelId(u64);

impl IdGenerator {
    pub fn new_channel_id(&mut self) -> ChannelId {
        let id = self.get_id();
        ChannelId(id)
    }
}
