# Activity: your first activity

Activities allow you to test you knowledge by practicing what you've learned. These take the form of small Rust projects that slowly build up on each other into something more advanced.

> The goal of these is to eventually have you writing you own Blockchain from scratch, and being able to say you did it all yourself!

## Starting an activity

You can find the code for activities on [github](https://github.com/Trantorian1/notes-rust-blockchain/). Create a [fork](https://github.com/Trantorian1/notes-rust-blockchain/fork) of the repository and `cd` into the `activities` folder to complete each set of exercises.

Activities assume you are using the latest stable version of Rust and will provide all needed dependencies. Feel free to add new dependencies as you see fit, just try not to use anything that makes the job too easy for you. All tests will be run locally on your machine so that is not a problem.

> A [sample activity](https://github.com/Trantorian1/notes-rust-blockchain/tree/main/activities/ex00) is provided for you to make sure everything compiles on your machine. Make sure that works before moving on!

## Activity 1: Multiple choice quiz

Since the first section of this book has so far been more focused on abstracts concepts surrounding Blockchain, our first activity will not be a hands-on project.

> Don't worry, the next activity will be hands on!

```rust
{{#include ../activities/ex01/src/lib.rs:q0}}

{{#include ../activities/ex01/src/lib.rs:q1}}

{{#include ../activities/ex01/src/lib.rs:q2}}

{{#include ../activities/ex01/src/lib.rs:q3}}

{{#include ../activities/ex01/src/lib.rs:q4}}

{{#include ../activities/ex01/src/lib.rs:q5}}

{{#include ../activities/ex01/src/lib.rs:q6}}

{{#include ../activities/ex01/src/lib.rs:q7}}

{{#include ../activities/ex01/src/lib.rs:q8}}

{{#include ../activities/ex01/src/lib.rs:q9}}

{{#include ../activities/ex01/src/lib.rs:q10}}

{{#include ../activities/ex01/src/lib.rs:q11}}
```

## Checking your answers

1. To make sure you haven't made any mistake in the _format_ of your answers:
```bash
cargo test
```

2. To make sure your answers are correct:
```bash
cargo test --features solutions
```
