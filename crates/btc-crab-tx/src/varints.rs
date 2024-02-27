use std::io::{Cursor, Read};

use crate::{BtcError, BtcResult};

//  Varint is shorthand for variable integer, which is a way
// to encode an integer into bytes
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum VarInt {
    U8,
    U16,
    U32,
    U64,
}

impl VarInt {
    pub const fn parse(byte: u8) -> Self {
        match byte {
            0..=252 => Self::U8,
            253 => Self::U16,
            254 => Self::U32,
            255 => Self::U64,
        }
    }

    pub const fn byte_len(&self) -> usize {
        match self {
            Self::U8 => 1,
            Self::U16 => 2,
            Self::U32 => 4,
            Self::U64 => 8,
        }
    }

    pub fn integer(&self, bytes: &mut Cursor<&[u8]>) -> BtcResult<usize> {
        let outcome = match self {
            Self::U8 => {
                if bytes.get_ref().len() < 1 {
                    return Err(BtcError::InvalidByteLengthToParseToInteger);
                }

                bytes.set_position(bytes.position() - 1);

                let mut buffer = [0u8; 1];
                bytes.read_exact(&mut buffer)?;

                buffer[0] as usize
            }
            Self::U16 => {
                if bytes.get_ref().len() < 2 {
                    return Err(BtcError::InvalidByteLengthToParseToInteger);
                }

                let mut buffer = [0u8; 2];
                bytes.read_exact(&mut buffer)?;

                u16::from_le_bytes(buffer) as usize
            }
            Self::U32 => {
                if bytes.get_ref().len() < 4 {
                    return Err(BtcError::InvalidByteLengthToParseToInteger);
                }

                let mut buffer = [0u8; 4];
                bytes.read_exact(&mut buffer)?;

                u32::from_le_bytes(buffer) as usize
            }
            Self::U64 => {
                if bytes.get_ref().len() < 8 {
                    return Err(BtcError::InvalidByteLengthToParseToInteger);
                }

                let mut buffer = [0u8; 8];
                bytes.read_exact(&mut buffer)?;

                u64::from_le_bytes(buffer) as usize
            }
        };

        Ok(outcome)
    }
}
