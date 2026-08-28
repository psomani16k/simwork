use etherparse::{Ipv4Header, Ipv6Header, TcpHeader, UdpHeader};

use crate::core::util::{
    id::IdGenerator,
    size::{Size, SizeOf},
};

#[derive(Clone)]
pub struct Packet {
    header: Header,
    data: PacketData,
    id: PacketId,
}

impl Packet {
    pub fn new(header: Header, data: PacketData, id: PacketId) -> Self {
        Packet { header, data, id }
    }

    pub fn dismantle(self) -> (Header, PacketData) {
        (self.header, self.data)
    }

    pub fn get_id(&self) -> PacketId {
        self.id
    }

    pub fn peek_data(&self) -> &PacketData {
        &self.data
    }

    pub fn peek_header(&self) -> &Header {
        &self.header
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut header = self.header.to_bytes();
        let mut data = self.data.to_bytes();
        header.append(&mut data);
        return header;
    }
}

impl SizeOf for Packet {
    fn size_in_bytes(&self) -> Size {
        let header_size = self.header.size_in_bytes();
        let data_size = self.data.size_in_bytes();
        header_size + data_size
    }
}

#[derive(Clone)]
pub enum Header {
    RawData,
    TCP(TcpHeader),
    UDP(UdpHeader),
    IPv4(Ipv4Header),
    IPv6(Ipv6Header),
}

impl Header {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Header::RawData => vec![],
            Header::TCP(tcp_header) => tcp_header.to_bytes().to_vec(),
            Header::UDP(udp_header) => udp_header.to_bytes().to_vec(),
            Header::IPv4(ipv4_header) => ipv4_header.to_bytes().to_vec(),
            Header::IPv6(ipv6_header) => ipv6_header.to_bytes().to_vec(),
        }
    }
}

impl SizeOf for Header {
    fn size_in_bytes(&self) -> Size {
        match self {
            Header::RawData => Size::ZERO,
            Header::TCP(tcp_header) => Size::from_bytes(tcp_header.header_len_u16() as u32),
            Header::UDP(udp_header) => Size::from_bytes(udp_header.header_len_u16() as u32),
            Header::IPv4(ipv4_header) => Size::from_bytes(ipv4_header.header_len() as u32),
            Header::IPv6(ipv6_header) => Size::from_bytes(ipv6_header.header_len() as u32),
        }
    }
}

#[derive(Clone)]
pub enum PacketData {
    Data(Vec<u8>),
    Packet(Box<Packet>),
}

impl PacketData {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            PacketData::Data(items) => items.clone(),
            PacketData::Packet(packet) => packet.to_bytes(),
        }
    }
}

impl SizeOf for PacketData {
    fn size_in_bytes(&self) -> Size {
        match self {
            PacketData::Data(d) => Size::from_bytes(d.len() as u32),
            PacketData::Packet(packet) => packet.size_in_bytes(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PacketId(u64);

impl IdGenerator {
    pub fn new_packet_id(&mut self) -> PacketId {
        let id = self.get_id();
        PacketId(id)
    }
}
