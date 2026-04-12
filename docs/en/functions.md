# Functions

## Declaration [✅ implemented]

```red
fn <name>(<params>) -> <return_type> {
    <body>
}
```

Parameters are comma-separated and **require type annotations**. The return type after `->` is optional and defaults to `()`.

```red
fn add(a: i64, b: i64) -> i64 {
    return a + b;
}
```

## Return type [✅ implemented]

Explicit return type is written after `->`:

```red
fn square(x: u64) -> u64 {
    return x * x;
}
```

Omitting `->` is equivalent to `-> ()`:

```red
fn greet() {
    print(72); // 'H'
}
```

## Implicit return

The last expression in a function body without a trailing `;` is the return value:

```red
fn mul(a: u64, b: u64) -> u64 {
    a * b
}
```

## Entry Point

Every program must define a `main` function. It should return `0`.

```red
fn main() -> i32 {
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

`print` is a built-in that prints a single value to stdout. It accepts any primitive type:

- integers — printed as decimal
- floats — printed with `%g`
- `bool` — printed as `0` or `1`
- `char` — printed as its numeric code point

```red
print(42);
print(3.14);
print(true);  // 1
print('A');   // 65
```
