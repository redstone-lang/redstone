# Expressions

## Arithmetic Operators

| Operator | Description                               |
|----------|-------------------------------------------|
| `+`      | Addition                                  |
| `-`      | Subtraction                               |
| `*`      | Multiplication                            |
| `/`      | Division (integer, truncates toward zero) |

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

## Assignment Operators

Compound assignment operators combine an arithmetic operation with reassignment:

| Operator | Equivalent   |
|----------|--------------|
| `+=`     | `x = x + y`  |
| `-=`     | `x = x - y`  |
| `*=`     | `x = x * y`  |
| `/=`     | `x = x / y`  |

```red
fn main() {
    let x = 10;
    x += 5;  // 15
    x -= 3;  // 12
    x *= 2;  // 24
    x /= 4;  // 6
    print(x);
    return 0;
}
```

## Comparison Operators

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
    print(1 < 2);  // 1
    print(1 > 2);  // 0
    print(1 == 1); // 1
    print(1 != 2); // 1
    return 0;
}
```

## Operator Precedence

Standard math precedence applies: `*` and `/` bind tighter than `+` and `-`.

```red
let x = 2 + 3 * 4; // 14, not 20
```

Use parentheses to control evaluation order:

```red
let x = (2 + 3) * 4; // 20
```

## Literals

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
