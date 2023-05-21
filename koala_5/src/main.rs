// enum Lista<T>
// {
//     Cons(T, Rc<Lista<T>>),
//     Nil
// }

// use std::rc::Rc;
// use std::cell::*;
// use Lista::*;

// fn main() {
//     let c = Cell::new(5);
//     {
//         println!("{}", c.get());
//         let a = &c;
//         let b = &c;
//         b.set(10);
//         println!("{}", c.get());
//     }
// }


/////////////////////////////////////////////////////////////////


fn chrup<T>(t: T) {}

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str 
{
    if(a.len() > b.len()) {
        return a;
    } else {
        return b;
    }
}

fn xd<'a, 'b>(a: &'a str, b: &'b str) -> &'a str{
    if(b.len() % 2 == 0) {
        return &a[1..3];
    }
    return &a[2..3];
}

fn f<'a>(v: &'a [i32]) -> &'a i32 {
    return &v[0];
}


fn main() {
    let mut v = vec![3,1, 4, 5, 6, 7, 8];
    let s = &v[1..3];
    let x = f(s);
    chrup(v);
    println!("{}", x);

    if 2 + 2 == 4 {
        todo!();
    }
    else {
        todo!();
    }

    // f(&v);
    // let s = &mut v[1..3];
    // for i in s {
    //     println!("{}", i);
    // }
    // v[3] = 10
}