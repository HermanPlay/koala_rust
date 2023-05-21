enum nasztyp {
    xd(u8),
    xdd(f32, u64),
}

fn f(v: &nasztyp) -> u8 {
    match v { 
        nasztyp::xd(x) => *x,
        nasztyp::xdd(_, _) => 7
    }
}

fn main() {
    
}