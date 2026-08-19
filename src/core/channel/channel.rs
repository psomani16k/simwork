use crate::core::{channel::id::ChannelId, device::id::DeviceId};

pub struct Channel {
    id: ChannelId,
    head: DeviceId,
    tail: DeviceId,
}
