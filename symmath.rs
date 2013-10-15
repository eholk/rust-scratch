#[deriving(Clone)]
enum Expr {
    Var(~str),
    Add(~Expr, ~Expr),
    Mul(~Expr, ~Expr),
}

impl Expr {
//    fn simplify(&self) -> ~Expr {
//        match *self {
//            Mul(ref e1, ~Add(ref e2, ref e3)) =>
//                ~Add(~Mul(e1.simplify(), e2.simplify()),
//                     ~Mul(e1.simplify(), e3.simplify())),
//            _ => ~self.clone()
//        }
//    }

    fn simplify(&self) -> ~Expr {
        match *self {
            Mul(ref e1, ~Add(ref e2, ref e3)) =>
                e1.simplify() * e2.simplify() + e1.simplify() * e3.simplify(),
            _ => ~self.clone()
        }
    }
}

impl ToStr for Expr {
    fn to_str(&self) -> ~str {
        match *self {
            Var(ref s) => s.clone(),
            Add(ref e1, ref e2) =>
                e1.to_str() + " + " + e2.to_str(),
            Mul(~ref e1, ~ref e2) => {
                let e1 = match *e1 {
                    Add(*) => "(" + e1.to_str() + ")",
                    _ => e1.to_str()
                };
                let e2 = match *e2 {
                    Add(*) => "(" + e2.to_str() + ")",
                    _ => e2.to_str()
                };
                e1 + " * " + e2
            }
        }
    }
}

impl Mul<~Expr, ~Expr> for ~Expr {
    fn mul(&self, rhs: &~Expr) -> ~Expr {
        ~Mul(self.clone(), rhs.clone())
    }
}

impl Add<~Expr, ~Expr> for ~Expr {
    fn add(&self, rhs: &~Expr) -> ~Expr {
        ~Add(self.clone(), rhs.clone())
    }
}

fn constructors() {
    let e = ~Mul(~Var(~"a"), ~Add(~Var(~"b"), ~Var(~"c")));

    println!("{:s} = {:s}", e.to_str(), e.simplify().to_str());
}

fn operators() {
    let e = ~Var(~"a") * (~Var(~"b") + ~Var(~"c"));

    println!("{:s} = {:s}", e.to_str(), e.simplify().to_str());
}

fn main() {
    println!("Using constructors:");
    constructors();

    println!("");
    println!("Using operators:");
    operators();
}
