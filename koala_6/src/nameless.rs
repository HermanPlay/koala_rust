#[derive(Clone)]
enum Term {
    Var(u32),
    Lam(Box<Term>),
    App(Box<Term>, Box<Term>),
}