# Control Flow

## `if` / `else` [✅ implemented]

Executes a block conditionally. The `else` branch is optional.

```red
if <condition> {
    <statements>
}
```

```red
if <condition> {
    <statements>
} else {
    <statements>
}
```

```red
fn max(a: i64, b: i64) -> i64 {
    let result = a;
    if b > a {
        result = b;
    }
    result
}
```

## `while` loop [✅ implemented]

Repeats a block as long as the condition is true.

```red
while <condition> {
    <statements>
}
```

```red
fn main() {
    let i = 0;
    while i < 5 {
        i += 1;
    }
    print(i); // 5
    return 0;
}
```

## Comparison operators [✅ implemented]

Used in conditions:

| Operator | Description           |
|----------|-----------------------|
| `<`      | Less than             |
| `>`      | Greater than          |
| `<=`     | Less than or equal    |
| `>=`     | Greater than or equal |
| `==`     | Equal                 |
| `!=`     | Not equal             |

Comparison expressions evaluate to `bool`.

## Assignment operators [✅ implemented]

Compound assignment operators are also supported:

| Operator | Equivalent    |
|----------|---------------|
| `+=`     | `x = x + ...` |
| `-=`     | `x = x - ...` |
| `*=`     | `x = x * ...` |
| `/=`     | `x = x / ...` |

## Implicit return [✅ implemented]

The last expression in a function body can be written without `return` and without a trailing `;`:

```red
fn fibonacci(n: u64) -> u64 {
    let a = 1;
    let b = 0;
    let count = 0;
    while count < n {
        let tmp = a + b;
        b = a;
        a = tmp;
        count += 1;
    }
    b
}
```
