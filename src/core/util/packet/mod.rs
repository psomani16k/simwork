use std::ops::{Index, IndexMut};

use crate::core::util::{
    packet::{
        data::PacketData,
        header::{Header, ipv4::Ipv4Header, ipv6::Ipv6Header, tcp::TcpHeader, udp::UdpHeader},
        id::PacketId,
    },
    size::{Size, SizeOf},
};

pub mod data;
pub mod header;
pub mod id;

#[derive(Clone)]
pub struct Packet {
    header: Header,
    data: PacketData,
    id: PacketId,
}

impl Packet {
    pub fn from_raw_data(data: Vec<u8>, id: PacketId) -> Self {
        Self {
            header: Header::RawData,
            data: PacketData::Data(data),
            id,
        }
    }

    pub fn wrap_tcp(self, header: TcpHeader, id: PacketId) -> Self {
        Self {
            header: Header::TCP(header),
            data: PacketData::Packet(Box::new(self)),
            id,
        }
    }

    pub fn wrap_udp(self, header: UdpHeader, id: PacketId) -> Self {
        Self {
            header: Header::UDP(header),
            data: PacketData::Packet(Box::new(self)),
            id,
        }
    }

    pub fn wrap_ipv4(self, header: Ipv4Header, id: PacketId) -> Self {
        Self {
            header: Header::IPv4(header),
            data: PacketData::Packet(Box::new(self)),
            id,
        }
    }

    pub fn wrap_ipv6(self, header: Ipv6Header, id: PacketId) -> Self {
        Self {
            header: Header::IPv6(header),
            data: PacketData::Packet(Box::new(self)),
            id,
        }
    }

    pub fn unwrap(self) -> (Header, PacketData) {
        (self.header, self.data)
    }

    pub fn id(&self) -> PacketId {
        self.id
    }

    pub fn peek_data(&self) -> &PacketData {
        &self.data
    }

    pub fn peek_header(&self) -> &Header {
        &self.header
    }
}

impl Into<Vec<u8>> for Packet {
    fn into(self) -> Vec<u8> {
        let mut header: Vec<u8> = self.header.into();
        let mut data = self.data.into();
        header.append(&mut data);
        return header;
    }
}

impl SizeOf for Packet {
    fn size(&self) -> Size {
        let header_size = self.header.size();
        let data_size = self.data.size();
        header_size + data_size
    }
}

impl Index<usize> for Packet {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        let header_len = self.header.size().as_bytes() as usize;
        if index < header_len {
            return self.header.index(index);
        }
        let data_len = self.data.size().as_bytes() as usize;
        if index < header_len + data_len {
            return self.data.index(index - header_len);
        }
        panic!(
            "index {} out of bounds for packet of size {} bytes",
            index,
            self.size().as_bytes()
        );
    }
}

impl IndexMut<usize> for Packet {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let header_len = self.header.size().as_bytes() as usize;
        let data_len = self.data.size().as_bytes() as usize;
        if index < header_len {
            return self.header.index_mut(index);
        }
        if index < header_len + data_len {
            return self.data.index_mut(index - header_len);
        }
        panic!(
            "index {} out of bounds for packet of size {} bytes",
            index,
            self.size().as_bytes()
        );
    }
}
