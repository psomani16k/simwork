use crate::core::util::packet::Packet;

pub enum NodeToDevice {
    Send(Packet)
}

pub enum DeviceOutput {}
