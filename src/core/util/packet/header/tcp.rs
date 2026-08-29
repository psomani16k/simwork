//   0                   1                   2                   3
//   0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
//  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//  |          Source Port          |       Destination Port        |
//  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//  |                        Sequence Number                        |
//  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//  |                    Acknowledgment Number                      |
//  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//  |  Data |       |C|E|U|A|P|R|S|F|                               |
//  | Offset| Rsrvd |W|C|R|C|S|S|Y|I|            Window             |
//  |       |       |R|E|G|K|H|T|N|N|                               |
//  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//  |           Checksum            |         Urgent Pointer        |
//  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//  |                           [Options]                           |
//  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//  |                                                               :
//  :                             Data                              :
//  :                                                               |
//  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

use std::ops::{Index, IndexMut};

use crate::core::util::{
    address::Port,
    size::{Size, SizeOf},
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TcpHeader {
    source_port: [u8; 2],
    dest_port: [u8; 2],

    seq_number: [u8; 4],

    ack_number: [u8; 4],

    data_offset_reserved: u8,
    flags: u8,
    window: [u8; 2],

    checksum: [u8; 2],
    urgent_pointer: [u8; 2],

    options: TcpOptions,
}

impl TcpHeader {
    pub fn destination_port(&self) -> Port {
        Port::TCP(self.destination_port_raw())
    }

    pub fn destination_port_raw(&self) -> u16 {
        u16::from_be_bytes(self.dest_port)
    }
}

impl IndexMut<usize> for TcpHeader {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0..=1 => &mut self.source_port[index],
            2..=3 => &mut self.dest_port[index - 2],
            4..=7 => &mut self.seq_number[index - 4],
            8..=11 => &mut self.ack_number[index - 8],
            12 => &mut self.data_offset_reserved,
            13 => &mut self.flags,
            14..=15 => &mut self.window[index - 14],
            16..=17 => &mut self.checksum[index - 16],
            18..=19 => &mut self.urgent_pointer[index - 18],
            _ => self.options.index_mut(index - 20),
        }
    }
}

impl Index<usize> for TcpHeader {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0..=1 => &self.source_port[index],
            2..=3 => &self.dest_port[index - 2],
            4..=7 => &self.seq_number[index - 4],
            8..=11 => &self.ack_number[index - 8],
            12 => &self.data_offset_reserved,
            13 => &self.flags,
            14..=15 => &self.window[index - 14],
            16..=17 => &self.checksum[index - 16],
            18..=19 => &self.urgent_pointer[index - 18],
            _ => self.options.index(index - 20),
        }
    }
}

impl SizeOf for TcpHeader {
    fn size(&self) -> Size {
        Size::from_bytes(20) + self.options.size()
    }
}

impl Into<Vec<u8>> for TcpHeader {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(20 + self.options.len as usize);
        bytes.extend_from_slice(&self.source_port);
        bytes.extend_from_slice(&self.dest_port);
        bytes.extend_from_slice(&self.seq_number);
        bytes.extend_from_slice(&self.ack_number);
        bytes.push(self.data_offset_reserved);
        bytes.push(self.flags);
        bytes.extend_from_slice(&self.window);
        bytes.extend_from_slice(&self.checksum);
        bytes.extend_from_slice(&self.urgent_pointer);
        bytes.append(&mut self.options.into());
        bytes
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct TcpOptions {
    pub len: u8,
    pub buf: [u8; 40],
}

impl IndexMut<usize> for TcpOptions {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if (self.len as usize) <= index {
            panic!(
                "index {} out of bounds for tcp option of size {} bytes",
                index, self.len
            );
        }
        return &mut self.buf[index];
    }
}

impl Index<usize> for TcpOptions {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        if (self.len as usize) <= index {
            panic!(
                "index {} out of bounds for tcp option of size {} bytes",
                index, self.len
            );
        }
        return &self.buf[index];
    }
}

impl SizeOf for TcpOptions {
    fn size(&self) -> Size {
        Size::from_bytes(self.len as u32)
    }
}

impl Into<Vec<u8>> for TcpOptions {
    fn into(self) -> Vec<u8> {
        let actual = &self.buf[..self.len as usize];
        return actual.to_vec();
    }
}
