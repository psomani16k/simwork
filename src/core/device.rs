use crate::core::{address::IpAddress, channel::ChannelId, node::NodeId, util::id::IdGenerator};

pub struct Device {
    id: DeviceId,
    node: NodeId,
    channel: ChannelId,
    ip_addr: IpAddress,

    device_impl: Box<dyn DeviceImpl>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DeviceId(u64);

impl IdGenerator {
    pub fn new_device_id(&mut self) -> DeviceId {
        let id = self.get_id();
        DeviceId(id)
    }
}

pub trait DeviceImpl {}
