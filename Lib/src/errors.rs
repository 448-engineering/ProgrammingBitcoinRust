// Import `fmt` method that allows us to format Rust code.
// Using `core` instead of `std` allows us to use this even
// in `no_std` context.
use core::fmt;

// Create a `Result` type that can be reused
// rather than having to type `-> Result<T, BtcError>`
pub type BtcResult<T> = Result<T, BtcError>;

/// Our Error type to handle returning errors
/// as the same type when we use `?`
#[derive(PartialEq, Eq)]
pub enum BtcError {
    /// Error caused by integer overflow
    IntegerOverflow,
    /// The `num` field must be less than the `prime` field
    NumMustBeLessThanPrimeOrder,
    /// Return this error if we are trying to add
    /// two `FieldElement`s that are not part of
    /// the same order
    PrimeOrderMustBeEqual,
}

impl fmt::Debug for BtcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error_as_str = match self {
          Self::IntegerOverflow => "An integer overflow ocurred",
          Self::NumMustBeLessThanPrimeOrder => "The `self.num` field must be less than the `self.prime` field",
          Self::PrimeOrderMustBeEqual => "Cannot add two numbers in different Fields. Prime numbers must be equal in field `self.prime`"
    };

        write!(f, "{}", error_as_str)
    }
}
