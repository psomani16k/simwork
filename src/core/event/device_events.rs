use crate::core::{
    event::{channel_events::DeviceToChannel, node_events::DeviceToNode},
    node::protocol_key::NodeProtocolKey,
    util::{address::MacAddress, packet::Packet},
};

pub enum DeviceEvent {
    FromSelf(DeviceToSelf),
    FromNode(NodeToDevice),
    FromChannel(ChannelToDevice),
}

pub enum DeviceToSelf {}

pub enum NodeToDevice {
    Send(
        Packet,
        NodeProtocolKey,
        MacAddress, // next hop mac
    ),
}

pub enum ChannelToDevice {
    Data(Packet),
    ReadyToTransmit,
    ChannelBusy(Packet),
}

pub enum DeviceOutput {
    ToSelf(DeviceToSelf),
    ToNode(DeviceToNode),
    ToChannel(DeviceToChannel),
}
