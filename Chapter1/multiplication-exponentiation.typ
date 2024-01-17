#import "../template.typ": *

== Finite Field Multiplication and Exponentiation

Just as we defined a new addition _(+#sub[f])_ for finite fields that was closed, we can also define a new multiplication for finite fields that’s closed. By multiplying the same number many times, we can also define exponentiation. In this section, we’ll go through exactly how to define this using modulo arithmetic.

Multiplication is adding multiple times:

#temp_fira[
5 #sym.dot 3 = 5 + 5 + 5 = 15 \
8 #sym.dot 17 = 8 + 8 + 8 + ... (17 total 8’s) ... + 8 = 136 
]

We can define multiplication on a finite field the same way. Operating in _F#sub[19]_ once again:

#temp_fira[
  5#dot_f[3] = 5 + #sub[f]5 + #sub[f]5 \
8#dot_f[17] = 8 + #sub[f]8 +#sub[f]8 +#sub[f] ... (17 total 8’s) ... +#sub[f]8
]

We already know how to do the right side, and that yields a number within the _F#sub[19]_ set:
#temp_fira[
  5#dot_f[3] = 5 +#sub[f]5 +#sub[f]5 = 15 % 19 = 15 \
8#dot_f[17] = 8 +#sub[f]8 +#sub[f]8 + #sub[f] ... (17 total 8’s) ... +#sub[f]8 = (8 #sym.dot 17) % 19 = 136 % 19 = 3
]

Note that the second result is pretty unintuitive. We don’t normally think of #temp_fira[$ 8#dot_f[17] = 3 $] but that’s part of what’s necessary in order to define multiplication to be closed.
That is, the result of field multiplication is always in the set _{0, 1, ... p–1}_.

Exponentiation is simply multiplying a number many times:
#temp_fira[$ 7^3=7#dot_f[7]#dot_f[7] = 343 $]

In a finite field, we can do exponentiation using modulo arithmetic.
In _F#sub[19]_:
#temp_math[
 $ 7^3 = 343 % 19=1 \
9^12 = 7
$
]

Exponentiation again gives us counterintuitive results. We don’t normally think _$7^3 = 1$_ or _$9^12 = 7$_. Again, finite fields have to be defined so that the operations always result in a number within the field.

=== Exercise

Solve the following equations in _F#sub[97]_ (again, assume _ #sym.dot _ and exponentiation are field versions):

- 95 #sym.dot 45 #sym.dot 31
- 17 #sym.dot 13 #sym.dot 19 #sym.dot 44
- 127 #sym.dot 7749

=== Exercise

For _$k = 1, 3, 7, 13, 18$_, what is this set in _F#sub[19]_?
#temp_math[$ {k ⋅ 0, k ⋅ 1, k ⋅ 2, k ⋅ 3, ... k ⋅ 18} $]
Do you notice anything about these sets?

#block_grey_bg[
*Why Fields Are Prime*

The answer to the last Exercise is why fields have to have a prime power number of elements. No matter what _k_ you choose, as long as it’s greater than _0_, multiplying the entire set by _k_ will result in the same set as you started with.
  
Intuitively, the fact that we have a _prime order_ results in every _element of a finite field being equivalent_. If the order of the set was a composite number, multiplying the set by one of the divisors would result in a smaller set.
]

=== Coding Multiplication in Rust

Now that we understand what multiplication should be in _FieldElement_, we want to define the _mul()_ method on struct `FieldElement` that overrides the `std::ops::Mul` trait. We want this to work:
```rs 
let a = FieldElement::new(3, 13);
let b = FieldElement::new(12, 13);
let c = FieldElement::new(10, 13);

assert_eq!((a * b), c);
```

As with addition and subtraction, let's make multiplication work for our class by defining the _mul()_ method on `FieldElement` struct.

```rs 
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
```

=== Coding Exponentiation in Rust

We need to define the exponentiation for _FieldElement_ struct, which in Rust can be implemented by creating a method _pow()_ on the struct. The difference here is that the exponent is not a FieldElement, so it has to be treated a bit differently. We want something like this to work:

```rs 

    let a = FieldElement::new(3, 13);
    let b = FieldElement::new(1, 13);

    let pow_a = a.pow(3).unwrap();
    assert_eq!(pow_a, b);
```
#pagebreak()

Note that because the exponent is an integer, instead of another instance of `FieldElement`, the method receives the variable exponent as an integer. We can code it this way:

```rs 
impl FieldElement {
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
```

Why don’t we force the exponent to be a FieldElement object? It turns out that the exponent doesn’t have to be a member of the finite field for the math to work. In fact, if it were, the exponents wouldn’t display the intuitive behavior we expect, like being able to add the exponents when we multiply with the same base.

Some of what we’re doing now may seem slow for large numbers, but we’ll use some clever tricks to improve the performance of these algorithms.

=== Exercise

For _p = 7, 11, 17, 31_, what is this set in _F#sub[p]_?
#temp_math[$ {1^((p – 1)), 2^((p – 1)), 3^((p – 1)), 4^((p – 1)), ... (p – 1)^((p – 1))} $]
