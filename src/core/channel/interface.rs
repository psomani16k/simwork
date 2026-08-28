use std::collections::HashSet;

use crate::core::{
    device::id::DeviceId,
    event::channel_events::ChannelOutput,
    sim::ctx::SimCtx,
    util::{duration::Duration, packet::Packet},
};

pub trait ChannelImpl {
    fn on_packet_from_device(
        &mut self,
        ctx: &SimCtx,
        source: DeviceId,
        packet: Packet,
    ) -> Vec<(Duration, ChannelOutput)>;
}
