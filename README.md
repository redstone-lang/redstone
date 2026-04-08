A minimal compiler with an LLVM backend. Visit [docs](./docs/README.md) to learn the language

## Syntax

- Primitive types: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize`, `f32`, `f64`, `bool`, `char`, `()`
- Default type is `i64` when no annotation is given
- Functions are declared using `fn`, with optional typed params and return type
- Variables using `let`, with optional type annotation; reassignment with `x = ...` and `x += ...` etc.
- `if`/`else` conditionals
- `while` loops with comparison operators: `<`, `>`, `<=`, `>=`, `==`, `!=`
- Implicit return: last expression in a function body without `;`
- Output using `print(...)`
- Comments using `//`

See [examples](./examples)

## CLI (`redc`)

The `redc` binary is the command-line interface to the compiler.

### Build

Compile a `.red` source file into a native executable:

```sh
redc build <file>
```

Options:

| Flag                    | Default     | Description                                     |
|-------------------------|-------------|-------------------------------------------------|
| `-o`, `--output <path>` | `a.out`     | Output executable path                          |
| `--target <triple>`     | host target | Target triple (e.g. `x86_64-unknown-linux-gnu`) |

Examples:

```sh
redc build main.red
redc build main.red --output my_program
redc build main.red --output my_program --target x86_64-unknown-linux-gnu
```

### Run

Compile and immediately execute a `.red` source file:

```sh
redc run <file>
```

Example:

```sh
redc run main.red
```

## Limitations

- No strings
- No recursion with non-trivial depth (no tail call optimization)
- `print` outputs integers as decimal, floats with `%g`, booleans as `0`/`1`, chars as their code point
