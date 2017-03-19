# subint

> Operations on a "partial" integer.

This library allows slightly more fluent expressions like
`subint.of(4).invert(x)` to invert the last 4 bits of the integer `x`,
or `subint.of(5).permute(3)` to iterate over all permutations of
3 bits within the last 5 bits of an integer.

## Table of Contents

- [Install](#install)
- [Usage](#usage)
- [Performance](#performance)
- [TODOs](#todos)
- [NOTDOs](#notdos)
- [Contribute](#contribute)

## Install

Add at an appropriate position to your `Cargo.toml`:

```TOML
[dependencies]
subint = { git = "https://github.com/BenWiederhake/subint.git" }
```

That should be it.  You'll be glad to hear that `subint` itself
does not have any dependencies.

### Additional step for best performance

For best performance, you should allow `rustc` (or in this case, LLVM actually)
to use special instructions that can speed up execution even more.
Specifically, this library makes extensive use of `u32::count_ones()`,
which could be compiled to the single special-purpose instruction `popcnt`.

To enable this instruction, add this to your `.cargo/config` file
[somewhere up the tree](http://doc.crates.io/config.html#hierarchical-structure):

```TOML
[build]
rustflags = ["-C", "target-feature=+popcnt"]
#rustflags = ["-C", "target-cpu=native"]
```

Feel free to be even more specific about the target architecture.
I only highlighted this singular instruction, as it is available
on all common architectures, and has the most impact, as far as the
current benchmarks are concerned.

<!--
  Assuming that the processor doesn't already recognize the pattern and
  optimize on its own.  In this case, `popcnt` might still be of advantage
  because of the limited instruction cache.
  The "bitcount hack" is pretty long!
-->

## Usage

Just use it!  No dependencies, and it's short enough.
The complexity lies in constructing an algorithm,
not in writing the code.

```Rust
extern crate subint;

for x in subint::of(5).permute(3) {
    // Prints all permutations of 3 bits where only the last 5 bits are considered.
    println!("{:032b}", x);
}
```

## Performance

FIXME

## TODOs

Next up are these:
* Fiddle around and analyse performance with:
    * Struct order?
    * Store the mask instead of recomputing it all the time?
* Ask people for feedback on:
    * Performance methodology, performance improvements
    * Making it "Idiomatic Rust"
    * Other Iterator-like interfaces that might be interesting to provide

## NOTDOs

Here are some things this project will definitely not support:
* Permutations over arbitrary bitmasks.  See [separate library](https://github.com/BenWiederhake/masked_permute/).

These are highly unlikely, but if you actually need them and can tell me
how to elegantly implement it (traits like `From<u32>` or so, I guess?),
I might look into it:
* Build for different kinds of integers
* Try to compile with `nostdlib` or whatever that's called
* Find out other ways to make it attractive for
  micro-controller-like environments, too.

## Contribute

Feel free to dive in! [Open an issue](https://github.com/BenWiederhake/subint/issues/new) or submit PRs.
