/// Below you find a small start of a data type modelling the abstract syntax tree for an expression,
/// and a small evaluator function.
///
/// Please extend this evaluator in the following ways:
///
/// - Add support for multiplication and division
///
/// - We have added the form "Summation(Vec<Expr>)", representing the sum of a list of expressions.
/// Question: why can we get away with Vec<Expr> enough in that case, instead of Box<Vec<Expr>> ?
///
/// - EXTRA: Since division can fail, the function eval needs to return an Option<i64>, where None indicates that a division by
///   zero has occurred. Can you change the code so that that errors are propagated correctly? (hint: use the ? syntax).

#[derive(PartialEq, Debug)]
enum Expr {
    Const(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),  // New variant for multiplication
    Div(Box<Expr>, Box<Expr>),  // New variant for division
    Var,
    Summation(Vec<Expr>),
}

// inject these two identifiers directly into the current namespace
use Expr::{Const, Summation, Var};

// These are convenience functions, so you don't have to type "Box::new" as often
// when building test-data types
fn add(x: Expr, y: Expr) -> Expr {
    Expr::Add(Box::new(x), Box::new(y))
}

fn sub(x: Expr, y: Expr) -> Expr {
    Expr::Sub(Box::new(x), Box::new(y))
}

fn mul(x: Expr, y: Expr) -> Expr {
    Expr::Mul(Box::new(x), Box::new(y))
}

fn div(x: Expr, y: Expr) -> Expr {
    Expr::Div(Box::new(x), Box::new(y))
}

// ...

fn eval(expr: &Expr, var: i64) -> Option<i64> {
    use Expr::*;
    match expr {
        Const(k) => Some(*k),
        Var => Some(var),
        Add(lhs, rhs) => Some(eval(lhs, var)? + eval(rhs, var)?),
        Sub(lhs, rhs) => Some(eval(lhs, var)? - eval(rhs, var)?),
        Mul(lhs, rhs) => Some(eval(lhs, var)? * eval(rhs, var)?),
        Div(lhs, rhs) => {
            let divisor = eval(rhs, var)?;
            if divisor == 0 {
                None // Division by zero
            } else {
                Some(eval(lhs, var)? / divisor)
            }
        },
        Summation(exprs) => {
            let mut acc = 0;
            for e in exprs {
                acc += eval(e, var)?;
            }
            Some(acc)
        }
    }
}

fn main() {
    let test = |expr| {
        let value = rand::random::<i8>() as i64;
        match eval(&expr, value) {
            Some(result) => println!("{:?} with Var = {} ==> {}", &expr, value, result),
            None => println!("{:?} with Var = {} ==> Division by zero error", &expr, value),
        }
    };

    test(Const(5));
    test(Var);
    test(sub(Var, Const(5)));
    test(mul(Var, Const(3)));
    test(div(Var, Const(0))); // To test division by zero
    test(add(sub(Var, Const(5)), Const(5)));
    test(Summation(vec![Var, Const(1), Const(2)]));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cases() {
        let x = 42;
        assert_eq!(eval(&Const(5), x), Some(5));
        assert_eq!(eval(&Var, x), Some(42));
        assert_eq!(eval(&sub(Var, Const(5)), x), Some(37));
        assert_eq!(eval(&mul(Var, Const(2)), x), Some(84));
        assert_eq!(eval(&div(Var, Const(2)), x), Some(21));
        assert_eq!(eval(&div(Var, Const(0)), x), None); // Division by zero
        assert_eq!(eval(&sub(Var, Var), x), Some(0));
        assert_eq!(eval(&add(sub(Var, Const(5)), Const(5)), x), Some(42));
        assert_eq!(eval(&Summation(vec![Var, Const(1)]), x), Some(43));
    }
}

// If you have time left and want to code more Rust: you can extend this exercise endlessly; one idea would be adding a Sigma(from,to,expr)
// constructor to Expr which computes the equivalent of (in LaTeX notation) \sum_{Var = from}^{to} expr; i.e. Sigma(Const(1), Const(5), Var) should be
// equivalent to Summation(vec![Const(1), Const(2), Const(3), Const(4), Const(5)]).
