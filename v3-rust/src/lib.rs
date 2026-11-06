use wasm_bindgen::prelude::*;

/*
 * v3 Rust/WASM native integration.
 *
 * Exposed to JavaScript through wasm-bindgen so the typed client can call
 * `fibonacci(n)` directly instead of poking raw WebAssembly exports.
 * The MOD constant keeps results bounded for UI display.
 */

const MOD: i64 = 1_000_000;

#[wasm_bindgen]
pub fn fibonacci(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let mut a: i64 = 0;
    let mut b: i64 = 1;
    for _ in 2..=n {
        let c = (a + b) % MOD;
        a = b;
        b = c;
    }
    b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibonacci_zero_is_zero() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn fibonacci_one_is_one() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn fibonacci_ten_is_fifty_five() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn fibonacci_large_modulo() {
        // 1_000_000th fib is astronomically large; result is modulo MOD.
        assert_eq!(fibonacci(1_000_000), 48665);
    }
}