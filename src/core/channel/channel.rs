use std::collections::HashSet;

use crate::core::{
    channel::{id::ChannelId, interface::ChannelImpl},
    device::id::DeviceId,
};

pub struct Channel {
    id: ChannelId,
    devices: HashSet<DeviceId>,

    channel_ipml: Box<dyn ChannelImpl>,
}
