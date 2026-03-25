# Cobra Tagging Scheme

In Cobra, we use the Least Significant Bit (LSB) to distinguish between 63-bit integers and booleans within a 64-bit word.

## Representation
- **Numbers**: Represented as `(n << 1)`. The LSB is always `0`.
  - Range: 63-bit signed integer.
  - Example: `5` becomes `10` (`0b1010`).
- **Booleans**: The LSB is always `1`.
  - `false`: `0x01` (`0b0001`)
  - `true`: `0x03` (`0b0011`)

## Runtime Type Checking
To verify if a value is a number, we perform `value & 1`.
- If the result is `0`, it is a number.
- If the result is `1`, it is a boolean.
Binary arithmetic operations (like `+`, `-`, `*`) and comparisons (like `<`) verify that both operands have a `0` tag before proceeding. [cite_start]Equality (`=`) ensures both operands have the same tag bit[cite: 25, 31].