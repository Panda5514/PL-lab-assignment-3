# Cobra Compiler

This repository contains a compiler for the **Cobra** programming language, which extends previous iterations with support for booleans, conditionals, loops, variable mutation, and runtime type checking.

## Tagging Scheme

In Cobra, we use the **Least Significant Bit (LSB)** to distinguish between 63-bit integers and booleans within a 64-bit word.

* **Numbers**: Represented as `(n << 1)`. The LSB is always `0`. For example, the number `5` becomes `10` (`0b1010`).
* **Booleans**: The LSB is always `1`.
    * `false`: `0x01` (`0b0001`).
    * `true`: `0x03` (`0b0011`).

### Runtime Type Checking
The compiler performs runtime checks to ensure type safety:
* Arithmetic operations (like `+`, `-`, `*`) and comparisons (like `<`) verify that both operands have a `0` tag before proceeding.
* Equality (`=`) ensures both operands have the same tag bit.
* To verify if a value is a number, the runtime performs `value & 1`; a result of `0` indicates a number, while `1` indicates a boolean.

---

## Language Syntax

The concrete syntax for Cobra is as follows:

```lisp
<expr> :=
  | <number>
  | true | false
  | input
  | <identifier>
  | (let ((<identifier> <expr>)+) <expr>)
  | (add1 <expr>) | (sub1 <expr>) | (negate <expr>)
  | (+ <expr> <expr>) | (- <expr> <expr>) | (* <expr> <expr>)
  | (< <expr> <expr>) | (> <expr> <expr>) 
  | (<= <expr> <expr>) | (>= <expr> <expr>) | (= <expr> <expr>)
  | (isnum <expr>) | (isbool <expr>)
  | (if <expr> <expr> <expr>)
  | (block <expr>+)
  | (loop <expr>)
  | (break <expr>)
  | (set! <identifier> <expr>)
