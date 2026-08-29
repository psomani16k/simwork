use std::ops::{Index, IndexMut};

use crate::core::util::{
    packet::Packet,
    size::{Size, SizeOf},
};

#[derive(Clone)]
pub enum PacketData {
    Data(Vec<u8>),
    Packet(Box<Packet>),
}

impl Into<Vec<u8>> for PacketData {
    fn into(self) -> Vec<u8> {
        match self {
            PacketData::Data(items) => items.clone(),
            PacketData::Packet(packet) => (*packet).into(),
        }
    }
}

impl SizeOf for PacketData {
    fn size(&self) -> Size {
        match self {
            PacketData::Data(d) => Size::from_bytes(d.len() as u32),
            PacketData::Packet(packet) => packet.size(),
        }
    }
}

impl Index<usize> for PacketData {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            PacketData::Data(items) => items.index(index),
            PacketData::Packet(packet) => (*packet).index(index),
        }
    }
}

impl IndexMut<usize> for PacketData {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self {
            PacketData::Data(items) => items.index_mut(index),
            PacketData::Packet(packet) => (*packet).index_mut(index),
        }
    }
}
