#import "../template.typ": *
#pagebreak()
== Finite Field Addition and Subtraction
Remember that we need to define finite field addition such that we ensure the result is still in the set. That is, we want to make sure that addition in a finite field is closed.

We can use what we just learned, modulo arithmetic, to make addition closed. Let’s say we have a _finite field of 19_:
#temp_fira[$  F#sub[19] = {0, 1, 2, ... 18} $]
where _a, b #sym.epsilon.alt F#sub[19]_. Note that the symbol _ #sym.epsilon.alt _ means _“is an element of.”_ In our case, _a and b
are elements of F#sub[19]_.

Addition being closed means:
#temp_fira[$ a+#sub[f]b ∈ F#sub[19] $]
We denote finite field addition with _+#sub[f]_ to avoid confusion with normal integer addition, _+_.
If we utilize modulo arithmetic, we can guarantee this to be the case. We can define _a+#sub[f]b_ this way:
#temp_fira[$ a+#sub[f]b = (a + b)%19 $]
For example:
#temp_fira[$
7+#sub[f]8 = (7+8)%19 = 15 \
11+#sub[f]17 = (11+17)%19 = 9
$]
and so on.

We take any _two numbers in the set, add, and “wrap around” the end to get the sum_.
We are creating our own addition operator here and the result is a bit unintuitive.
After all, _11+#sub[f]17 = 9_ just doesn’t look right because we’re not used to finite field addition.

More generally, we define field addition this way:
#temp_fira[$ a+#sub[f]b = (a + b)%p $]
where _a, b #sym.epsilon.alt F#sub[p]_.

We also define the additive inverse this way. _a #sym.epsilon.alt F#sub[p]_ implies that _–#sub[f]a #sym.epsilon.alt F#sub[p]_:
#temp_fira[$ –#sub[f]a = (–a) % p $]

Again, for clarity, we use –#sub[f] to distinguish field subtraction and negation from integer subtraction and negation.
In F#sub[19]:
#temp_fira[$ –#sub[f]9 = (–9) % 19 = 10 $]
which means that:
#temp_fira[$ 9+#sub[f]10 = 0 $]
And that turns out to be true.
#pagebreak()
Similarly, we can do field subtraction:
#temp_fira[$ a–#sub[f]b = (a – b)%p $]
where _a, b #sym.epsilon.alt F#sub[p]_.
In F#sub[19]:
#temp_fira[$ 
11– #sub[f]9=(11-9)%19 = 2 \
6–#sub[f]13=(6-13)%19 = 12
$]
and so on.

=== Exercise

Solve these problems in _F#sub[57]_ (assume all _+_’s here are _+#sub[f]_ and _–_’s here are _–#sub[f]_):
1. 44 + 33
2. 9 – 29
3. 17 + 42 + 49
4. 52 – 30 – 38



=== Coding Addition and Subtraction in Rust

Rust allows us to override the trait `std::ops::Add` with our own custom implementation that would allow as to add `FieldElement + FieldElement` together. If we try to add two `FieldElement`s together without implementing this we get the error:
#temp_fira_error[
  ```sh
cannot add `FieldElement` to `FieldElement` ... an implementation of `Add` might be missing for `FieldElement`
```
]

Let's implement this trait
```rs 
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
            prime: self.prime + other.prime,
        })
    }
}
```

We are now returning _return Err(BtcError::PrimeOrderMustBeEqual);_ so we add it  to our error type by extending the current `BtcError` enum with variant _PrimeOrderMustBeEqual_
```rs 
...
pub enum BtcError {
    ...
  
    /// Return this error if we are trying to add
    /// two `FieldElement`s that are not part of
    /// the same order
    PrimeOrderMustBeEqual,
}
```

We also want to implement `fmt::Debug` for the new `PrimeOrderMustBeEqual` enum variant so we extend the `impl fmt::Debug for BtcError` trait
```rs 
impl fmt::Debug for BtcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error_as_str = match self {
         ...
          Self::PrimeOrderMustBeEqual => "Cannot add two numbers in different Fields. Prime numbers must be equal in field `self.prime`"
          };
        ...
}
```

=== Exercise 3

Write the corresponding std::ops::Sub method that defines the subtraction of two `FieldElement` objects.
```rs 
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
            prime: self.prime - other.prime,
        })
    }
}
```

#pagebreak()