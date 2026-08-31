//   0      7 8     15 16    23 24    31
//  +--------+--------+--------+--------+
//  |     Source      |   Destination   |
//  |      Port       |      Port       |
//  +--------+--------+--------+--------+
//  |                 |                 |
//  |     Length      |    Checksum     |
//  +--------+--------+--------+--------+
//  |                                   |
//  :          data octets ...          :
//  |                                   |
//  +-----------------------------------+

use std::ops::{Index, IndexMut};

use crate::core::util::{
    address::Port,
    packet::{Packet, Wrap, data::PacketData, header::Header, id::PacketId, trailer::Trailer},
    size::{Size, SizeOf},
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct UdpHeader {
    source_port: [u8; 2],
    dest_port: [u8; 2],

    len: [u8; 2],
    checksum: [u8; 2],
}

impl UdpHeader {
    pub fn destination_port(&self) -> Port {
        Port::UDP(self.destination_port_raw())
    }

    pub fn destination_port_raw(&self) -> u16 {
        u16::from_be_bytes(self.dest_port)
    }
}

impl IndexMut<usize> for UdpHeader {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0..=1 => &mut self.source_port[index],
            2..=3 => &mut self.dest_port[index - 2],
            4..=5 => &mut self.len[index - 4],
            6..=7 => &mut self.checksum[index - 6],
            _ => {
                panic!(
                    "index {} out of bounds for udp header of size {} bytes",
                    index,
                    self.size()
                );
            }
        }
    }
}

impl Index<usize> for UdpHeader {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0..=1 => &self.source_port[index],
            2..=3 => &self.dest_port[index - 2],
            4..=5 => &self.len[index - 4],
            6..=7 => &self.checksum[index - 6],
            _ => {
                panic!(
                    "index {} out of bounds for udp header of size {} bytes",
                    index,
                    self.size()
                );
            }
        }
    }
}

impl SizeOf for UdpHeader {
    fn size(&self) -> Size {
        Size::from_bytes(8)
    }
}

impl Into<Vec<u8>> for UdpHeader {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(8);
        bytes.extend_from_slice(&self.source_port);
        bytes.extend_from_slice(&self.dest_port);
        bytes.extend_from_slice(&self.len);
        bytes.extend_from_slice(&self.checksum);
        bytes
    }
}

impl Wrap<UdpHeader> for Packet {
    fn wrap(self, header: UdpHeader, id: PacketId) -> Self {
        Self {
            header: Header::UDP(header),
            data: PacketData::Packet(Box::new(self)),
            trailer: Trailer::None,
            id,
        }
    }
}
