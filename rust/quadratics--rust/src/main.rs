#![feature(isqrt)]

mod util {
    use std::cmp::Ordering;
    pub fn cmp_split<T>(x: i32, greater_fn: T, less_fn: T) -> Option<T> {
        match x.cmp(&0) {
            Ordering::Greater => Some(greater_fn),
            Ordering::Less => Some(less_fn),
            Ordering::Equal => None,
        }
    }
}

mod factor {
    use super::util::cmp_split;
    use list_comprehension_macro::comp;

    type Pair = (i32, i32);
    type Triple = (i32, i32, i32);

    fn get_factors(v: i32) -> Option<Vec<Pair>> {
        let range = cmp_split(v, 1..v.checked_isqrt()? + 1, v..0)?;
        Some(comp![(i, v / i) for i in range if v % i == 0])
    }

    pub fn select_factor(vars: Triple) -> Option<Pair> {
        let (a, b, c) = vars;

        get_factors(a * c)?
            .into_iter()
            .filter(|(x, y)| x + y == b)
            .nth(0)
    }

    pub fn fmt_pair(pair: Pair, variable: char) -> Option<String> {
        let sign = |x| Some(cmp_split(x, '+', '-')?);
        let factor = |x| Some(format!("({variable} {} {})", sign(x)?, x));

        Some(format!("{}{}", factor(pair.0)?, factor(pair.1)?))
    }
}

use factor::{fmt_pair, select_factor};

fn main() {
    let vars = (3, -17, -6);

    let pair = select_factor(vars).unwrap();

    println!("{}", fmt_pair(pair, 'x').unwrap())
}
