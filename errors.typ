#import "./template.typ": *

=== Handling Rust Errors

Rust language allows us to enforce error handling by returning the `Result` type. Let's create this _Result_ type in Rust types.

#temp_fira[```rs
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
    IntegerOverflow
}

impl fmt::Debug for BtcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error_as_str = match self {          
          Self::IntegerOverflow => "An integer overflow ocurred",
    };

        write!(f, "{}", error_as_str)
    }
}

```]