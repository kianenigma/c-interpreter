# C-Interpreter

A minimal interpreter-like environment for quick C prototyping. Written in Rust.

> Motivation: I've always found it annoying that I can easily test a quick snippet in python or node's interpreter, while for C it is not as easy. If, similar to me, you don't code C every day then you might as well fail the first compile attempts due to the fact that you forgot the proper includes or main signature. This library prevents such hassles and make quickly testing a few lines of C code much easier.


GIF

# Installation

### Build from source


### Cargo


# How to Use

Simply insert normal statements and they will be treated as if they were inside an empty C `main()` function.

Aside from normal statements taht go inside the main function, the following are also supported:

  - includes simply insert `#include <foo.h>` etc.
  - defines. simply insert `#define FOO 10` etc.
  - functions. Must be prepended with `#fun`. Example: `#fun int f(int x) { return x/10; }`

### Command Set

The interpreter also supports a set of auxiliary commands. All of them start with `~`.

  - `~src`: Displays the current source code.
  - `~run`: Runs the current source code.
    - Note that this is executed implicitly after each successful statement insertion.
  - `~del <X>`: Deletes statement at index `<X>` from the source code. the indexes are shown as `(X)` in the `~src` commands.
  - `~arg <X?>`: Displays the current argv fed into the binary if `<X?>` is not provided. Otherwise, it sets `<X?>` as the new argv.
    - `?` in `<X?>` stands for optional parameter.
  - `~xcc <X?>`: Displays the current compiler used to generate the binary if `<X?>` is not provided. Otherwise, it sets `<X?>` as the new compiler. Default is `gcc`.
    - Of course, it is your responsibility to make sure that whatever you define as the compiler is
      available in `$PATH`.
