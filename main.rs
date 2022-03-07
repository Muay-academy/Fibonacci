fn fib(n: u32) -> u32 {
    let mut prev: u32 = 0;
    let mut curr: u32 = 1;
    for _ in 1..n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    curr
}

fn main() {
    for n in 0..30 {
    println!("Fibonacci ({})= {}",n, fib(n));
  }
}