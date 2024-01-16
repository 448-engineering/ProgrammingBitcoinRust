#import "../template.typ": *

= Finite Fields

One of the most difficult things about learning how to program Bitcoin is knowing where to start. There are so many components that depend on each other that learning one thing may lead you to have to learn another, which in turn may lead you to need to learn something else before you can understand the original thing.

This chapter is going to get you off to a more manageable start. It may seem strange, but we’ll start with the basic math that you need to understand elliptic curve cryptography. Elliptic curve cryptography, in turn, gives us the signing and verification algorithms. These are at the heart of how transactions work, and transactions are the atomic unit of value transfer in Bitcoin. By learning about finite fields and elliptic curves first, you’ll get a firm grasp of concepts that you’ll need to progress logically.

Be aware that this chapter and the next two chapters may feel a bit like you’re eating vegetables, especially if you haven’t done formal math in a long time. I would encourage you to get through them, though, as the concepts and code presented here will be used throughout the book.

=== Learning Higher-Level Math

Learning about new mathematical structures can be a bit intimidating, and in this chapter, I hope to dispel the myth that high-level math is difficult. Finite fields, in particular, don’t require all that much more in terms of prior mathematical knowledge than, say, algebra.
Think of finite fields as something that you could have learned instead of trigonometry, except that the education system you’re a part of decided that trigonometry was more important for you to learn. This is my way of telling you that finite fields are not that hard to learn and require no more background than algebra.

=== Finite Field Definition

Mathematically, a finite field is defined as a finite set of numbers and two operations + (addition) and ⋅ (multiplication) that satisfy the following:
1.  If a and b are in the set, a + b and a ⋅ b are in the set. We call this property *closed*.
2.  0 exists and has the property a + 0 = a. We call this the *additive identity*.
3.  1 exists and has the property a ⋅ 1 = a. We call this the *multiplicative identity*.
4.  If a is in the set, –a is in the set, which is defined as the value that makes a + (–a) = 0. This is what we call the *additive inverse*.
5.  If a is in the set and is not 0, a–1 is in the set, which is defined as the value that makes a ⋅ a–1 = 1. This is what we call the *multiplicative inverse*.

Let’s unpack each of these criteria.

We have a set of numbers that’s finite. Because the set is finite, we can designate a
number p, which is how big the set is. This is what we call the order of the set.

`#1` says we are _closed under addition and multiplication_. This means that we have to define _addition and multiplication_ in a way that _ensures the results stay in the set_. For
example, a set containing `{0,1,2}` is _not closed under addition_, since
  $ 1 + 2 = 3 $ 
and _3 is not in the set_; neither is 
  $ 2 + 2 = 4 $ Of course we can define addition a little differently
to make this work, but using “normal” addition, this set is not closed. On the other hand, the set 
$ {1,0,1} $
_is closed under normal multiplication_. Any two numbers can be multiplied (there are nine such combinations), and the result is always in the set.

The other option we have in mathematics is to _define multiplication in a particular way to make these sets closed_. We’ll get to how exactly we define addition and multiplication later in this chapter, but the key concept here is that _we can define addition and subtraction differently than the addition and subtraction you are familiar with_.

`#2` and `#3` mean that we have the additive and multiplicative identities. That means _0 and 1_ are in the set.

`#4` means that we have the additive inverse. That is, _if a is in the set, –a is in the set_.
Using the additive inverse, we can define subtraction.

`#5` means that multiplication has the same property. If a is in the set, a–1 is in the set.
That is 
$ a ⋅ a–1 = 1 $
Using the multiplicative inverse, we can define division. This will be
the trickiest to define in a finite field.

== Defining Finite Sets

If the order (or size) of the set is _p_, we can call the elements of the set, $ 0, 1, 2, ... p – 1 $
These _numbers_ are what we call the _elements of the set_, not necessarily the traditional numbers 0, 1, 2, 3, etc. They behave in many ways like traditional numbers, but have some differences in how we add, subtract, multiply, and so forth.

