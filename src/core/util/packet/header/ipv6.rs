//   0               1               2               3
//   0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7
//  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//  |Version| Traffic Class |           Flow Label                  |
//  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//  |         Payload Length        |  Next Header  |   Hop Limit   |
//  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//  |                                                               |
//  +                                                               +
//  |                                                               |
//  +                         Source Address                        +
//  |                                                               |
//  +                                                               +
//  |                                                               |
//  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//  |                                                               |
//  +                                                               +
//  |                                                               |
//  +                      Destination Address                      +
//  |                                                               |
//  +                                                               +
//  |                                                               |
//  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

use std::ops::{Index, IndexMut};

use crate::core::util::{
    address::Ipv6Address,
    size::{Size, SizeOf},
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ipv6Header {
    version_traffic_class_flow_label: [u8; 4],

    payload_len: [u8; 2],
    next_header: u8,
    hop_limit: u8,

    source_addr: [u8; 16],
    dest_addr: [u8; 16],
}

impl Ipv6Header {
    pub fn source_addr(&self) -> Ipv6Address {
        Ipv6Address::new(self.source_addr)
    }
}

impl IndexMut<usize> for Ipv6Header {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0..=3 => &mut self.version_traffic_class_flow_label[index],
            4..=5 => &mut self.payload_len[index - 4],
            6 => &mut self.next_header,
            7 => &mut self.hop_limit,
            8..=23 => &mut self.source_addr[index - 8],
            24..=39 => &mut self.dest_addr[index - 24],
            _ => {
                panic!(
                    "index {} out of bounds for ipv6 header of size {} bytes",
                    index,
                    self.size()
                );
            }
        }
    }
}

impl Index<usize> for Ipv6Header {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0..=3 => &self.version_traffic_class_flow_label[index],
            4..=5 => &self.payload_len[index - 4],
            6 => &self.next_header,
            7 => &self.hop_limit,
            8..=23 => &self.source_addr[index - 8],
            24..=39 => &self.dest_addr[index - 24],
            _ => {
                panic!(
                    "index {} out of bounds for ipv6 header of size {} bytes",
                    index,
                    self.size()
                );
            }
        }
    }
}

impl SizeOf for Ipv6Header {
    fn size(&self) -> Size {
        Size::from_bytes(40)
    }
}

impl Into<Vec<u8>> for Ipv6Header {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(40);
        bytes.extend_from_slice(&self.version_traffic_class_flow_label);
        bytes.extend_from_slice(&self.payload_len);
        bytes.push(self.next_header);
        bytes.push(self.hop_limit);
        bytes.extend_from_slice(&self.source_addr);
        bytes.extend_from_slice(&self.dest_addr);
        bytes
    }
}
