#import "../template.typ": *

== Redefining Exponentiation

One last thing that we need to take care of before we leave this chapter is the _pow()_ method, which needs to handle negative exponents. For example, a–3 needs to be a finite field element, but the current code does not take care of this case. We want, for example, something like this to work:
```rs 
let a = FieldElement::new(7, 13);
let b = FieldElement::new(8, 13);
assert!(a.pow(-3).unwrap() == b);
```
Unfortunately, the way we’ve defined _pow()_ method on _FieldElement_ struct  simply doesn’t handle negative exponents, because the second parameter of the built-in Rust method _checked_pow() on an integer is required to be positive_.

Thankfully, we can use some math we already know to solve this. We know from _Fermat’s little theorem_ that:
#temp_math[$ a^(p–1) = 1 $]
This fact means that we can multiply by _$a^(p–1)$_ as many times as we want. So, for _$a^(–3)$_, we can do:
#temp_math[$ a^(–3) = a^(–3) #sym.dot a^(p–1) = a^(p–4) $]

To force a number out of being negative, we use _rem_euclid()_ integer method where we subtract _1_ from the prime and use it as the parameter for method arguments of the _pow()_ method 
#temp_fira[ 
exponent.rem_euclid((self.prime - 1) as i32) as u32;
]
#pagebreak()

As a bonus, we can also reduce very large exponents at the same time given that _$a^(p–1) = 1$_. This will make the pow function not work as hard. Let's change our _pow()_ method on _FieldElement_ to
```rs 
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
```

== Conclusion
In this chapter we learned about finite fields and how to implement them in Rust. We’ll be using finite fields in _Chapter 3_ for elliptic curve cryptography. We turn next to the other mathematical component that we need for elliptic curve cryptography: _elliptic curves_