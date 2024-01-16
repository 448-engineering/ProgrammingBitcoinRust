#import "template.typ": *
#show: ams-article.with(
  title: [Programming Bitcoin],
  authors: (
    (
      name: "PROGRAMMING BITCOIN USING RUST LANGUAGE",
      url: "https://github.com/jimmysong/programmingbitcoin"
    ),
  ),
  abstract: "This book will teach you the technology of Bitcoin at a fundamental level. It doesn’t cover the monetary, economic, or social dynamics of Bitcoin, but knowing how Bitcoin works under the hood should give you greater insight into what’s possible.
There’s a tendency to hype Bitcoin and blockchain without really understanding what’s going on; this book is meant to be an antidote to that tendency. After all, there are lots of books about Bitcoin, covering the history and the economic aspects and giving technical descriptions. The aim of this book is to get you to understand Bitcoin by coding all of the components necessary for a Bitcoin library. Thelibrary is not meant to be exhaustive or efficient. The aim of the library is to help you
learn.
",
  bibliography-file: "refs.bib",
)

#block([
  *LICENSE*
  
  The original contents based on Programming Bitcoin Book are licensed under #link("https://creativecommons.org/licenses/by-nc-nd/4.0/legalcode")[CC-BY-NC-ND] while all Rust code is released to the public domain under #link("https://creativecommons.org/publicdomain/zero/1.0/")[CC0 1.0] with the copyright notice.  

  *CC0 1.0 (No Copyright)*
  
 The person who associated a work with this deed has dedicated the work to the public domain by waiving all of his or her rights to the work worldwide under copyright law, including all related and neighboring rights, to the extent allowed by law.
You can copy, modify, distribute and perform the work, even for commercial purposes, all without asking permission.


In no way are the patent or trademark rights of any person affected by CC0, nor are the rights that other persons may have in the work or in how the work is used, such as #link("https://creativecommons.org/publicdomain/zero/1.0/#ref-publicity-rights")[publicity or privacy] rights.

Unless expressly stated otherwise, the person who associated a work with this deed makes no warranties about the work, and disclaims liability for all uses of the work, to the fullest extent permitted by applicable law.
When using or citing the work, you should not imply #link("https://creativecommons.org/publicdomain/zero/1.0/#ref-endorsement")[endorsement] by the author or the affirmer.
]
)
#pagebreak()
#set align(center)
#image("images/golden-btc-coins.jpg", width: 80%)

#set align(left)
* Introduction *

This book will teach you how to program Bitcoin from scratch using the #link("https://www.rust-lang.org/")[
 Rust Programming Language.
]
The original Book by Jimmy Song has code written in Python. This book rewrites that code in Rust Programming Language.

* What you need to know? *

This book assumes you have installed the Rust Programming Language stable toolchain (at least version 1.75) and that you have read through the Rust programming language book. Install the Rust stable toolchain from #link("https://www.rust-lang.org/tools/install"). You can find the book at #link("https://doc.rust-lang.org/book/").

#show emph: it => {
  text(rgb("#127885ff"), it.body)
}

*RECOMMENDATION*

_Install Sccache crate to improve developer productivity by caching files that don't need to be rebuilt. Install `sccache` from_ #link("https://crates.io/crates/sccache").

#include "./Chapter1/summary.typ"