use crate::core::{event::node_events::NodeOutput, node::ctx::NodeCtx, packet::Packet, util::duration::Duration};


pub trait NodeImpl {
    fn on_packet_from_socket(
        &mut self,
        ctx: NodeCtx,
        packet: Packet,
    ) -> Vec<(Duration, NodeOutput)>;

    fn on_packet_from_device(
        &mut self,
        ctx: NodeCtx,
        packet: Packet,
    ) -> Vec<(Duration, NodeOutput)>;
}
