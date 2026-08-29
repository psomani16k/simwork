use crate::core::{
    device::interface::DeviceImpl,
    event::device_events::DeviceOutput,
    sim::ctx::SimCtx,
    util::{duration::Duration, packet::Packet},
};

pub struct EthernetDevice {}

impl DeviceImpl for EthernetDevice {
    fn on_packet_from_node(
        &mut self,
        ctx: &SimCtx,
        packet: Packet,
    ) -> Vec<(Duration, DeviceOutput)> {
        todo!()
    }

    fn on_packet_from_channel(
        &mut self,
        ctx: &SimCtx,
        packet: Packet,
    ) -> Vec<(Duration, DeviceOutput)> {
        todo!()
    }
}
