# Types

Redstone supports all Rust primitive types.

## Integers

| Type    | Size     | Range |
|---------|----------|-------|
| `i8`    | 8-bit    | −128 … 127 |
| `i16`   | 16-bit   | −32 768 … 32 767 |
| `i32`   | 32-bit   | −2 147 483 648 … 2 147 483 647 |
| `i64`   | 64-bit   | −9.2×10¹⁸ … 9.2×10¹⁸ |
| `i128`  | 128-bit  | very large |
| `isize` | pointer  | platform-dependent |
| `u8`    | 8-bit    | 0 … 255 |
| `u16`   | 16-bit   | 0 … 65 535 |
| `u32`   | 32-bit   | 0 … 4 294 967 295 |
| `u64`   | 64-bit   | 0 … 1.8×10¹⁹ |
| `u128`  | 128-bit  | very large |
| `usize` | pointer  | platform-dependent |

Integer literals are written as plain decimal numbers: `42`, `0`, `255`.

## Floating point

| Type  | Size   |
|-------|--------|
| `f32` | 32-bit |
| `f64` | 64-bit |

Float literals require a decimal point: `3.14`, `0.5`, `1.0`.

## Boolean

`bool` — either `true` or `false`.

```red
let flag = true;
let other = false;
```

## Char

`char` — a Unicode scalar value (4 bytes, stored as `u32`).

Literals use single quotes:

```red
let letter = 'A';
let emoji  = '\u{1F600}';
```

`print` outputs the numeric code point value.

## Unit

`()` — the unit type. Its only value is `()`. Used as the return type of functions that produce no value.

```red
fn greet() -> () {
    print(72);  // 'H'
}
```

## Default type

When no type annotation is written, the type defaults to `i64` for backwards compatibility.

## Type annotations

Type annotations are required wherever the type cannot be inferred from the value alone.

- Float literals (`3.14`) default to `f64`; use `f32` annotation to override
- `bool`, `char`, `()` literals are always unambiguous
- Integer literals are ambiguous — a type annotation or context is required:

```red
let x: i32 = 10;        // annotation on let
let y: i64 = x * 2;     // 2 is resolved from x's type
let z = square(9);      // 9 is resolved from the param type of square
```

Function parameters always require a type annotation. Return type is optional and defaults to `()`:

```red
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
```
