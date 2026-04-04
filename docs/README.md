# Redstone Language Documentation

Redstone is a minimal compiled language with an LLVM backend.

## Contents

- [Types](types.md) — primitive types
- [Variables](variables.md) — variable declarations
- [Functions](functions.md) — function declarations and calls
- [Expressions](expressions.md) — arithmetic and expressions
- [Control Flow](control-flow.md) — `while` loops, comparisons, assignment
- [Comments](comments.md) — comments syntax

## Quick Example

```red
fn main() {
    let x = 21 * 2;
    print(x); // 42
    return 0;
}
```
