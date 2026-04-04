fn double(x: u64) -> u64 {
    return x * 2;
}

fn inc(x: u64) -> u64 {
    return x + 1;
}

fn main() {
    print(double(inc(5))); // 12
    return 0;
}