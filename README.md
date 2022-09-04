# bedmap: Constructing Composite Iterators In The Presence Of Errors
Bart Massey 2022

This library crate contains a function `bed_map()` that
iterates over a couple of streams and produces a new
stream. See the Rustdoc for details. See
`examples/bedmap.rs` for an example usage.

This Rust crate started out as a "quick hack" in response to
this
[Reddit post](https://www.reddit.com/r/learnrust/comments/x54oq2/nesting_iterators/).
Many hours later I had been fully reminded of a what a mess
Rust iterator programming can be, especially when dealing
with possible errors.

This work is made available under the "MIT License". Please
see the file `LICENSE.txt` in this distribution for license
terms.
