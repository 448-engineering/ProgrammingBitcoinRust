use crate::{BtcError, BtcResult, VarInt, TX_ID_LEN};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct TxInput {
    // Little endian bytes of SHA256 hash
    previous_tx_id: [u8; TX_ID_LEN],
    //  each transaction has to have at least one output, but may have many.
    // Thus, we need to define exactly which output within a transaction we’re spending,
    // which is captured in the previous transaction index.
    // This is 4 bytes little endian which translates to u32
    previous_tx_index: u32,
    // preceded by a VarInt since it's a variable length field
    script_sig: Vec<u8>,
    // 4 Bytes little endian which translates to a u32 in Rust
    sequence: u32,
}
