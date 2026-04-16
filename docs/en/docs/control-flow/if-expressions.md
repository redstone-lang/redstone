# `if` Expressions

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
