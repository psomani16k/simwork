#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Ipv4Address([u8; 4]);

impl Ipv4Address {
    pub fn new(octets: [u8; 4]) -> Self {
        Ipv4Address(octets)
    }

    pub fn octets(&self) -> [u8; 4] {
        self.0
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Ipv6Address([u8; 16]);

impl Ipv6Address {
    pub fn new(octets: [u8; 16]) -> Self {
        Ipv6Address(octets)
    }

    pub fn octets(&self) -> [u8; 16] {
        self.0
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum IpAddress {
    Ipv4(Ipv4Address),
    Ipv6(Ipv6Address),
}

/// The name a socket is reachable by, and the widest address vocabulary the
/// application layer is given: an application may name a peer, but nothing it
/// holds can say how that peer is reached.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Endpoint {
    ip: IpAddress,
    port: Port,
}

impl Endpoint {
    pub fn new(ip: IpAddress, port: Port) -> Self {
        Endpoint { ip, port }
    }

    pub fn ip(&self) -> IpAddress {
        self.ip
    }

    pub fn port(&self) -> Port {
        self.port
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Port {
    TCP(u16),
    UDP(u16),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct MacAddress([u8; 6]);

impl MacAddress {
    pub fn new(octets: [u8; 6]) -> Self {
        Self(octets)
    }

    pub fn octets(&self) -> [u8; 6] {
        self.0
    }
}
