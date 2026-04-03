# Expressions

## Arithmetic Operators

| Operator | Description    |
|----------|----------------|
| `+`      | Addition       |
| `-`      | Subtraction    |
| `*`      | Multiplication |
| `/`      | Division (integer, truncates toward zero) |

```
fn main() {
    let a = 10;
    let b = 3;
    print(a + b); // 13
    print(a - b); // 7
    print(a * b); // 30
    print(a / b); // 3
    return 0;
}
```

## Operator Precedence

Standard math precedence applies: `*` and `/` bind tighter than `+` and `-`.

```
let x = 2 + 3 * 4; // 14, not 20
```

Use parentheses to control evaluation order:

```
let x = (2 + 3) * 4; // 20
```

## Literals

Integer literals are decimal numbers, optionally negative:

```
let a = 42;
let b = -7;
```
