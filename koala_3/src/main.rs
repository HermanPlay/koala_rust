struct A {
    x: i8,
    y: u8,
}

struct B {
    n: i32,
    a: A,
}

trait HasA {
    fn take_a(x: Self) -> A;
}

impl HasA for A {
    fn take_a(x: A) -> A {
        return x;
    }
}

impl HasA for B {
    fn take_a(x: B) -> A {
        return x.a;
    }
}

fn f<T: HasA>(x: T) {
    println!("!!!{}!!!", T::take_A(x).x); // take_A to metoda
}

fn main() {
    let za = A {x: 0, y: 0};
    let zb = B {n: 0, a: A {x: 1, y: 1}};
    f(za);
    f(zb);
}
