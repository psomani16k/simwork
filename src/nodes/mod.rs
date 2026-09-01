pub mod arp;
pub mod ipv4;
pub mod ipv6;

use crate::{
    core::{
        event::node_events::NodeOutput,
        node::interface::NodeImpl,
        sim::ctx::SimCtx,
        util::{
            address::IpAddress,
            duration::Duration,
            packet::{Packet, header::Header},
        },
    },
    nodes::{arp::ArpNode, ipv4::Ipv4Node, ipv6::Ipv6Node},
};

pub struct DemuxNode {
    ipv4_router: Option<Ipv4Node>,
    ipv6_router: Option<Ipv6Node>,
    arp_node: Option<ArpNode>,
}

impl NodeImpl for DemuxNode {
    fn on_packet_from_socket(
        &mut self,
        ctx: &SimCtx,
        packet: Packet,
        dest_ip: IpAddress,
    ) -> Vec<(Duration, NodeOutput)> {
        match dest_ip {
            IpAddress::Ipv4(ipv4_address) => {
                if let Some(ipv4_impl) = self.ipv4_router.as_mut() {
                    ipv4_impl.on_packet_from_socket(ctx, packet, ipv4_address)
                } else {
                    vec![]
                }
            }
            IpAddress::Ipv6(ipv6_address) => {
                if let Some(ipv6_impl) = self.ipv6_router.as_mut() {
                    ipv6_impl.on_packet_from_socket(ctx, packet, ipv6_address)
                } else {
                    vec![]
                }
            }
        }
    }

    fn on_packet_from_device(
        &mut self,
        ctx: &SimCtx,
        packet: Packet,
    ) -> Vec<(Duration, NodeOutput)> {
        match packet.peek_header() {
            Header::IPv4(_) => {
                if let Some(ipv4_impl) = self.ipv4_router.as_mut() {
                    ipv4_impl.on_packet_from_device(ctx, packet)
                } else {
                    vec![]
                }
            }
            Header::IPv6(_) => {
                if let Some(ipv6_impl) = self.ipv6_router.as_mut() {
                    ipv6_impl.on_packet_from_device(ctx, packet)
                } else {
                    vec![]
                }
            }
            Header::ARP(_) => {
                if let Some(arp_node) = self.arp_node.as_mut() {
                    arp_node.on_packet_from_device(ctx, packet)
                } else {
                    vec![]
                }
            }
            Header::Ethernet(_) => {
                vec![]
            }
            Header::RawData => {
                vec![]
            }
            Header::TCP(_) => {
                vec![]
            }
            Header::UDP(_) => {
                vec![]
            }
        }
    }
}
