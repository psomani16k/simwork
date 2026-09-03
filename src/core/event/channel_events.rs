use crate::core::{
    event::device_events::ChannelToDevice,
    util::{address::MacAddress, packet::Packet},
};

pub enum ChannelEvent {
    FromSelf(ChannelToSelf),
    FromDevice(MacAddress, DeviceToChannel),
}

pub enum ChannelToSelf {}

pub enum DeviceToChannel {
    Send(Packet),
}

pub enum ChannelOutput {
    ToSelf(ChannelToSelf),
    ToDevice(MacAddress, ChannelToDevice),
}
