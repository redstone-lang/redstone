# Redstone Language Documentation

Redstone is a minimal compiled language with an LLVM backend.

## Contents

- [Types](types) — primitive types
- [Variables](variables) — variable declarations
- [Functions](functions) — function declarations and calls
- [Expressions](expressions) — arithmetic and expressions
- [Control Flow](control-flow) — `if`/`else` conditionals, `while` loops, comparisons, assignment
- [Comments](comments) — comments syntax

## Quick Example

```red
fn main() {
    let x = 21 * 2;
    print(x); // 42
    return 0;
}
```
