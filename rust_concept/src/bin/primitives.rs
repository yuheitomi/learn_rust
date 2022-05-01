use thiserror::Error;

fn main() -> Result<(), DivError> {
    println!("1st calc");
    print_repeat_mydiv(repeat_mydiv(&[(2, 1), (9, 3)]));

    println!("2nd calc");
    print_repeat_mydiv(repeat_mydiv(&[(2, 1), (-6, -3), (5, 2), (-6, 0)]));

    let x = 1;
    if x > 0 {
        println!("x > 0");
    }

    hash();

    Ok(())
}

#[derive(Error, Debug)]
enum DivError {
    #[error("{0} divided by zero")]
    DivByZero(i32),
    #[error("Both numerator {0} and denominator {1} are negative")]
    BothNegative(i32, i32),
}

fn mydiv(x: i32, y: i32) -> Result<i32, DivError> {
    if y == 0 {
        Err(DivError::DivByZero(y))
    } else if x < 0 && y < 0 {
        Err(DivError::BothNegative(x, y))
    } else {
        Ok(x / y)
    }
}

fn repeat_mydiv(ary: &[(i32, i32)]) -> Result<Vec<i32>, DivError> {
    let mut ret = Vec::new();
    for aa in ary {
        let ans = mydiv(aa.0, aa.1)?;
        ret.push(ans);
        println!("pushed: {} / {} = {}", aa.0, aa.1, ans);
    }
    Ok(ret)
}

fn print_repeat_mydiv(result: Result<Vec<i32>, DivError>) {
    match result {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{}", e),
    }
}

#[allow(dead_code)]
fn print_num(x: i32) {
    println!("input:{}", x);
    let _s = match x {
        0 => Some("zero"),
        1 | 2 => Some("small"),
        3..=9 => Some("large"),
        _ => None,
    };
}

use std::collections::HashMap;

fn hash() {
    let mut capitals = HashMap::new();
    capitals.insert("Japan", "Tokyo");
    capitals.insert("UK", "London");

    let targets = vec!["Japan", "US"];
    for tg in targets {
        match capitals.get(tg) {
            Some(e) => println!("{} > {}", tg, e),
            None => println!("Not found for {}", tg),
        }
    }
}
