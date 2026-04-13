# Expressions

## Arithmetic Operators [✅ implemented]

| Operator | Description                               |
|----------|-------------------------------------------|
| `+`      | Addition                                  |
| `-`      | Subtraction                               |
| `*`      | Multiplication                            |
| `/`      | Division (integer, truncates toward zero) |

## Comparison Operators [✅ implemented]

| Operator | Description           |
|----------|-----------------------|
| `<`      | Less than             |
| `>`      | Greater than          |
| `<=`     | Less than or equal    |
| `>=`     | Greater than or equal |
| `==`     | Equal                 |
| `!=`     | Not equal             |

Comparison expressions evaluate to `bool`.

```red
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

## Operator Precedence [✅ implemented]

Standard math precedence applies: `*` and `/` bind tighter than `+` and `-`.

```red
let x = 2 + 3 * 4; // 14, not 20
```

Use parentheses to control evaluation order:

```red
let x = (2 + 3) * 4; // 20
```

## Literals [✅ implemented]

Integer literals are decimal numbers, optionally negative:

```red
let a = 42;
let b = -7;
```

Float literals require a decimal point and default to `f64`:

```red
let pi = 3.14;
let half: f32 = 0.5;
```
