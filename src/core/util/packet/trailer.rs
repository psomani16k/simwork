use std::ops::{Index, IndexMut};

use crate::core::util::size::{Size, SizeOf};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Trailer {
    None,
    EthernetTrailer(EthernetPaddingFcs),
}

impl Into<Vec<u8>> for Trailer {
    fn into(self) -> Vec<u8> {
        match self {
            Trailer::None => vec![],
            Trailer::EthernetTrailer(trailer) => trailer.into(),
        }
    }
}

impl SizeOf for Trailer {
    fn size(&self) -> Size {
        match self {
            Trailer::None => Size::ZERO,
            Trailer::EthernetTrailer(trailer) => trailer.size(),
        }
    }
}

impl Index<usize> for Trailer {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Trailer::None => panic!("trying to index into a trailer of size 0"),
            Trailer::EthernetTrailer(trailer) => trailer.index(index),
        }
    }
}

impl IndexMut<usize> for Trailer {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self {
            Trailer::None => panic!("trying to index into a trailer of size 0"),
            Trailer::EthernetTrailer(trailer) => trailer.index_mut(index),
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct EthernetPaddingFcs {
    padding_len: u8,
    padding: [u8; 46],
    fcs: Option<[u8; 4]>,
}

impl EthernetPaddingFcs {
    /// Zero-filled padding, no FCS yet: the FCS is set after the crc is
    /// computed over the frame, so the padding is covered by it.
    pub fn new(padding_len: u8) -> Self {
        assert!(padding_len <= 46, "ethernet padding is at most 46 bytes");
        Self {
            padding_len,
            padding: [0; 46],
            fcs: None,
        }
    }

    pub fn set_fcs(&mut self, fcs: [u8; 4]) {
        self.fcs = Some(fcs);
    }
}

impl Into<Vec<u8>> for EthernetPaddingFcs {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.size().as_bytes() as usize);
        bytes.extend_from_slice(&self.padding[..self.padding_len as usize]);
        if let Some(fcs) = self.fcs {
            bytes.extend_from_slice(&fcs);
        }
        bytes
    }
}

impl SizeOf for EthernetPaddingFcs {
    fn size(&self) -> Size {
        let fcs_len = if self.fcs.is_some() { 4 } else { 0 };
        Size::from_bytes(self.padding_len as u32 + fcs_len)
    }
}

impl Index<usize> for EthernetPaddingFcs {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        let padding_len = self.padding_len as usize;
        if index < padding_len {
            return &self.padding[index];
        }
        if let Some(fcs) = &self.fcs {
            if index < padding_len + 4 {
                return &fcs[index - padding_len];
            }
        }
        panic!(
            "index {} out of bounds for ethernet trailer of size {} bytes",
            index,
            self.size().as_bytes()
        );
    }
}

impl IndexMut<usize> for EthernetPaddingFcs {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        // computed before borrowing: the returned reference keeps `self`
        // borrowed, so the panic below cannot touch it
        let len = self.size().as_bytes();
        let padding_len = self.padding_len as usize;
        if index < padding_len {
            return &mut self.padding[index];
        }
        if let Some(fcs) = &mut self.fcs {
            if index < padding_len + 4 {
                return &mut fcs[index - padding_len];
            }
        }
        panic!(
            "index {} out of bounds for ethernet trailer of size {} bytes",
            index, len
        );
    }
}