In math notation the finite field set looks like this:

$ F#sub[p] = {0, 1, 2, ... p–1} $

What’s in the finite field set are the elements. F#sub[p] is a specific finite field called _“field of p"_ or _“field of 29”_ or whatever the size of it is (again, _the size is what mathematicians call order_). The numbers between the _{}_s represent what elements are in the field. We name the elements 0, 1, 2, etc. because these names are convenient for our purposes.

A finite field of order 11 looks like this:
$ F#sub[11] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10} $


A finite field of order 17 looks like this:
$ F#sub[17] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16} $

A finite field of order 983 looks like this:
$ F#sub[983] = {0, 1, 2,...982} $

Notice _the order of the field is always 1 more than the largest element_. You might have noticed that the _field has a prime order every time_. For a variety of reasons that will become clear later, it turns out that _fields must have an order that is a power of a prime_, and that the finite fields whose order is prime are the ones we’re interested in.

#pagebreak()

=== Constructing a Finite Field in Rust

#include "../errors.typ"


We want to represent each finite field element, so in Rust, we’ll be creating a struct that represents a single finite field element. Naturally, we’ll name the struct _FieldElement_.
The struct represents an element in a field *F#sub[prime]*. The bare bones of the struct look like this:
#show raw: set text(font: "Fira Code")
```rs
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
}

```
1. First we use `#[derive(Debug, PartialEq, Eq)]` to ensure 
- we can print the fields of the struct to the using `Debug` the
- we can compare two `FieldElement`s to see if they are equal using the `PartialEq, Eq`. In Rust deriving `PartialEq, Eq` on a struct automatically implements equality operations for fields of a struct if the fields are primitives or if their types already implement `PartialEq, Eq` as part of the standard library.

2. We then create the struct _FieldElement_ with the fields _num: u32, prime: u32_ 
3. We then initialize the struct by creating a method _new()_ withing the impl block _impl FieldElement_ where the _new()_ method takes two parameters _num: u32, prime: u32_ and returns _Self_ which is the struct _FieldElement_
4. Next we create a method _is_within_order()_ which is used to:
- check if the _num_ field is an element within the _order_ of length defined by the _prime_ field and return an error if the outcome is _true_. Note that since we are using Rust _u32_ we do not need to check if the `num`  field is less than _0_
      #temp_fira[if self.num >= self.prime]

We have also introduced _BtcError::NumMustBeLessThanPrimeOrder_ as the return type so we add that to our error enum
```rs
...

pub enum BtcError {
    /// The `num` field must be less than the `prime` field
    NumMustBeLessThanPrimeOrder, // <- Add this variant
    ...
}
```

and then we implement debug for that new element
```rs
impl fmt::Debug for BtcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error_as_str = match self {          
          ...
          Self::NumMustBeLessThanPrimeOrder => "The `self.num` field must be less than the `self.prime` field
    };

        write!(f, "{}", error_as_str)
    }
}
```

We can then use our _FieldElement_ to perform operations
```rs 
fn main() {
  // Define an element using `FieldElement::new()` method
    let field_element_a = FieldElement::new(7, 13);
    let field_element_b = FieldElement::new(6, 13);
    let field_element_c = FieldElement::new(13, 13);

    // Since we implemented equality comparison 
    // using `PartialEq` and `Eq` traits 
    // using #[derive(PartialEq, Eq)] we can check for
    // equality between two `FieldElement`s returning
    // true or false
    println!("{}", field_element_a == field_element_b);
    assert_ne!(field_element_a, field_element_b);
    println!("{}", field_element_a == field_element_a);
    assert_eq!(field_element_a, field_element_a);

    // We can check if the `FieldElement` field `num`
    // is within the `order` using the 
    // method `is_within_order()`
    println!("{:?}", field_element_a.is_within_order());
    println!("{:?}", field_element_b.is_within_order());
    println!("{:?}", field_element_c.is_within_order());
}
```

#pagebreak()
