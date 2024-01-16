mod errors;
pub use errors::*;

fn main() {
    println!("{}", (-27i64).rem_euclid(13));
    println!("{}", 7u32.rem_euclid(3));
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct FieldElement {
    num: u32,
    prime: u32,
}

impl FieldElement {
    pub fn new(num: u32, prime: u32) -> Self {
        Self { num, prime }
    }

    pub fn is_within_order(&self) -> BtcResult<&Self> {
        // By using a Rust `u32` we always ensure the number cannot be less than 0
        // and we avoid checking `num >= prime || num < 0`
        if self.num >= self.prime {
            return Err(BtcError::NumMustBeLessThanPrimeOrder);
        } else {
            Ok(self)
        }
    }

    pub fn pow(&self, exponent: u32) -> BtcResult<Self> {
        if let Some(exp) = self.num.checked_pow(exponent) {
            let modulo = exp.rem_euclid(self.prime);
            Ok(Self {
                num: modulo,
                prime: self.prime,
            })
        } else {
            Err(BtcError::IntegerOverflow)
        }
    }
}

impl std::ops::Add for FieldElement {
    /// We want to return an error if the `order` of both sets is not equal
    type Output = BtcResult<Self>;

    fn add(self, other: Self) -> BtcResult<Self> {
        // We have to ensure that the elements are from the same finite field,
        //otherwise this calculation doesn’t have any meaning
        if self.prime != other.prime {
            return Err(BtcError::PrimeOrderMustBeEqual);
        }

        // Addition in a finite field is defined with the modulo operator, as explained earlier.
        // We return an instance of [Self], which we can conveniently access.
        // However, in Rust we enforce the return type by wrapping our [Self] as part of the Result using `Ok()`
        Ok(Self {
            num: (self.num + other.num).rem_euclid(self.prime),
            prime: self.prime,
        })
    }
}

impl std::ops::Sub for FieldElement {
    /// We want to return an error if the `order` of both sets is not equal
    type Output = BtcResult<Self>;

    fn sub(self, other: Self) -> BtcResult<Self> {
        // We have to ensure that the elements are from the same finite field,
        //otherwise this calculation doesn’t have any meaning
        if self.prime != other.prime {
            return Err(BtcError::PrimeOrderMustBeEqual);
        }

        Ok(Self {
            num: (self.num - other.num).rem_euclid(self.prime),
            prime: self.prime,
        })
    }
}

impl std::ops::Mul for FieldElement {
    /// We want to return an error if the `order` of both sets is not equal
    type Output = BtcResult<Self>;

    fn mul(self, other: Self) -> BtcResult<Self> {
        // We have to ensure that the elements are from the same finite field,
        //otherwise this calculation doesn’t have any meaning
        if self.prime != other.prime {
            return Err(BtcError::PrimeOrderMustBeEqual);
        }

        Ok(Self {
            num: (self.num * other.num).rem_euclid(self.prime),
            prime: self.prime,
        })
    }
}
