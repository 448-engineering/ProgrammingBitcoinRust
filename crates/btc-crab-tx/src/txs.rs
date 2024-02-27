use sha2::{digest::KeyInit, Sha256};

use crate::{BtcError, BtcResult, TxVersion, VarInt, TX_ID_LEN, TX_VERSION_BYTE_LEN};
use std::io::{BufRead, Cursor, Read};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct BtcTx {
    version: TxVersion,
    inputs: Vec<TxInput>,
    outputs: Vec<TxOutput>,
    locktime: u32,
}

impl BtcTx {
    pub fn from_hex_bytes(bytes: impl AsRef<[u8]>) -> BtcResult<Self> {
        let mut bytes = Cursor::new(bytes.as_ref());

        // Check if there are enough bytes to get te transaction version from
        if bytes.get_ref().len() < 4 {
            return Err(BtcError::InvalidByteLengthToParseTransactionVersion);
        }

        let mut version_bytes = [0u8; TX_VERSION_BYTE_LEN];
        bytes.read_exact(&mut version_bytes)?;
        let version = TxVersion::from_bytes(version_bytes)?;

        let inputs = BtcTx::get_inputs(&mut bytes)?;
        let outputs = BtcTx::outputs_decoder(&mut bytes)?;
        let locktime = BtcTx::locktime(&mut bytes)?;

        Ok(BtcTx {
            version,
            inputs,
            outputs,
            locktime,
        })
    }

    fn get_inputs(bytes: &mut Cursor<&[u8]>) -> BtcResult<Vec<TxInput>> {
        let mut varint_len = [0u8];
        bytes.read_exact(&mut varint_len)?;

        let no_of_inputs = VarInt::parse(varint_len[0]).byte_len();

        dbg!(bytes.position());
        dbg!(no_of_inputs);

        let mut inputs = Vec::<TxInput>::new();

        (0..no_of_inputs).into_iter().for_each(|_| {
            inputs.push(BtcTx::input_decoder(bytes).unwrap());
        });

        Ok(inputs)
    }

    fn input_decoder(bytes: &mut Cursor<&[u8]>) -> BtcResult<TxInput> {
        let mut previous_tx_id = [0u8; 32];
        bytes.read_exact(&mut previous_tx_id)?;
        previous_tx_id.reverse();
        dbg!(hex::encode(&previous_tx_id));

        let mut previous_tx_index_bytes = [0u8; 4];

        bytes.read_exact(&mut previous_tx_index_bytes)?;
        let previous_output_index = u32::from_le_bytes(previous_tx_index_bytes);

        let mut signature_script_size = [0u8];
        bytes.read_exact(&mut signature_script_size)?;
        dbg!(&signature_script_size);

        let varint = VarInt::parse(signature_script_size[0]);
        let integer_from_varint = varint.integer(bytes)?;
        dbg!(&integer_from_varint);

        let mut signature_script = Vec::<u8>::new();
        let mut sig_buf = [0u8; 1];
        (0..integer_from_varint).for_each(|_| {
            bytes.read_exact(&mut sig_buf).unwrap();

            signature_script.extend_from_slice(&sig_buf);
        });
        dbg!(&signature_script.len());
        dbg!(&signature_script.capacity());

        let mut sequence_num_bytes = [0u8; 4];
        bytes.read_exact(&mut sequence_num_bytes)?;
        dbg!(sequence_num_bytes);
        let sequence_number = u32::from_le_bytes(sequence_num_bytes);

        let tx_input = TxInput {
            previous_tx_id,
            previous_output_index,
            signature_script,
            sequence_number,
        };

        Ok(tx_input)
    }

    fn outputs_decoder(bytes: &mut Cursor<&[u8]>) -> BtcResult<Vec<TxOutput>> {
        let mut num_of_input_bytes = [0u8; 1];
        bytes.read_exact(&mut num_of_input_bytes)?;
        let num_of_inputs = VarInt::parse(num_of_input_bytes[0]).integer(bytes)?;

        let mut outputs = Vec::<TxOutput>::new();

        (0..num_of_inputs).into_iter().for_each(|_| {
            let mut satoshis_as_bytes = [0u8; 8];
            bytes.read_exact(&mut satoshis_as_bytes).unwrap();
            let satoshis = u64::from_le_bytes(satoshis_as_bytes);

            let mut locking_script_len = [0u8; 1];
            bytes.read_exact(&mut locking_script_len).unwrap();

            let script_len = VarInt::parse(locking_script_len[0]).integer(bytes).unwrap();
            let mut script = Vec::<u8>::new();

            dbg!(&script_len);

            dbg!(bytes.get_ref().len());
            dbg!(bytes.position());
            dbg!(0..script_len);

            (0..script_len).enumerate().for_each(|(index, _)| {
                //dbg!(index);
                let mut current_byte = [0u8; 1];

                bytes.read_exact(&mut current_byte).unwrap();
                script.extend_from_slice(&current_byte);
            });

            outputs.push(TxOutput {
                amount: satoshis,
                locking_script: script,
            });
        });

        Ok(outputs)
    }

    fn locktime(bytes: &mut Cursor<&[u8]>) -> BtcResult<u32> {
        let mut locktime_bytes = [0u8; 4];
        bytes.read_exact(&mut locktime_bytes)?;

        Ok(u32::from_le_bytes(locktime_bytes))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct TxInput {
    previous_tx_id: [u8; TX_ID_LEN],
    previous_output_index: u32,
    signature_script: Vec<u8>,
    sequence_number: u32,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct TxOutput {
    amount: u64,
    locking_script: Vec<u8>,
}
