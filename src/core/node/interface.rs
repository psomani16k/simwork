use crate::core::{
    event::node_events::NodeOutput,
    node::ctx::NodeCtx,
    util::{address::IpAddress, duration::Duration, packet::Packet},
};

pub trait NodeImpl {
    fn on_packet_from_socket(
        &mut self,
        ctx: NodeCtx,
        packet: Packet,
        dst_ip: IpAddress,
    ) -> Vec<(Duration, NodeOutput)>;

    fn on_packet_from_device(
        &mut self,
        ctx: NodeCtx,
        packet: Packet,
    ) -> Vec<(Duration, NodeOutput)>;
}
