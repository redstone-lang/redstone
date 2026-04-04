# Functions

## Declaration

```red
fn <name>(<params>) {
    <body>
    return <expression>;
}
```

Parameters are comma-separated names with no type annotations.

```red
fn add(a, b) {
    return a + b;
}
```

## Entry Point

Every program must define a `main` function. It should return `0`.

```red
fn main() {
    return 0;
}
```

## Calling Functions

```red
let result = add(3, 4);
```

Arguments can be arbitrary expressions:

```red
let x = add(2 * 3, 10 - 4); // add(6, 6) = 12
```

Calls can be nested:

```red
print(double(inc(5))); // 12
```

## Output

`print` is a built-in that prints a single `i64` value to stdout:

```red
print(42);
print(result);
```
