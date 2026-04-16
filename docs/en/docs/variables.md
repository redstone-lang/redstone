# Variables

Variables are declared with `let`. They can be reassigned after declaration.

```red
let <name>[: <type>] = <expression>;
```

```red
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

## Reassignment

A variable declared with `let` can be reassigned:

```red
x = <expression>;
```

Compound assignment operators are also available: `+=`, `-=`, `*=`, `/=`. See [Expressions](expressions.md) for details.
