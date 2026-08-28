use crate::core::{
    event::device_events::DeviceOutput,
    sim::ctx::SimCtx,
    util::{duration::Duration, packet::Packet},
};

pub trait DeviceImpl {
    fn on_packet_from_node(
        &mut self,
        ctx: &SimCtx,
        packet: Packet,
    ) -> Vec<(Duration, DeviceOutput)>;

    fn on_packet_from_channel(
        &mut self,
        ctx: &SimCtx,
        packet: Packet,
    ) -> Vec<(Duration, DeviceOutput)>;
}
