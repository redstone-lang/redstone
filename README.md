A minimal compiler with an LLVM backend. Visit [docs](./docs/README.md) to learn the language

## Syntax

- Primitive types: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize`, `f32`, `f64`, `bool`, `char`, `()`
- Default type is `i64` when no annotation is given
- Functions are declared using `fn`, with optional typed params and return type
- Variables using `let`, with optional type annotation; reassignment with `x = ...` and `x += ...` etc.
- `while` loops with comparison operators: `<`, `>`, `<=`, `>=`, `==`, `!=`
- Implicit return: last expression in a function body without `;`
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
    print(sum);  // 13
    print(diff); // 7
    print(prod); // 30
    print(quot); // 3
    return 0;
}
```

### Typed variables and floats

```
fn main() {
    let x: i32 = 100;
    let pi: f64 = 3.14;
    let flag = true;
    let letter = 'A';
    print(x);      // 100
    print(pi);     // 3.14
    print(flag);   // 1
    print(letter); // 65
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

### Typed function

```
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn main() {
    let r = add(10, 20);
    print(r); // 30
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

### Fibonacci

```
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

fn main() -> i32 {
    print(fibonacci(32)); // 2178309
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

## CLI (`rsc`)

The `rsc` binary is the command-line interface to the compiler.

### Build

Compile a `.red` source file into a native executable:

```
rsc build <file>
```

Options:

| Flag | Default | Description |
|------|---------|-------------|
| `-o`, `--output <path>` | `a.out` | Output executable path |
| `--target <triple>` | host target | Target triple (e.g. `x86_64-unknown-linux-gnu`) |

Examples:

```
rsc build main.red
rsc build main.red --output my_program
rsc build main.red --output my_program --target x86_64-unknown-linux-gnu
```

### Run

Compile and immediately execute a `.red` source file:

```
rsc run <file>
```

Example:

```
rsc run main.red
```

## Limitations

- No conditions (`if`)
- No strings
- No recursion with non-trivial depth (no tail call optimization)
- `print` outputs integers as decimal, floats with `%g`, booleans as `0`/`1`, chars as their code point
