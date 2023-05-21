struct Osoba {
    imie: String,
    wiek: u8,
}

enum Expr {
    Liczba(i64),
    suma(Box<Expr>, Box<Expr>),
}

enum List<T> {
    pusta, // oznacza gdzioe sie konczy lista
    cons(T, Box<List<T>>), // constructor
}

use crate::List::cons;

#[test]
fn test_list() {
    let l: List<f32> = cons(3.5, Box::new(cons(6.0, Box::new(pusta))));
    assert!(2 + 2 == 4);
} 

fn fib(n: u64) -> u64 {
    return if n <= 1{
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }

}

fn main() {
    println!("{}", fib(8));
    println!("Hello, World!");
}