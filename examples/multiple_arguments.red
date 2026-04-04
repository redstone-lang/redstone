fn add(a: u64, b: u64) -> u64 {
    return a + b;
}

fn mul(a: u64, b: u64) -> u64 {
    a * b;
}

fn main() {
    let x = add(3, 4); // 7
    let y = mul(x, 2); // 14
    print(y);
    return 0;
}