#import "../template.typ": *

== Modulo Arithmetic

One of the tools we can use to make a finite field closed under addition, subtraction, multiplication, and division is something called _modulo arithmetic_.
We can define addition on the finite set using modulo arithmetic, which is something you probably learned when you first learned division.
#temp_fira[$ 7 div 3 = 2 R 1 $]

Whenever the division wasn’t even, there was something called the _“remainder,”_ which is the amount left over from the actual division. We define modulo in the same way. We use the operator _%_ for modulo:
#temp_fira[7 % 3 = 1]

Another example:
#temp_fira[$ 27 div 7 = 3 R 6 $]

Formally speaking, the modulo operation is the remainder after division of one number by another. Let’s look at another example with larger numbers:
#temp_fira[1747 % 241 = 60]
If it helps, you can think of modulo arithmetic as _“wraparound”_ or _“clock” math_. Imagine a problem like this:
It is currently 3 o’clock. 
#oneline_par[What hour will it be 47 hours from now?]
#block([#align(left, [#text("The answer is 2 o’clock because:")])])
#temp_fira[(3 + 47) % 12 = 2]

#figure(
  image("../images/modulo-clock.png", width: 80%),
  caption: [Clock going forward 47 hours.],
)

We can also see this as “wrapping around” in the sense that we go past 0 every time we move ahead 12 hours.

We can perform modulo on negative numbers. For example, you can ask:
#oneline_par[It is currently 3 o’clock. What hour was it 16 hours ago?]

The answer is 11 o’clock:
#temp_fira[(3 – 16) % 12 = 11]

The minute hand is also a modulo operation. For example, you can ask:
#oneline_par[It is currently 12 minutes past the hour. What minute will it be 843 minutes from now?]

It will be 15 minutes past the hour:
#temp_fira[(12 + 843) % 60 = 15]

Likewise, we can ask:
#oneline_par[It is currently 23 minutes past the hour. What minute will it be 97 minutes from now?]
In this case, the answer is 0:
#temp_fira[(23 + 97) % 60 = 0]
_0_ is another way of saying _there is no remainder_.

The result of the _modulo (%)_ operation for minutes is always between _0 and 59_, inclusive. This happens to be a very useful property as even very large numbers can
be brought down to a relatively small range with modulo:
#temp_fira[14738495684013 % 60 = 33]
We’ll be using modulo as we define field arithmetic. Most operations in finite fields use the modulo operator in some capacity.

=== Modulo Arithmetic in Python

Rust uses the integer method `rem_euclid()` operator for modulo arithmetic. Here is how the modulo operator is used:
#temp_fira_raw[```rs
println!("{}", 7u32.rem_euclid(3));
```]
Remember that in Rust we have to specify the integer type, eg _7.rem_euclid(3)_ would throw the error: 
#temp_fira_error[```sh
error[E0689]: can't call method `rem_euclid` on ambiguous numeric type `{integer}` ... 
you must specify a concrete type for this numeric value, like `i32`
```]

We can also use the modulo operator on negative numbers, like this:
#temp_fira_raw[```rs
println!("{}", (-27i64).rem_euclid(13));
```]
 