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
//  |                  Frame Check Sequence (FCS)                   | // check ./../trailer.rs
//  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

use std::ops::{Index, IndexMut};

use crate::core::util::{
    address::MacAddress,
    packet::{Packet, Wrap, data::PacketData, header::Header, id::PacketId, trailer::{EthernetPaddingFcs, Trailer}},
    size::{Size, SizeOf},
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct EthernetHeader {
    dest_mac: [u8; 6],
    source_mac: [u8; 6],
    ether_type: [u8; 2],
}

impl EthernetHeader {
    pub fn new(source_mac: MacAddress, dest_mac: MacAddress, ether_type: EtherType) -> Self {
        Self {
            dest_mac: dest_mac.octets(),
            source_mac: source_mac.octets(),
            ether_type: ether_type.into(),
        }
    }

    pub fn destination_mac(&self) -> MacAddress {
        MacAddress::new(self.dest_mac)
    }

    pub fn source_mac(&self) -> MacAddress {
        MacAddress::new(self.source_mac)
    }

    pub fn ether_type(&self) -> EtherType {
        self.ether_type.into()
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

impl Wrap<EthernetHeader> for Packet {
    fn wrap(self, header: EthernetHeader, id: PacketId) -> Self {
        // payloads under the 46-byte minimum are zero-padded up to it
        let padding_len = 46u64.saturating_sub(self.size().as_bytes()) as u8;
        let mut trailer = EthernetPaddingFcs::new(padding_len);
        let mut header_packet = Self {
            header: Header::Ethernet(header),
            data: PacketData::Packet(Box::new(self)),
            // fcs not set yet: the crc below covers header, payload and padding
            trailer: Trailer::EthernetTrailer(trailer),
            id,
        };
        let fcs = crc32(&header_packet).to_le_bytes();
        trailer.set_fcs(fcs);
        header_packet.trailer = Trailer::EthernetTrailer(trailer);
        header_packet
    }
}

// Reflected CRC-32 (IEEE 802.3): divisor 0x04C11DB7, bit-reversed to
// 0xEDB88320 because ethernet sends each byte LSB-first; init and final XOR
// are 0xFFFFFFFF. Covers dest MAC through end of payload; the low byte of the
// result goes on the wire first, hence to_le_bytes() at the call site.
pub fn crc32(packet: &Packet) -> u32 {
    let padding = 64u64.saturating_sub(packet.size().as_bytes());
    if packet.size().as_bytes() < 64 {}
    let mut crc: u32 = 0xFFFF_FFFF;
    for byte in packet.bytes() {
        crc ^= byte as u32;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xEDB8_8320;
            } else {
                crc >>= 1;
            }
        }
    }
    !crc
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum EtherType {
    Unknown([u8; 2]),
    Ipv4,
    Ipv6,
    Arp,
}

impl Into<[u8; 2]> for EtherType {
    fn into(self) -> [u8; 2] {
        match self {
            EtherType::Ipv4 => [0x08, 0x00],
            EtherType::Ipv6 => [0x86, 0xDD],
            EtherType::Arp => [0x08, 0x06],
            EtherType::Unknown(v) => v,
        }
    }
}

impl From<[u8; 2]> for EtherType {
    fn from(value: [u8; 2]) -> Self {
        match value {
            [0x08, 0x00] => EtherType::Ipv4,
            [0x86, 0xDD] => EtherType::Ipv6,
            [0x08, 0x06] => EtherType::Arp,
            v => EtherType::Unknown(v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::crc32;
    use crate::core::util::{id::IdGenerator, packet::Packet};

    #[test]
    fn crc32_check_value() {
        // standard CRC-32/ISO-HDLC test vector; RawData header and None
        // trailer are both zero-sized, so only the data bytes are covered
        let id = IdGenerator::new().new_packet_id();
        let packet = Packet::from_raw_data(b"123456789".to_vec(), id);
        assert_eq!(crc32(&packet), 0xCBF43926);
    }
}
