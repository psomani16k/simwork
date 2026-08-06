pub struct Ipv4Address([u8; 4]);

pub struct Ipv6Address([u8; 16]);

pub enum IpAddress {
    Ipv4(Ipv4Address),
    Ipv6(Ipv6Address),
}
