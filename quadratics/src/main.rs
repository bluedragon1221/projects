pub mod util {
    pub fn checked_isqrt(num: i32) -> Option<i32> {
        if num == 0 || num == 1 {
            return Some(num);
        }

        let mut x = num / 2;
        let mut y = (x + num / x) / 2;

        while y < x {
            x = y;
            y = (x + num / x) / 2;
        }

        if x * x == num {
            Some(x)
        } else {
            None
        }
    }
}

use util::checked_isqrt;

struct Quadratic {
    a: i32,
    b: i32,
    c: i32,
}

fn get_factor_pairs(num: i32) -> Option<Vec<(i32, i32)>> {
    let range = if num > 0 {
        Some(1..checked_isqrt(num)? + 1)
    } else if num < 0 {
        Some(num..0)
    } else {
        None
    };

    range.map(|x| {
        x.filter(|y| num % y == 0)
            .map(|y| (y, num / y))
            .collect::<Vec<(i32, i32)>>()
    })
}

impl Quadratic {
    fn to_factors(&self) -> (i32, i32) {}
}
