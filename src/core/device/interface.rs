use crate::core::{
    device::ctx::DeviceCtx,
    event::device_events::DeviceOutput,
    node::protocol_key::NodeProtocolKey,
    util::{address::MacAddress, duration::Duration, packet::Packet},
};

pub trait DeviceImpl {
    fn on_packet_from_node(
        &mut self,
        ctx: &DeviceCtx,
        packet: Packet,
        protocol: NodeProtocolKey,
        dest_mac: MacAddress,
    ) -> Vec<(Duration, DeviceOutput)>;

    fn on_packet_from_channel(
        &mut self,
        ctx: &DeviceCtx,
        packet: Packet,
    ) -> Vec<(Duration, DeviceOutput)>;

    fn on_channel_free(&mut self, ctx: &DeviceCtx) -> Vec<(Duration, DeviceOutput)>;

    fn on_channel_busy(
        &mut self,
        ctx: &DeviceCtx,
        returned_packet: Packet,
    ) -> Vec<(Duration, DeviceOutput)>;
}
