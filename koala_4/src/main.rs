struct Fibonacci {
    stare: u64,
    mlodych: u64,
}

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci {stare: 0, mlodych: 1}
    }
}

impl std::iter::Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        *self = Fibonacci {
            stare: self.stare + self.mlodych, mlodych: self.stare
        };
        return  Some(self.stare);
    }
}

fn main() {
    let fib = Fibonacci::new();
    // for _ in 1..10 {
    //     println!("n: {}", fib.next().unwrap());
    // }
    // println!("answer: {}", fib.all(|n| n % 2 == 0));
    for n in fib.filter(|n| n % 2 == 0).take(5) {
        println!("n: {}", n);
    }
}
