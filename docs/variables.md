# Variables

Variables are declared with `let`. They are immutable after assignment.

```
let <name>[: <type>] = <expression>;
```

```
fn main() {
    let a = 10;
    let b = a * 2;
    print(b); // 20
    return 0;
}
```

- No type annotation required — defaults to `i64`; use `: Type` to specify another type
- The right-hand side can be any expression, including function calls
- Variables are scoped to the function they are declared in
