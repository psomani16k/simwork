use crate::core::{
    event::node_events::NodeOutput,
    sim::ctx::SimCtx,
    util::{address::IpAddress, duration::Duration, packet::Packet},
};

pub trait NodeImpl {
    fn on_packet_from_socket(
        &mut self,
        ctx: &SimCtx,
        packet: Packet,
        dst_ip: IpAddress,
    ) -> Vec<(Duration, NodeOutput)>;

    fn on_packet_from_device(
        &mut self,
        ctx: &SimCtx,
        packet: Packet,
    ) -> Vec<(Duration, NodeOutput)>;
}
