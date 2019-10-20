```rust
// Generate the nth fibonacci number.
use std::io;

fn main() {
    let mut requested_fib = String::new();

    println!("What Fibo you want");

    io::stdin()
        .read_line(&mut requested_fib)
        .expect("Please enter a number");

    let requested_fib: u32 = match requested_fib.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let result = get_fib_recursive(requested_fib);
    let result2 = get_fib_bottom_up(requested_fib);
    println!("Result recursive {}", result);
    println!("Result bottom-up {}", result2);
}

fn get_fib_recursive(requested_fib: u32) -> u32 {
    const F0: u32 = 0;
    const F1: u32 = 1;

    if requested_fib == 0 {
        F0
    } else if requested_fib == 1 {
        F0 + F1
    } else {
        get_fib_recursive(requested_fib - 1) + get_fib_recursive(requested_fib - 2)
    }
}

fn get_fib_bottom_up(requested_fib: u32) -> u32 {
    if requested_fib == 0 || requested_fib == 1 {
        requested_fib
    } else {
        let mut flags: (u32, u32) = (0, 1);

        for _ in 2..requested_fib + 1 {
            flags = (flags.1, flags.0 + flags.1)
        }

        flags.1
    }
}
```
