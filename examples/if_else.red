fn max(a: i64, b: i64) -> i64 {
    let result = a;
    if b > a {
        result = b;
    }
    result
}

fn main() -> i32 {
    print(max(3, 7));   // 7
    print(max(10, 4));  // 10
    return 0;
}
