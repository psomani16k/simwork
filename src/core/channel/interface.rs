use crate::core::{
    event::channel_events::ChannelOutput,
    sim::ctx::SimCtx,
    util::{address::MacAddress, duration::Duration, packet::Packet},
};

pub trait ChannelImpl {
    fn on_packet_from_device(
        &mut self,
        ctx: &SimCtx,
        source: MacAddress,
        packet: Packet,
    ) -> Vec<(Duration, ChannelOutput)>;
}
