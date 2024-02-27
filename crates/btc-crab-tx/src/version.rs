use crate::{BtcError, BtcResult};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum TxVersion {
    #[default]
    One,
    Two,
}

impl TxVersion {
    pub fn to_decimal(&self) -> u32 {
        match self {
            Self::One => 1u32,
            Self::Two => 2u32,
        }
    }

    pub fn from_bytes(bytes: [u8; 4]) -> BtcResult<Self> {
        let outcome = match u32::from_le_bytes(bytes) {
            1u32 => Self::One,
            2u32 => Self::Two,
            _ => return Err(BtcError::InvalidTransactionVersion),
        };

        Ok(outcome)
    }
}

#[cfg(test)]
mod tx_sanity_checks {
    use crate::{BtcError, TxVersion};

    #[test]
    fn tx_version() {
        assert_eq!([1u8, 0, 0, 0], TxVersion::One.to_decimal().to_le_bytes());
        assert_eq!([2u8, 0, 0, 0], TxVersion::Two.to_decimal().to_le_bytes());

        assert_eq!(
            TxVersion::One,
            TxVersion::from_bytes([1u8, 0, 0, 0]).unwrap()
        );
        assert_eq!(
            TxVersion::Two,
            TxVersion::from_bytes([2u8, 0, 0, 0]).unwrap()
        );

        assert!(TxVersion::from_bytes([1u8, 0, 0, 0]).is_ok());
        assert!(TxVersion::from_bytes([2u8, 0, 0, 0]).is_ok());

        assert_eq!(
            Err(BtcError::InvalidTransactionVersion),
            TxVersion::from_bytes([0u8, 0, 0, 0])
        );
        assert!(TxVersion::from_bytes([0u8, 0, 0, 0]).is_err());
    }
}
