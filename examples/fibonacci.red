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
    print(fibonacci(32));
    return 0;
}
