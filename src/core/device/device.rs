use crate::core::{
    address::IpAddress,
    channel::id::ChannelId,
    device::{id::DeviceId, interface::DeviceImpl},
    node::id::NodeId,
};

pub struct Device {
    id: DeviceId,
    node: NodeId,
    channel: ChannelId,
    ip_addr: IpAddress,

    device_impl: Box<dyn DeviceImpl>,
}
