//   0               1               2               3
//   0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7
//  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//  |         Hardware Type         |         Protocol Type         |
//  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//  |   HW Addr Len  |     P.A.L    |           Operation           | // P.A.L: Protocol Address Length
//  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//  |          Sender Hardware Address (octets 1-4)                 |
//  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//  |  Sender HW Addr (octets 5-6)  |       S.P.A (octets 1-2)      | // S.P.A: Sender Protocol Address
//  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//  |       S.P.A (octets 3-4)      |       T.H.A (octets 2-1)      | // T.H.A: Target Hardware Address
//  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//  |          Target Hardware Address (THA)  (octets 3-6)          |
//  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//  |          Target Protocol Address (TPA)  (octets 1-4)          |
//  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

use std::ops::{Index, IndexMut};

use crate::core::util::{
    address::{Ipv4Address, MacAddress},
    packet::{
        Packet, Wrap,
        data::PacketData,
        header::{Header, ethernet::EtherType},
        id::PacketId,
        trailer::Trailer,
    },
    size::{Size, SizeOf},
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArpHeader {
    hardware_type: [u8; 2],
    protocol_type: [u8; 2],
    hardware_addr_len: u8,
    protocol_addr_len: u8,
    operation: [u8; 2],
    sender_hardware_addr: [u8; 6],
    sender_protocol_addr: [u8; 4],
    target_hardware_addr: [u8; 6],
    target_protocol_addr: [u8; 4],
}

impl ArpHeader {
    /// An ethernet/IPv4 ARP message: the only pairing this simulation speaks,
    /// so the hardware and protocol type fields are fixed here.
    pub fn new(
        operation: ArpOperation,
        sender_hardware_addr: MacAddress,
        sender_protocol_addr: Ipv4Address,
        target_hardware_addr: MacAddress,
        target_protocol_addr: Ipv4Address,
    ) -> Self {
        Self {
            hardware_type: HardwareType::Ethernet.into(),
            protocol_type: EtherType::Ipv4.into(),
            hardware_addr_len: 6,
            protocol_addr_len: 4,
            operation: operation.into(),
            sender_hardware_addr: sender_hardware_addr.octets(),
            sender_protocol_addr: sender_protocol_addr.octets(),
            target_hardware_addr: target_hardware_addr.octets(),
            target_protocol_addr: target_protocol_addr.octets(),
        }
    }

    pub fn hardware_type(&self) -> HardwareType {
        self.hardware_type.into()
    }

    pub fn protocol_type(&self) -> EtherType {
        self.protocol_type.into()
    }

    pub fn operation(&self) -> ArpOperation {
        self.operation.into()
    }

    pub fn sender_hardware_addr(&self) -> MacAddress {
        MacAddress::new(self.sender_hardware_addr)
    }

    pub fn sender_protocol_addr(&self) -> Ipv4Address {
        Ipv4Address::new(self.sender_protocol_addr)
    }

    pub fn target_hardware_addr(&self) -> MacAddress {
        MacAddress::new(self.target_hardware_addr)
    }

    pub fn target_protocol_addr(&self) -> Ipv4Address {
        Ipv4Address::new(self.target_protocol_addr)
    }
}

impl IndexMut<usize> for ArpHeader {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0..=1 => &mut self.hardware_type[index],
            2..=3 => &mut self.protocol_type[index - 2],
            4 => &mut self.hardware_addr_len,
            5 => &mut self.protocol_addr_len,
            6..=7 => &mut self.operation[index - 6],
            8..=13 => &mut self.sender_hardware_addr[index - 8],
            14..=17 => &mut self.sender_protocol_addr[index - 14],
            18..=23 => &mut self.target_hardware_addr[index - 18],
            24..=27 => &mut self.target_protocol_addr[index - 24],
            _ => {
                panic!(
                    "index {} out of bounds for arp header of size {} bytes",
                    index,
                    self.size()
                );
            }
        }
    }
}

impl Index<usize> for ArpHeader {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0..=1 => &self.hardware_type[index],
            2..=3 => &self.protocol_type[index - 2],
            4 => &self.hardware_addr_len,
            5 => &self.protocol_addr_len,
            6..=7 => &self.operation[index - 6],
            8..=13 => &self.sender_hardware_addr[index - 8],
            14..=17 => &self.sender_protocol_addr[index - 14],
            18..=23 => &self.target_hardware_addr[index - 18],
            24..=27 => &self.target_protocol_addr[index - 24],
            _ => {
                panic!(
                    "index {} out of bounds for arp header of size {} bytes",
                    index,
                    self.size()
                );
            }
        }
    }
}

impl SizeOf for ArpHeader {
    fn size(&self) -> Size {
        Size::from_bytes(28)
    }
}

impl Into<Vec<u8>> for ArpHeader {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(28);
        bytes.extend_from_slice(&self.hardware_type);
        bytes.extend_from_slice(&self.protocol_type);
        bytes.push(self.hardware_addr_len);
        bytes.push(self.protocol_addr_len);
        bytes.extend_from_slice(&self.operation);
        bytes.extend_from_slice(&self.sender_hardware_addr);
        bytes.extend_from_slice(&self.sender_protocol_addr);
        bytes.extend_from_slice(&self.target_hardware_addr);
        bytes.extend_from_slice(&self.target_protocol_addr);
        bytes
    }
}

impl Wrap<ArpHeader> for Packet {
    fn wrap(self, header: ArpHeader, id: PacketId) -> Self {
        // an arp message carries no payload of its own: whatever is wrapped is
        // expected to be empty, and the header is the whole message
        Self {
            header: Header::ARP(header),
            data: PacketData::Packet(Box::new(self)),
            trailer: Trailer::None,
            id,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum HardwareType {
    Unknown([u8; 2]),
    Ethernet,
}

impl Into<[u8; 2]> for HardwareType {
    fn into(self) -> [u8; 2] {
        match self {
            HardwareType::Ethernet => [0x00, 0x01],
            HardwareType::Unknown(v) => v,
        }
    }
}

impl From<[u8; 2]> for HardwareType {
    fn from(value: [u8; 2]) -> Self {
        match value {
            [0x00, 0x01] => HardwareType::Ethernet,
            v => HardwareType::Unknown(v),
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArpOperation {
    Unknown([u8; 2]),
    Request,
    Reply,
}

impl Into<[u8; 2]> for ArpOperation {
    fn into(self) -> [u8; 2] {
        match self {
            ArpOperation::Request => [0x00, 0x01],
            ArpOperation::Reply => [0x00, 0x02],
            ArpOperation::Unknown(v) => v,
        }
    }
}

impl From<[u8; 2]> for ArpOperation {
    fn from(value: [u8; 2]) -> Self {
        match value {
            [0x00, 0x01] => ArpOperation::Request,
            [0x00, 0x02] => ArpOperation::Reply,
            v => ArpOperation::Unknown(v),
        }
    }
}
