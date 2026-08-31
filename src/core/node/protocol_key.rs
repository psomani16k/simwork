use crate::core::util::packet::header::ethernet::EtherType;

pub enum NodeProtocolKey {
    Ipv4,
    Ipv6,
    Arp,
}

impl Into<EtherType> for NodeProtocolKey {
    fn into(self) -> EtherType {
        match self {
            NodeProtocolKey::Ipv4 => EtherType::Ipv4,
            NodeProtocolKey::Ipv6 => EtherType::Ipv6,
            NodeProtocolKey::Arp => EtherType::Arp,
        }
    }
}
