//   0               1               2               3
//   0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7
//  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//  |                     Preamble (octets 1-4)                     | // ignored here
//  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//  |             Preamble (octets 5-7)             |      SFD      | // ignored here
//  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//  |             Destination MAC Address (octets 1-4)              |
//  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//  |     Dst MAC (octets 5-6)      |     Src MAC (octets 1-2)      |
//  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//  |                Source MAC Address (octets 3-6)                |
//  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//  |           EtherType           |      Payload starts here      |
//  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//  |                                                               |
//  :          Payload / MAC Client Data (46-1500 octets)           :
//  |                                                               |
//  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//  |                  Frame Check Sequence (FCS)                   |
//  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

use std::ops::{Index, IndexMut};

use crate::core::util::{
    address::MacAddress,
    size::{Size, SizeOf},
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct EthernetHeader {
    dest_mac: [u8; 6],
    source_mac: [u8; 6],
    ether_type: [u8; 2],
}

impl EthernetHeader {
    pub fn destination_mac(&self) -> MacAddress {
        MacAddress::new(self.dest_mac)
    }

    pub fn source_mac(&self) -> MacAddress {
        MacAddress::new(self.source_mac)
    }

    pub fn ether_type(&self) -> u16 {
        u16::from_be_bytes(self.ether_type)
    }
}

impl IndexMut<usize> for EthernetHeader {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0..=5 => &mut self.dest_mac[index],
            6..=11 => &mut self.source_mac[index - 6],
            12..=13 => &mut self.ether_type[index - 12],
            _ => {
                panic!(
                    "index {} out of bounds for ethernet header of size {} bytes",
                    index,
                    self.size()
                );
            }
        }
    }
}

impl Index<usize> for EthernetHeader {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0..=5 => &self.dest_mac[index],
            6..=11 => &self.source_mac[index - 6],
            12..=13 => &self.ether_type[index - 12],
            _ => {
                panic!(
                    "index {} out of bounds for ethernet header of size {} bytes",
                    index,
                    self.size()
                );
            }
        }
    }
}

impl SizeOf for EthernetHeader {
    fn size(&self) -> Size {
        Size::from_bytes(14)
    }
}

impl Into<Vec<u8>> for EthernetHeader {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(14);
        bytes.extend_from_slice(&self.dest_mac);
        bytes.extend_from_slice(&self.source_mac);
        bytes.extend_from_slice(&self.ether_type);
        bytes
    }
}
