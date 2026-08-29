use std::ops::{Index, IndexMut};

use crate::core::util::size::{Size, SizeOf};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Trailer {
    None,
    EthernetFcs([u8; 4]),
}

impl Into<Vec<u8>> for Trailer {
    fn into(self) -> Vec<u8> {
        match self {
            Trailer::None => vec![],
            Trailer::EthernetFcs(fcs) => fcs.to_vec(),
        }
    }
}

impl SizeOf for Trailer {
    fn size(&self) -> Size {
        match self {
            Trailer::None => Size::ZERO,
            Trailer::EthernetFcs(_) => Size::from_bytes(4),
        }
    }
}

impl Index<usize> for Trailer {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Trailer::None => panic!("trying to index into a trailer of size 0"),
            Trailer::EthernetFcs(fcs) => match index {
                0..=3 => &fcs[index],
                _ => {
                    panic!(
                        "index {} out of bounds for ethernet fcs trailer of size {} bytes",
                        index,
                        self.size()
                    );
                }
            },
        }
    }
}

impl IndexMut<usize> for Trailer {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self {
            Trailer::None => panic!("trying to index into a trailer of size 0"),
            Trailer::EthernetFcs(fcs) => match index {
                0..=3 => &mut fcs[index],
                _ => {
                    panic!(
                        "index {} out of bounds for ethernet fcs trailer of size 4 bytes",
                        index
                    );
                }
            },
        }
    }
}
