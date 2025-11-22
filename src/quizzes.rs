use crate::tools::*;
pub fn add() -> bool {
    let x = get_rand(0, 9);
    let y = get_rand(0, 9);

    print!("{x} + {y} = ");
    let answer = get_number();
    x + y == answer
}

pub fn mult() -> bool {
    let x = get_rand(0, 9);
    let y = get_rand(0, 9);

    print!("{x} * {y} = ");
    let answer = get_number();
    x * y == answer
}

pub fn sub() -> bool {
    let x = get_rand(0, 9);
    let y = get_rand(0, 9);
    let sum = x + y;

    print!("{sum} - {y} = ");
    let answer = get_number();
    x == answer
}

pub fn int_div() -> bool {
    let x = get_rand(0, 9);
    let y = get_rand(1, 10);
    let prod = x * y;

    print!("{prod} / {y} = ");
    let answer = get_number();
    prod / y == answer
}

pub fn rem_div() -> bool {
    let q = get_rand(0, 9);
    let n = get_rand(1, 10);
    let r = get_rand(0, n - 1);
    let a = n * q + r;

    print!("{a} / {n} = ");
    let answer = get_number();
    if answer != q {
        return false;
    }

    print!("remainder = ");
    let answer = get_number();
    if answer != r {
        return false;
    }
    true
}
