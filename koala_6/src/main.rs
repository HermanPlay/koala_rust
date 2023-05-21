fn fresh(vars: Vec<String>) -> String {
    let letters = "abcdefghijklmnopqrstuvwxyz";
    for i in 0.. {
        for c in letters.chars() {
            let new_var = match i {
                0 => format!("{}", c),
                _ => format!("{}{}", c, i),
            };
            if vars.contains(&new_var) {
                return new_var;
            }
        }
    }
    unreachable!();
}
#[derive(Clone)]
enum Term {
    Var(String),
    Lam(String, Box<Term>),
    App(Box<Term>, Box<Term>),
}

impl ToString for Term {
    fn to_string(&self) -> String {
        match self {
            Term::Var(v) => v.clone(),
            Term::Lam(v, b) => format!("λ{}. {}", v, b.to_string()),
            Term::App(e1, e2) => format!("({} {})", e1.to_string(), e2.to_string()),
        }
    }
}

impl Term {
    fn fv(&self) -> Vec<String> {
        match self {
            Term::Var(v) => vec![v.clone()],
            Term::Lam(v, b) => b.fv().into_iter().filter(|u| u != v).collect(),
            Term::App(e1, e2) => e1.fv().into_iter().chain(e2.fv().into_iter()).collect(),
        }
    }
    fn rename(&mut self, x: &String, y: &String) {
        match self {
            Term::Var(v) => match v == x {
                true => *self = Term::Var(y.clone()),
                false => {}
            },
            Term::Lam(v, b) => match v == x {
                true => {}
                false => b.as_mut().rename(x, y),
            },
            Term::App(e1, e2) => {
                e1.as_mut().rename(x, y);
                e2.as_mut().rename(x, y)
            }
        }
    }
    fn subst(&mut self, x: &String, y: &Term) {
        match self {
            Term::Var(z) => match z == x {
                true => *self = y.clone(),
                false => {}
            },
            Term::Lam(v, b) => {
                let fvs = y.fv(); // fvs = free variables

                match fvs.contains(v) {
                    true => {
                        let v2 = fresh(
                            vec![v.clone()]
                                .into_iter()
                                .chain(fvs)
                                .chain(b.fv())
                                .collect(),
                        ); // wymyslec nowe v
                        b.as_mut().rename(v, &v2); // podstawiamy v na nowe v
                        b.as_mut().subst(x, y); // podstawiamy v -> nowe_v na b
                        *self = Term::Lam(v2, b.clone());
                    }
                    false => b.as_mut().subst(x, y),
                }
            }
            Term::App(e1, e2) => {
                e1.as_mut().subst(x, y);
                e2.as_mut().subst(x, y);
            }
        }
    }

    fn normalize(&mut self) {
        // call by name
        match self {
            Term::Var(_) => {}
            Term::Lam(_, b) => b.normalize(),
            Term::App(e1, e2) => {
                e1.normalize();
                match e1.as_mut() {
                    Term::Var(_) => {}
                    Term::Lam(v, b) => {
                        b.subst(v, e2);
                        *self = b.as_mut().clone();
                    }
                    Term::App(_, _) => {}
                }
            }
        }
    }
}

macro_rules! app {
    ($x: expr, $y: expr) => {
        Term::App(Box::new($x), Box::new($y))
    };
}

macro_rules! lam {
    ($v: expr, $b: expr) => {
        Term::Lam(String::from($v), Box::new($b))
    };
}

macro_rules! var {
    ($v: expr) => {
        Term::Var(String::from($v))
    };
}

fn main() {
    let expr = app!(lam!("x", app!(var!("x"), var!("y"))), var!("z"));
    let mut expr2 = expr.clone();
    expr2.normalize();
    println!("{} --> {}", expr.to_string(), expr2.to_string());
}


// zrobic zeby liczylo cos bardzo dlugpo 2^64, funkcja akermana,  