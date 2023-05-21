struct Osoba {
    imie: String,
    wiek: u8,
}

impl ToString for Osoba {
    fn to_string(&self) -> String {
        return format!("Nazywa sie {}; ma lat {}", self.imie, self.wiek);
    }
}

impl Osoba {
    fn new(s: String) -> Osoba  {
        Osoba {imie: s, wiek:18}
    }
    fn postarz(&mut self) {
        self.wiek += 1;
    }
}

fn apply<T1, T2>(a: T1, b: T2) -> (T2, T1) {
    (b, a)
}

enum Liczba {
    Zero,
    Jeden, 
    Wiele(i32),
}

fn f(l: &Liczba) -> i32 {
    match l {
        Liczba::Zero => 0,
        Liczba::Jeden => 1,
        Liczba::Wiele(n) => *n,
    }
}

use std::fmt::format;

use Liczba::*;
// fn main() {
//     let li: Liczba = Zero;
//     f(&li);
//     println!("Wypisujemy {}", f(&li));
// }


fn main() {
    let o = Osoba::new(String::from("Maciek"));
    let mut v: Vec<i8> = Vec::new();
    Vec::push(&mut v, 6);
    v.push(6);
    println!("{}", o.to_string());
}