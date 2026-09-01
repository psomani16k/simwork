//   0               1               2               3
//   0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7
//  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//  |Version|  IHL  |Type of Service|          Total Length         |
//  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//  |         Identification        |Flags|      Fragment Offset    |
//  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//  |  Time to Live |    Protocol   |         Header Checksum       |
//  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//  |                       Source Address                          |
//  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//  |                    Destination Address                        |
//  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//  |                    Options                    |    Padding    |
//  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

use std::ops::{Index, IndexMut};

use crate::core::util::{
    address::Ipv4Address,
    packet::{Packet, Wrap, data::PacketData, header::Header, id::PacketId, trailer::Trailer},
    size::{Size, SizeOf},
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ipv4Header {
    version_ihl: u8,
    type_of_service: u8,
    total_len: [u8; 2],

    identification: [u8; 2],
    flags_fragment_offset: [u8; 2],

    ttl: u8,
    protocol: u8,
    header_checksum: [u8; 2],

    source_addr: [u8; 4],

    dest_addr: [u8; 4],

    options: Ipv4Options,
}

impl Ipv4Header {
    pub fn source_addr(&self) -> Ipv4Address {
        Ipv4Address::new(self.source_addr)
    }
}

impl IndexMut<usize> for Ipv4Header {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.version_ihl,
            1 => &mut self.type_of_service,
            2..=3 => &mut self.total_len[index - 2],
            4..=5 => &mut self.identification[index - 4],
            6..=7 => &mut self.flags_fragment_offset[index - 6],
            8 => &mut self.ttl,
            9 => &mut self.protocol,
            10..=11 => &mut self.header_checksum[index - 10],
            12..=15 => &mut self.source_addr[index - 12],
            16..=19 => &mut self.dest_addr[index - 16],
            _ => self.options.index_mut(index - 20),
        }
    }
}

impl Index<usize> for Ipv4Header {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.version_ihl,
            1 => &self.type_of_service,
            2..=3 => &self.total_len[index - 2],
            4..=5 => &self.identification[index - 4],
            6..=7 => &self.flags_fragment_offset[index - 6],
            8 => &self.ttl,
            9 => &self.protocol,
            10..=11 => &self.header_checksum[index - 10],
            12..=15 => &self.source_addr[index - 12],
            16..=19 => &self.dest_addr[index - 16],
            _ => self.options.index(index - 20),
        }
    }
}

impl SizeOf for Ipv4Header {
    fn size(&self) -> Size {
        Size::from_bytes(20) + self.options.size()
    }
}

impl Wrap<Ipv4Header> for Packet {
    fn wrap(self, header: Ipv4Header, id: PacketId) -> Self {
        Self {
            header: Header::IPv4(header),
            data: PacketData::Packet(Box::new(self)),
            trailer: Trailer::None,
            id,
        }
    }
}

impl Into<Vec<u8>> for Ipv4Header {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(20 + self.options.len as usize);
        bytes.push(self.version_ihl);
        bytes.push(self.type_of_service);
        bytes.extend_from_slice(&self.total_len);
        bytes.extend_from_slice(&self.identification);
        bytes.extend_from_slice(&self.flags_fragment_offset);
        bytes.push(self.ttl);
        bytes.push(self.protocol);
        bytes.extend_from_slice(&self.header_checksum);
        bytes.extend_from_slice(&self.source_addr);
        bytes.extend_from_slice(&self.dest_addr);
        bytes.append(&mut self.options.into());
        bytes
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Ipv4Options {
    pub len: u8,
    pub buf: [u8; 40],
}

impl IndexMut<usize> for Ipv4Options {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if (self.len as usize) <= index {
            panic!(
                "index {} out of bounds for ipv4 option of size {} bytes",
                index, self.len
            );
        }
        return &mut self.buf[index];
    }
}

impl Index<usize> for Ipv4Options {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        if (self.len as usize) <= index {
            panic!(
                "index {} out of bounds for ipv4 option of size {} bytes",
                index, self.len
            );
        }
        return &self.buf[index];
    }
}

impl SizeOf for Ipv4Options {
    fn size(&self) -> Size {
        Size::from_bytes(self.len as u32)
    }
}

impl Into<Vec<u8>> for Ipv4Options {
    fn into(self) -> Vec<u8> {
        let actual = &self.buf[..self.len as usize];
        return actual.to_vec();
    }
}
