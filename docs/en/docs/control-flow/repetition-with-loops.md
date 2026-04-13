# Repetition with Loops

## `while` loop [✅ implemented]

Repeats a block as long as the condition is true.

```red
while <condition> {
    <statements>
}
```

```red
fn main() {
    let i = 0;
    while i < 5 {
        i += 1;
    }
    print(i); // 5
    return 0;
}
```

## `for` loop [❌ not implemented]

todo

```red
fn main() {
    for number in (1..4).rev() {
        print(number);
    }
    print(123);
    return 0;
}
```