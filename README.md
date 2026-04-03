A minimal compiler with an LLVM backend. The source is parsed using pest, LLVM IR is generated using inkwell, and the result is linked into a native executable.

## Syntax

- All values ​​are integers (`i64`)
- Functions are declared using `fn`
- Variables using `let`
- Output using `print(...)`
- Comments using `//`

## Examples

### Hello, number

```
fn main() {
    print(42);
    return 0;
}
```

### Variables and Arithmetic

```
fn main() {
    let a = 10;
    let b = 3;
    let sum = a + b;
    let diff = a - b;
    let prod = a * b;
    let quot = a / b;
    print(sum); // 13 
    print(diff); // 7 
    print(prod); // 30 
    print(quot); // 3 
    return 0;
}
```

### Function call

```
fn square(x) { 
    return x * x;
}

fn main() { 
let result = square(9); 
    print(result); // 81 
    return 0;
}
```

### Multiple arguments

```
fn add(a, b) { 
    return a + b;
}

fn mul(a, b) { 
    return a * b;
}

fn main() { 
    let x = add(3, 4); // 7 
    let y = mul(x, 2); // 14 
    print(y); 
    return 0;
}
```

### Nested Calls

```
fn double(x) {
    return x * 2;
}

fn inc(x) {
    return x + 1;
}

fn main() {
    print(double(inc(5))); // 12
    return 0;
}
```

### Expressions in Arguments

```
fn add(a, b) {
    return a + b;
}

fn main() {
    let x = add(2 * 3, 10 - 4); // add(6, 6) = 12
    print(x);
    return 0;
}
```

## Limitations

- The only data type is `i64`
- No conditions (`if`) or loops (`while`)
- No strings
- No recursion with non-trivial depth (the stack is unlimited, but there is no tail call optimization)
- `print` prints one number per call