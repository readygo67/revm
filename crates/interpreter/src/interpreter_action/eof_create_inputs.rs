use std::sync::Arc;

use bytecode::Eof;
use primitives::{Address, Bytes, U256};

use super::CallInput;

/// EOF create can be called from two places:
/// * EOFCREATE opcode
/// * Creation transaction.
///
/// Creation transaction uses initdata and packs EOF and initdata inside it,
/// and this eof bytecode needs to be validated.
///
/// Opcode creation uses already validated EOF bytecode, and input from Interpreter memory.
///
/// Address is already known and is passed as an argument.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EOFCreateKind {
    Tx {
        initdata: Bytes,
    },
    Opcode {
        initcode: Arc<Eof>,
        input: CallInput,
        created_address: Address,
    },
}

impl EOFCreateKind {
    /// Returns created address
    pub fn created_address(&self) -> Option<&Address> {
        match self {
            EOFCreateKind::Opcode {
                created_address, ..
            } => Some(created_address),
            EOFCreateKind::Tx { .. } => None,
        }
    }
}

impl Default for EOFCreateKind {
    fn default() -> Self {
        EOFCreateKind::Opcode {
            initcode: Arc::new(Eof::default()),
            input: CallInput::Bytes(Bytes::default()),
            created_address: Address::default(),
        }
    }
}

/// Inputs for EOF Create call
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EOFCreateInputs {
    /// Caller of EOF Create
    pub caller: Address,
    /// Values of ether transferred
    pub value: U256,
    /// Gas limit for the create call
    pub gas_limit: u64,
    /// EOF Create kind
    pub kind: EOFCreateKind,
}

impl EOFCreateInputs {
    /// Creates new EOF Create input from transaction that has concatenated eof init code and calldata.
    ///
    /// Legacy transaction still have optional nonce so we need to obtain it.
    pub fn new(caller: Address, value: U256, gas_limit: u64, kind: EOFCreateKind) -> Self {
        //let (eof_init_code, input) = Eof::decode_dangling(tx.data.clone())?;
        EOFCreateInputs {
            caller,
            value,
            gas_limit,
            kind,
        }
    }

    /// Returns a new instance of EOFCreateInput.
    pub fn new_opcode(
        caller: Address,
        created_address: Address,
        value: U256,
        eof_init_code: Eof,
        gas_limit: u64,
        input: CallInput,
    ) -> EOFCreateInputs {
        EOFCreateInputs::new(
            caller,
            value,
            gas_limit,
            EOFCreateKind::Opcode {
                initcode: Arc::new(eof_init_code),
                input,
                created_address,
            },
        )
    }
}
