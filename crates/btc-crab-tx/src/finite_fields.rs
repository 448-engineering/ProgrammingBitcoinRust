use crate::{BtcError, BtcResult};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FieldElement {
    num: u32,
    prime: u32,
}

impl FieldElement {
    pub fn new(num: u32, prime: u32) -> BtcResult<Self> {
        // By using a Rust `u32` we always ensure the number cannot be less than 0
        // and we avoid checking `num >= prime || num < 0`
        if num >= prime {
            return Err(BtcError::NumMustBeLessThanPrimeOrder);
        } else {
            Ok(Self { num, prime })
        }
    }

    pub fn pow(&self, exponent: i32) -> BtcResult<Self> {
        // Force number out of becoming a negative
        let n = exponent.rem_euclid((self.prime - 1) as i32) as u32;

        if let Some(exp) = self.num.checked_pow(n) {
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

impl core::ops::Add for FieldElement {
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

#[cfg(test)]
mod sanity_checks {
    use crate::FieldElement;

    #[test]
    fn equality() {
        let a = FieldElement::new(7, 13);
        let b = FieldElement::new(6, 13);

        assert!(a.is_ok());
        assert!(b.is_ok());

        assert!(a == a);
        assert!(b == b);

        assert_eq!(false, (a.unwrap() == b.unwrap()));
    }

    #[test]
    fn addition() {
        let a = FieldElement::new(7, 13);
        let b = FieldElement::new(12, 13);
        let c = FieldElement::new(6, 13);

        assert!(a.is_ok());
        assert!(b.is_ok());
        assert!(c.is_ok());

        assert!((a.unwrap() + b.unwrap()) == c);
    }

    #[test]
    fn subtraction() {
        let a = FieldElement::new(7, 13);
        let b = FieldElement::new(6, 13);
        let c = FieldElement::new(1, 13);

        assert!(a.is_ok());
        assert!(b.is_ok());
        assert!(c.is_ok());

        assert!((a.unwrap() - b.unwrap()) == c);
    }

    #[test]
    fn multiplication() {
        let a = FieldElement::new(3, 13);
        let b = FieldElement::new(12, 13);
        let c = FieldElement::new(10, 13);

        assert!(a.is_ok());
        assert!(b.is_ok());
        assert!(c.is_ok());

        assert!((a.unwrap() * b.unwrap()) == c);
    }

    #[test]
    fn power() {
        let a = FieldElement::new(3, 13);
        let b = FieldElement::new(1, 13);

        assert!(a.is_ok());
        assert!(b.is_ok());

        assert!(a.unwrap().pow(3) == b);
    }

    #[test]
    fn negative_power() {
        let a = FieldElement::new(3, 13);
        let b = FieldElement::new(1, 13);

        assert!(a.is_ok());
        assert!(b.is_ok());

        assert!(a.unwrap().pow(-3) == b);
    }
}
