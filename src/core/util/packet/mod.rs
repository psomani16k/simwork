pub mod data;
pub mod header;
pub mod id;
pub mod trailer;

use std::ops::{Index, IndexMut};

use crate::core::util::{
    packet::{data::PacketData, header::Header, id::PacketId, trailer::Trailer},
    size::{Size, SizeOf},
};

#[derive(Clone)]
pub struct Packet {
    header: Header,
    data: PacketData,
    trailer: Trailer,
    id: PacketId,
}

impl Packet {
    pub fn from_raw_data(data: Vec<u8>, id: PacketId) -> Self {
        Self {
            header: Header::RawData,
            data: PacketData::Data(data),
            trailer: Trailer::None,
            id,
        }
    }

    pub fn unwrap(self) -> (Header, PacketData, Trailer) {
        (self.header, self.data, self.trailer)
    }

    pub fn id(&self) -> PacketId {
        self.id
    }

    pub fn peek_data(&self) -> &PacketData {
        &self.data
    }

    pub fn peek_header(&self) -> &Header {
        &self.header
    }

    pub fn peek_trailer(&self) -> &Trailer {
        &self.trailer
    }
}

impl Into<Vec<u8>> for Packet {
    fn into(self) -> Vec<u8> {
        let mut header: Vec<u8> = self.header.into();
        let mut data = self.data.into();
        let mut trailer: Vec<u8> = self.trailer.into();
        header.append(&mut data);
        header.append(&mut trailer);
        return header;
    }
}

impl SizeOf for Packet {
    fn size(&self) -> Size {
        let header_size = self.header.size();
        let data_size = self.data.size();
        let trailer_size = self.trailer.size();
        header_size + data_size + trailer_size
    }
}

impl Index<usize> for Packet {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        let header_len = self.header.size().as_bytes() as usize;
        if index < header_len {
            return self.header.index(index);
        }
        let data_len = self.data.size().as_bytes() as usize;
        if index < header_len + data_len {
            return self.data.index(index - header_len);
        }
        let trailer_len = self.trailer.size().as_bytes() as usize;
        if index < header_len + data_len + trailer_len {
            return self.trailer.index(index - header_len - data_len);
        }
        panic!(
            "index {} out of bounds for packet of size {} bytes",
            index,
            self.size().as_bytes()
        );
    }
}

impl IndexMut<usize> for Packet {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let header_len = self.header.size().as_bytes() as usize;
        let data_len = self.data.size().as_bytes() as usize;
        let trailer_len = self.trailer.size().as_bytes() as usize;
        if index < header_len {
            return self.header.index_mut(index);
        }
        if index < header_len + data_len {
            return self.data.index_mut(index - header_len);
        }
        if index < header_len + data_len + trailer_len {
            return self.trailer.index_mut(index - header_len - data_len);
        }
        panic!(
            "index {} out of bounds for packet of size {} bytes",
            index,
            self.size().as_bytes()
        );
    }
}

pub struct PacketBytes<'a> {
    packet: &'a Packet,
    pos: usize,
    len: usize,
}

impl<'a> Iterator for PacketBytes<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.len {
            return None;
        }
        let byte = self.packet[self.pos];
        self.pos += 1;
        Some(byte)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.len - self.pos;
        (remaining, Some(remaining))
    }
}

impl<'a> IntoIterator for &'a Packet {
    type Item = u8;
    type IntoIter = PacketBytes<'a>;

    fn into_iter(self) -> Self::IntoIter {
        PacketBytes {
            packet: self,
            pos: 0,
            len: self.size().as_bytes() as usize,
        }
    }
}

impl Packet {
    pub fn bytes(&self) -> PacketBytes<'_> {
        self.into_iter()
    }
}

pub trait Wrap<T> {
    fn wrap(self, header: T, id: PacketId) -> Self;
}
