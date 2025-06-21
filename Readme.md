## Rust Version

<p>
The Rust language makes you a simple promise: if your program passes the compiler’s
checks, it is free of undefined behavior. Dangling pointers, double-frees, and null
pointer dereferences are all caught at compile time. Array references are secured with
a mix of compile-time and run-time checks, so there are no buffer overruns
</p>

### Features

* Rust memory management is handled by Rust without the need for a garbage collector
* If your code compiles, it will run without error
* Native cross-platform executables
* Helps enforce consistency which supports governance and makes onboarding easier
* Allows mentoring of developers to focus on areas other than defensive coding

```bash
rustc --version
cargo --version
```

## Create New Project

```bash
cargo new project-name
```

## Build project

```bash
cargo build
```

## Run project

```bash
cargo run
```

### Macro

function name contains exclamation called macro

```rust
println!();
```

### Rust closures

In the Rust programming language, closures are anonymous functions that can capture variables from the scope where they
are defined. Closures can be passed as arguments to functions,
returned as values from functions, and assigned to variables. Closures can be called multiple times or can be called
only once. Closures can be borrowed immutably, borrowed mutably,
or can take ownership of captured variables. Closures can be implemented using the Fn, FnMut, and FnOnce traits.

### Rust closure traits

In the Rust programming language, Fn, FnMut, and FnOnce are traits that define different behaviors for closures and
other types of function objects

1. The Fn trait represents closures that can be called multiple times and can be borrowed immutably. This trait has one
   associated method, call(&self, args), that takes a borrowed reference to self, and it can be implemented by any
   closure that meets these requirements.

2. The FnMut trait represents closures that can be called multiple times and can be borrowed mutably. This trait has one
   associated method, call_mut(&mut self, args), that takes a mutable reference to the self, and it can be implemented
   by any closure that meets these requirements.

3. The FnOnce trait represents closures that can be called only once. This trait has one associated method, call_once(
   self, args), that takes ownership of self, and it can be implemented by any closure that meets these requirements.

## Lifetime

<p>The principal objective of lifetimes is to prevent dangling references. That’s it! Keep this in mind especially when you’re deep in the vagaries of lifetimes.</p>

What is a dangling reference?<br> <p>A dangling reference occurs when a reference outlives a borrowed value. At that
time, the refer- ence points to invalid memory.</p>

| Command                   | Description                                                                           |
|---------------------------|---------------------------------------------------------------------------------------|
| cargo new <project_ name> | Creates a new Rust project in a new directory.                                        |
| cargo init                | Initializes a new Rust project in the current directory.                              |
| cargo build               | Compiles the current project and all of its depen- dencies.                           |
| cargo run                 | Compiles and runs the current project.                                                |
| cargo test                | Runs the tests for the current project.                                               |
| cargo check               | Quickly checks your code to ensure it compiles but does not produce an executable.    |
| cargo clean               | Removes the target directory with the compiled artifacts.                             |
| cargo update              | Updates dependencies as recorded in the local lock file.                              |
| cargo doc                 | Generates documentation for the current project‘s dependencies.                       |
| cargo publish             | Packages and uploads the current project to crates.io.                                |
| cargo bench               | Runs the benchmarks of the current project. (Note: Requires a nightly build of Rust.) |

## Compile Rust program
To compile Rust source files Main.rs and Lib.rs into object files and then link them into a final executable, you need to use the rustc command-line tool.
```shell
rustc --emit=obj -o Main.o Main.rs

rustc --emit=obj -o Lib.o Lib.rs

rustc Main.o Lib.o -o MyProgram
```
