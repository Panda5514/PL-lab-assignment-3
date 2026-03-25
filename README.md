# Cobra Compiler

[cite_start]This repository contains a compiler for the **Cobra** programming language, which extends previous iterations with support for booleans, conditionals, loops, variable mutation, and runtime type checking[cite: 5, 6].

## Tagging Scheme

[cite_start]In Cobra, we use the **Least Significant Bit (LSB)** to distinguish between 63-bit integers and booleans within a 64-bit word[cite: 6].

* **Numbers**: Represented as `(n << 1)`. [cite_start]The LSB is always `0`[cite: 6]. [cite_start]For example, the number `5` becomes `10` (`0b1010`)[cite: 7].
* [cite_start]**Booleans**: The LSB is always `1`[cite: 7].
    * [cite_start]`false`: `0x01` (`0b0001`)[cite: 8].
    * [cite_start]`true`: `0x03` (`0b0011`)[cite: 8].

### Runtime Type Checking
The compiler performs runtime checks to ensure type safety:
* [cite_start]Arithmetic operations (like `+`, `-`, `*`) and comparisons (like `<`) verify that both operands have a `0` tag before proceeding[cite: 10, 40, 41, 44].
* [cite_start]Equality (`=`) ensures both operands have the same tag bit[cite: 11, 46].
* [cite_start]To verify if a value is a number, the runtime performs `value & 1`; a result of `0` indicates a number, while `1` indicates a boolean[cite: 9, 14, 15].

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
