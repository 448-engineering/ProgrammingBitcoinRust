#import "../template.typ": *

== Finite Field Division

The intuition that helps us with addition, subtraction, multiplication, and perhaps even exponentiation unfortunately doesn’t help us quite as much with division.

Because division is the hardest operation to make sense of, we’ll start with something that should make sense.

In normal math, division is the inverse of multiplication:
- $7 #sym.dot 8 = 56$ implies that $56 div 8 = 7$
- $12 #sym.dot 2 = 24$ implies that $24 div 12 = 2$

And so on. We can use this as the definition of division to help us. Note that like in normal math, _you cannot divide by 0_.

In _F#sub[19]_, we know that:
#temp_math[
3#dot_f[7] = 21%19 = 2 implies that 2/#sub[f]7 = 3 \
9#dot_f[5] = 45%19 = 7 implies that 7/#sub[f]5 = 9
]

This is very unintuitive, as we generally think of 2/#sub[f]7 or 7/#sub[f]5 as fractions, not nice finite field elements. Yet that is one of the remarkable things about finite fields: finite fields are closed under division. That is, dividing any two numbers where the denominator is not 0 will result in another finite field element.

The question you might be asking yourself is, how do I calculate 2/f7 if I don’t know beforehand that _3#dot_f[7] = 2_? This is indeed a very good question; to answer it, we’ll have to use the result from previous Exercise.

In case you didn’t get it, the answer is that _$n^((p–1))$_ is always _1 for every p that is prime and every n > 0_. This is a beautiful result from number theory called Fermat’s little theorem. Essentially, the theorem says:
#temp_math[$ n^((p–1))%p = 1 $]
#temp_fira[where p is prime.]

Since we are operating in prime fields, this will always be true.
#block_grey_bg[
  *Fermat’s Little Theorem*
  
  There are many proofs of this theorem, but perhaps the simplest is using what we saw  that these sets are equal:
  #temp_math[$ {1, 2, 3, ... p–2, p–1} = {n%p, 2n%p, 3n%p (p–2)n%p, (p–1)n%p} $]
  The resulting numbers might not be in the right order, but the same numbers are in both sets. We can then multiply every element in both sets to get this equality:
#temp_math[$ 1 ⋅ 2 ⋅ 3 ⋅ ... ⋅ (p–2) ⋅ (p–1) % p = n ⋅ 2n ⋅ 3n ⋅ ... ⋅ (p–2)n ⋅ (p–1)n % p $]


The left side is the same as _(p–1)! % p where ! is the factorial_ e.g., #temp_math[$5! = 5 ⋅ 4 ⋅ 3 ⋅ 2 ⋅ 1 $]
On the right side, we can gather up all the _$n`$_’s and get:
#temp_math[
  $ (p–1)! #sym.dot n^((p–1)) % p $
]

The _(p–1)!_ on both sides cancel, giving us:
#temp_math[$ 1 = n^((p–1)) % p $]
This proves Fermat’s little theorem.
]

Because division is the inverse of multiplication, we know:
#temp_fira[$ a/b = a#sym.dot#sub[f] (1/b) = a#sym.dot#sub[f]b^(–1) $]

We can reduce the division problem to a multiplication problem as long as we can figure out what _b–1_ is. This is where Fermat’s little theorem comes into play. We know:
#temp_math[$ b^((p–1)) = 1 $]
because _p is prime_. Thus:
#temp_math[$ b^(–1) = b^(–1) #dot_f_none 1 = b^(–1)#dot_f_none b^((p–1)) = b^((p–2)) $]
or:
#temp_math[$ b^(–1) = b^((p–2)) $]
In _F#sub[19]_, this means practically that _$b^18$ = 1_ , which means that _$b^(–1) = b^17$_ for all _b > 0_.

So in other words, we can calculate the inverse using the exponentiation operator. In _F#sub[19]_:
#temp_math[$
2/7 = 2#sym.dot 7^((19 – 2)) = 2#sym.dot 7^17=465261027974414%19 = 3 \
7/5 = 7#sym.dot 5^((19 – 2)) = 7#sym.dot 5^17=5340576171875%19 = 9
$]

This is a relatively expensive calculation as exponentiating grows very fast. Division is the most expensive operation for that reason. To lessen the expense, we can use the _checked_pow()_ method on an integer in Rust, which does exponentiation and returns `Option::None` in case of integer overflow. In Rust, _7u32.checked_pow(17)_ is a good example. We then modulo the result.

=== Exercise

Solve the following equations in _F#sub[31]_:
- 3 / 24
- $17^(–3)$
- $4^(–4) #sym.dot 11$


=== Exercise

Write the corresponding _truediv()_ method on _FieldElement_ struct that defines the division of two field elements.

