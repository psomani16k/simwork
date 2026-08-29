use crate::core::{
    event::node_events::NodeOutput,
    sim::ctx::SimCtx,
    util::{address::Ipv6Address, duration::Duration, packet::Packet},
};

pub struct Ipv6Node {}

impl Ipv6Node {
    pub fn on_packet_from_socket(
        &mut self,
        ctx: &SimCtx,
        packet: Packet,
        dest_ip: Ipv6Address,
    ) -> Vec<(Duration, NodeOutput)> {
        todo!()
    }

    pub fn on_packet_from_device(
        &mut self,
        ctx: &SimCtx,
        packet: Packet,
    ) -> Vec<(Duration, NodeOutput)> {
        todo!()
    }
}
