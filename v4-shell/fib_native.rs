
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n: u32 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(20);
    let mut a: u32 = 0;
    let mut b: u32 = 1;
    for _ in 0..n {
        let c = a.wrapping_add(b);
        a = b;
        b = c;
    }
    println!("{}", a);
}
