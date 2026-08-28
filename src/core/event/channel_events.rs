use crate::core::{
    device::id::DeviceId, event::device_events::ChannelToDevice, util::packet::Packet,
};

pub enum ChannelEvent {
    FromSelf(ChannelToSelf),
    FromDevice(DeviceId, DeviceToChannel),
}

pub enum ChannelToSelf {}

pub enum DeviceToChannel {
    Send(Packet),
}

pub enum ChannelOutput {
    ToSelf(ChannelToSelf),
    ToDevice(DeviceId, ChannelToDevice),
}
