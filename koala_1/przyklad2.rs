struct Osoba {
    imie: String,
    wiek: u8,
}

fn f(o: &Osoba) {
    println!("{} {}", o.imie, o.wiek);
}

fn starzej(o: &mut Osoba) {
    o.wiek = o.wiek + 1;
}

fn wypisz(n: u64) {
    println!("{}", n);
}

fn plus3(n:u64) -> u64 {
    n + 3 // z ; bedzie statement, bez wyrazenie, i automatycznie zwraca
}

fn main() {
    // let mut o = Osoba {imie: String::from("max"), wiek: 9};
    // starzej(&mut o);
    // f(&o);
    // o = Osoba {imie: String::from("maciej"), wiek: 9};
    // f(&o);
    // for i in 0..=5 {
    //     println!("{}", i);
    // }
    // let mut i = 0;
    // while i  <  5 {
    //     println!("{}", i);
    //     i += 1;
    // }

    let a = {
        for i in 0..3 {
            println!("XD");
        }
        7
    };

}