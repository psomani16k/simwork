pub mod data;
pub mod header;
pub mod id;
pub mod trailer;

use std::ops::{Index, IndexMut};

use crate::core::util::{
    packet::{
        data::PacketData,
        header::{
            Header, ethernet::EthernetHeader, ipv4::Ipv4Header, ipv6::Ipv6Header, tcp::TcpHeader,
            udp::UdpHeader,
        },
        id::PacketId,
        trailer::Trailer,
    },
    size::{Size, SizeOf},
};

#[derive(Clone)]
pub struct Packet {
    header: Header,
    data: PacketData,
    trailer: Trailer,
    id: PacketId,
}

impl Packet {
    pub fn from_raw_data(data: Vec<u8>, id: PacketId) -> Self {
        Self {
            header: Header::RawData,
            data: PacketData::Data(data),
            trailer: Trailer::None,
            id,
        }
    }

    pub fn wrap_tcp(self, header: TcpHeader, id: PacketId) -> Self {
        Self {
            header: Header::TCP(header),
            data: PacketData::Packet(Box::new(self)),
            trailer: Trailer::None,
            id,
        }
    }

    pub fn wrap_udp(self, header: UdpHeader, id: PacketId) -> Self {
        Self {
            header: Header::UDP(header),
            data: PacketData::Packet(Box::new(self)),
            trailer: Trailer::None,
            id,
        }
    }

    pub fn wrap_ipv4(self, header: Ipv4Header, id: PacketId) -> Self {
        Self {
            header: Header::IPv4(header),
            data: PacketData::Packet(Box::new(self)),
            trailer: Trailer::None,
            id,
        }
    }

    pub fn wrap_ipv6(self, header: Ipv6Header, id: PacketId) -> Self {
        Self {
            header: Header::IPv6(header),
            data: PacketData::Packet(Box::new(self)),
            trailer: Trailer::None,
            id,
        }
    }

    pub fn wrap_ethernet(self, header: EthernetHeader, fcs: [u8; 4], id: PacketId) -> Self {
        Self {
            header: Header::Ethernet(header),
            data: PacketData::Packet(Box::new(self)),
            trailer: Trailer::EthernetFcs(fcs),
            id,
        }
    }

    pub fn unwrap(self) -> (Header, PacketData, Trailer) {
        (self.header, self.data, self.trailer)
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

    pub fn peek_trailer(&self) -> &Trailer {
        &self.trailer
    }
}

impl Into<Vec<u8>> for Packet {
    fn into(self) -> Vec<u8> {
        let mut header: Vec<u8> = self.header.into();
        let mut data = self.data.into();
        let mut trailer: Vec<u8> = self.trailer.into();
        header.append(&mut data);
        header.append(&mut trailer);
        return header;
    }
}

impl SizeOf for Packet {
    fn size(&self) -> Size {
        let header_size = self.header.size();
        let data_size = self.data.size();
        let trailer_size = self.trailer.size();
        header_size + data_size + trailer_size
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
        let trailer_len = self.trailer.size().as_bytes() as usize;
        if index < header_len + data_len + trailer_len {
            return self.trailer.index(index - header_len - data_len);
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
        let trailer_len = self.trailer.size().as_bytes() as usize;
        if index < header_len {
            return self.header.index_mut(index);
        }
        if index < header_len + data_len {
            return self.data.index_mut(index - header_len);
        }
        if index < header_len + data_len + trailer_len {
            return self.trailer.index_mut(index - header_len - data_len);
        }
        panic!(
            "index {} out of bounds for packet of size {} bytes",
            index,
            self.size().as_bytes()
        );
    }
}
