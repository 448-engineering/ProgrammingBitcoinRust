use std::io::ErrorKind;

// Create a `Result` type that can be reused
// rather than having to type `-> Result<T, BtcError>`
pub type BtcResult<T> = Result<T, BtcError>;
/// Our Error type to handle returning errors
/// as the same type when we use `?`
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum BtcError {
    /// Error caused by integer overflow
    #[error("An integer overflow ocurred")]
    IntegerOverflow,
    /// The `num` field must be less than the `prime` field
    #[error("The `self.num` field must be less than the `self.prime` field")]
    NumMustBeLessThanPrimeOrder,
    /// Return this error if we are trying to add
    /// two `FieldElement`s that are not part of
    /// the same order
    #[error(
        "Cannot add two numbers in different Fields. Prime numbers must be equal in field `self.prime`"
    )]
    PrimeOrderMustBeEqual,
    /// The values provided are not on the Bitcoin secp256k1 elliptic curve `y2 = x3 + ax + b`
    #[error("The values x = {x} and y = {y} provided are not on the Bitcoin secp256k1 elliptic curve `y2 = x3 + ax + b`")]
    NotOnEllipticCurve { x: i64, y: i64 },
    /// Error while parsing the transaction version from bytes
    #[error("Error while parsing the transaction version from bytes.")]
    InvalidTransactionVersion,
    /// At least 4 bytes are required to parse a transaction version
    #[error("At least 4 bytes are required to parse a transaction version.")]
    InvalidByteLengthToParseTransactionVersion,
    /// The number of bytes required to convert to an unsigned is invalid
    #[error("The number of bytes required to convert to an unsigned is invalid")]
    InvalidByteLengthToParseToInteger,
    /// A slice of at least 32 bytes was expected by the operation
    #[error("A slice of at least 32 bytes was expected by the operation")]
    ExpectedLengthAtLeast32Bytes,
    /// The length of the slice passed was less than the expected length
    #[error("Invalid length of the byte slice. Expected length was {expected} but found length was {found}")]
    InvalidByteLength { expected: usize, found: usize },
    /// Encountered [std::io::Error] which yielded [std::io::ErrorKind]
    #[error("Encountered [std::io::Error] which yielded [std::io::ErrorKind] type {0}")]
    Io(ErrorKind),
}

impl From<std::io::Error> for BtcError {
    fn from(value: std::io::Error) -> Self {
        BtcError::Io(value.kind())
    }
}
