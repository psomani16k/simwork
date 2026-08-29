pub mod ipv4;
pub mod ipv6;
pub mod tcp;
pub mod udp;

use std::ops::{Index, IndexMut};

use crate::core::util::{
    packet::header::{ipv4::Ipv4Header, ipv6::Ipv6Header, tcp::TcpHeader, udp::UdpHeader},
    size::{Size, SizeOf},
};

#[derive(Clone)]
pub enum Header {
    RawData,
    TCP(TcpHeader),
    UDP(UdpHeader),
    IPv4(Ipv4Header),
    IPv6(Ipv6Header),
}

impl Into<Vec<u8>> for Header {
    fn into(self) -> Vec<u8> {
        match self {
            Header::RawData => vec![],
            Header::TCP(tcp_header) => tcp_header.into(),
            Header::UDP(udp_header) => udp_header.into(),
            Header::IPv4(ipv4_header) => ipv4_header.into(),
            Header::IPv6(ipv6_header) => ipv6_header.into(),
        }
    }
}

impl SizeOf for Header {
    fn size(&self) -> Size {
        match self {
            Header::RawData => Size::ZERO,
            Header::TCP(tcp_header) => tcp_header.size(),
            Header::UDP(udp_header) => udp_header.size(),
            Header::IPv4(ipv4_header) => ipv4_header.size(),
            Header::IPv6(ipv6_header) => ipv6_header.size(),
        }
    }
}

impl Index<usize> for Header {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Header::RawData => panic!("trying to index into a header of size 0"),
            Header::TCP(tcp_header) => tcp_header.index(index),
            Header::UDP(udp_header) => udp_header.index(index),
            Header::IPv4(ipv4_header) => ipv4_header.index(index),
            Header::IPv6(ipv6_header) => ipv6_header.index(index),
        }
    }
}

impl IndexMut<usize> for Header {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self {
            Header::RawData => panic!("trying to index into a header of size 0"),
            Header::TCP(tcp_header) => tcp_header.index_mut(index),
            Header::UDP(udp_header) => udp_header.index_mut(index),
            Header::IPv4(ipv4_header) => ipv4_header.index_mut(index),
            Header::IPv6(ipv6_header) => ipv6_header.index_mut(index),
        }
    }
}
