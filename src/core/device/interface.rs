use crate::core::{
    device::ctx::DeviceCtx,
    event::device_events::DeviceOutput,
    util::{duration::Duration, packet::Packet},
};

pub trait DeviceImpl {
    fn on_packet_from_node(
        &mut self,
        ctx: DeviceCtx,
        packet: Packet,
    ) -> Vec<(Duration, DeviceOutput)>;

    fn on_packet_from_channel(
        &mut self,
        ctx: DeviceCtx,
        packet: Packet,
    ) -> Vec<(Duration, DeviceOutput)>;
}
